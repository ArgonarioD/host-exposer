use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use time::OffsetDateTime;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{reject, Rejection, Reply};
use warp::ws::{Message, WebSocket};

use public_lib::public_lib::{IpAddresses, MessagePack};

use crate::db;
use crate::db::client::save_new_client_information;
use crate::entity::client;
use crate::entity::prelude::DbClient;
use crate::result::HEError;

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
    async fn get_adapter_addresses(&mut self, db: &DbConn) -> Result<HashMap<String, IpAddresses>, HEError> {
        self.handler_tx.send(MessagePack::AddrRequest.to_warp_message()).unwrap();
        if let Some(message) = self.client_rx.next().await {
            match MessagePack::from_str(message.to_str().unwrap()) {
                Ok(MessagePack::AddrResponse { adapter_addresses }) => {
                    save_new_client_information(&self.id, db).await?;
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

pub async fn handle_connection(ws: WebSocket, clients: Clients, db: Arc<DbConn>) {
    info!("Establishing connection");
    let (mut ws_tx, mut ws_rx) = ws.split();

    let (handler_tx, handler_rx) = mpsc::unbounded_channel();
    let mut handler_rx = UnboundedReceiverStream::new(handler_rx);

    let (client_tx, client_rx) = mpsc::unbounded_channel();
    let client_rx = UnboundedReceiverStream::new(client_rx);
    let client_id;
    if let Some(message) = ws_rx.next().await {
        let msg = message.unwrap();
        let text = msg.to_str().unwrap();
        info!("Received message: {}", text);
        match MessagePack::from_str(text).unwrap() {
            MessagePack::Establish { id } => {
                client_id = id;
                clients.write().await.insert(id, Client { id, handler_tx, client_rx });
                info!("Establishing connection with id: {}", &id);
                ws_tx.send(MessagePack::Acknowledge.to_warp_message()).await.unwrap();
                if let Err(e) = save_new_client_information(&id, db.as_ref()).await {
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

pub async fn get_clients_information(clients: Clients, db: Arc<DbConn>) -> Result<impl Reply, Rejection> {
    let mut clients = clients.write().await;
    let mut clients_info: Vec<Value> = Vec::new();
    let all_client_ids: Vec<Uuid> = clients.keys()
        .copied()
        .collect();
    let db = db.as_ref();
    let target_clients: HashMap<Uuid, client::Model> = DbClient::find()
        .filter(client::Column::Id.is_in(all_client_ids))
        .all(db).await.unwrap()
        .into_iter()
        .map(|mut client| {
            client.last_fetch_time = OffsetDateTime::now_local().unwrap();
            (client.id, client)
        })
        .collect();
    let mut updated_client_ids: Vec<Uuid> = Vec::new();
    for (id, client) in clients.iter_mut() {
        let client_adapter_addresses = client.get_adapter_addresses(db).await.unwrap_or_else(|e| {
            error!("Failed to get adapter addresses: {:?}", e);
            match e {
                HEError::Io(error) => { HashMap::from([(error.to_string(), IpAddresses::empty())]) }
                HEError::Message(msg) => { HashMap::from([(msg, IpAddresses::empty())]) }
                HEError::Db(error) => { HashMap::from([(error.to_string(), IpAddresses::empty())]) }
            }
        });
        clients_info.push(json!({
            "entity": target_clients.get(id).unwrap(),
            "adapter_addresses": client_adapter_addresses
        }));
        updated_client_ids.push(*id);
    }
    if let Err(e) = db::client::update_clients_fetch_time(updated_client_ids.as_slice(), db).await {
        return Err(reject::custom(e));
    }
    Ok(warp::reply::json(&clients_info))
}

#[derive(Deserialize)]
pub struct ModifyClientNameBody {
    new_name: String,
}

pub async fn modify_client_name(id: Uuid, db: Arc<DbConn>, body: ModifyClientNameBody) -> Result<impl Reply, Rejection> {
    let db = db.as_ref();
    if let Err(e) = db::client::modify_client_name(&id, body.new_name, db).await {
        return Err(reject::custom(e));
    }
    Ok(warp::reply::with_status("", warp::http::StatusCode::NO_CONTENT))
}

