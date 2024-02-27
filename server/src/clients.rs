use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::Json;
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::UtcOffset;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, error, info};
use uuid::Uuid;

use public_lib::message::{IpAddresses, MessagePack};

use crate::{AppState, db};
use crate::db::client::save_new_client_information;
use crate::entity::client;
use crate::entity::prelude::DbClient;
use crate::result::HEError;
use crate::times::local_offset_date_time;

#[derive(Serialize)]
pub struct Client {
    #[serde(default)]
    id: Uuid,
    #[serde(skip)]
    handler_tx: mpsc::UnboundedSender<Message>,
    #[serde(skip)]
    client_rx: UnboundedReceiverStream<Message>,
}

impl Client {
    async fn get_adapter_addresses(&mut self, db: &DatabaseConnection, default_offset: &UtcOffset) -> Result<Vec<IpAddresses>, HEError> {
        self.handler_tx.send(MessagePack::AddrRequest.to_framework_message())?;
        if let Some(message) = self.client_rx.next().await {
            match MessagePack::from_str(message.to_text().unwrap()) {
                Ok(MessagePack::AddrResponse { adapter_addresses }) => {
                    save_new_client_information(&self.id, db, default_offset).await?;
                    Ok(adapter_addresses)
                }
                Ok(_) => {
                    Err(HEError::Message("Unexpected message from client when requesting adapter addresses".to_string()))
                }
                Err(e) => {
                    Err(HEError::Message(format!("Error deserializing message: {}", e)))
                }
            }
        } else {
            Err(HEError::Message("Expected adapter addresses message, received nothing".to_string()))
        }
    }
}

pub type Clients = Arc<RwLock<HashMap<Uuid, Client>>>;

pub async fn handle_expose_websocket(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        handle_connection(socket, state.clients, state.db, state.server_base64_password, state.default_offset).await
    })
}

pub async fn handle_connection(ws: WebSocket, clients: Clients, db: DatabaseConnection, server_password: String, default_offset: UtcOffset) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    let (handler_tx, handler_rx) = mpsc::unbounded_channel();
    let mut handler_rx = UnboundedReceiverStream::new(handler_rx);

    let (client_tx, client_rx) = mpsc::unbounded_channel();
    let client_rx = UnboundedReceiverStream::new(client_rx);
    let client_id;
    if let Some(message) = ws_rx.next().await {
        let msg = message.unwrap();
        let text = msg.to_text().unwrap();
        debug!("Received message: {}", text);
        match MessagePack::from_str(text).unwrap() {
            MessagePack::Establish { id, password } => {
                if password != server_password {
                    ws_tx.send(MessagePack::Error { message: "Invalid password".to_string() }.to_framework_message()).await.unwrap();
                    return;
                }
                client_id = id;
                clients.write().await.insert(id, Client { id, handler_tx, client_rx });
                info!("Establishing connection with id: {}", &id);
                ws_tx.send(MessagePack::Acknowledge.to_framework_message()).await.unwrap();
                if let Err(e) = save_new_client_information(&id, &db, &default_offset).await {
                    error!("Failed to save new client information: {:?}", e);
                    return;
                }
            }
            pack => {
                error!("Unexpected message: {:?} from client when establishing connection, expected Establish message.", pack);
                return;
            }
        }
    } else {
        error!("Expected text established message, received something else");
        return;
    }

    tokio::spawn(async move {
        while let Some(message) = handler_rx.next().await {
            ws_tx.send(message).await.unwrap_or_else(|e| {
                error!("websocket send error: {}", e);
            });
        }
    });

    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("client {} websocket receive error: {}", &client_id, e);
                break;
            }
        };
        client_tx.send(msg).unwrap()
    }

    clients.write().await.remove(&client_id);
}

pub async fn get_clients_information(State(state): State<AppState>) -> Result<Json<Vec<Value>>, HEError> {
    let mut clients = state.clients.write().await;
    let mut clients_info: Vec<Value> = Vec::new();
    let all_client_ids: Vec<Uuid> = clients.keys()
        .copied()
        .collect();
    let db = &state.db;
    let default_offset = &state.default_offset;
    let target_clients: HashMap<Uuid, client::Model> = DbClient::find()
        .filter(client::Column::Id.is_in(all_client_ids))
        .all(db).await?
        .into_iter()
        .map(|mut client| {
            client.last_fetch_time = local_offset_date_time(default_offset);
            (client.id, client)
        })
        .collect();
    let mut updated_client_ids: Vec<Uuid> = Vec::new();
    for (id, client) in clients.iter_mut() {
        let client_adapter_addresses = client.get_adapter_addresses(db, default_offset).await.unwrap_or_else(|e| {
            error!("Failed to get adapter addresses: {:?}", e);
            vec![IpAddresses::empty(e.to_string())]
        });
        clients_info.push(json!({
            "entity": target_clients.get(id).unwrap(),
            "adapter_addresses": client_adapter_addresses
        }));
        updated_client_ids.push(*id);
    }
    db::client::update_clients_fetch_time(updated_client_ids.as_slice(), db, default_offset).await?;
    Ok(Json(clients_info))
}

#[derive(Deserialize)]
pub struct ModifyClientNameBody {
    new_name: String,
}

pub async fn modify_client_name(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<ModifyClientNameBody>,
) -> Result<(), HEError> {
    let db = &state.db;
    db::client::modify_client_name(&id, body.new_name, db).await?;
    Ok(())
}

