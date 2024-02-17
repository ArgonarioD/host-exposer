use std::collections::HashMap;
use std::str::FromStr;

use clap::{arg, Parser};
use futures_util::{SinkExt, StreamExt};
use local_ip_address::list_afinet_netifas;
use log::{error, info};
use tokio::fs::OpenOptions;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::http::Uri;
use uuid::Uuid;

use public_lib::public_lib::{IpAddresses, MessagePack, set_default_logger_level};

#[derive(Parser, Debug)]
#[command(name = "Host Exposer Client")]
#[command(author, version, about)]
struct Args {
    /// Target server websocket URI
    #[arg(short, long, value_parser = parse_uri, value_name = "URI")]
    target_uri: Uri,
}

fn parse_uri(s: &str) -> Result<Uri, String> {
    Uri::from_str(s)
        .map_err(|e| e.to_string())
        .and_then(|uri| {
            if uri.scheme_str() == Some("ws") || uri.scheme_str() == Some("wss") {
                Ok(uri)
            } else {
                Err("URI must have a scheme of 'ws' or 'wss'".to_string())
            }
        })
}

async fn get_self_id() -> io::Result<Uuid> {
    let mut exposer_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(".exposer_id")
        .await?;
    let mut buffer = String::new();
    exposer_file.read_to_string(&mut buffer).await?;

    match Uuid::from_str(&buffer) {
        Ok(uuid) => { Ok(uuid) }
        Err(_) => {
            let new_uuid = Uuid::new_v4();
            exposer_file.set_len(0).await?;
            exposer_file.write_all(new_uuid.to_string().as_bytes()).await?;
            Ok(new_uuid)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_default_logger_level("info");
    pretty_env_logger::init_timed();
    let args = Args::parse();
    let (ws_stream, _) = connect_async(&args.target_uri).await?;
    info!("Establishing connection to server {}", &args.target_uri);
    let self_id = get_self_id().await?;
    info!("Self id: {}", &self_id);
    let (mut ws_tx, mut ws_rx) = ws_stream.split();
    ws_tx.send(
        MessagePack::Establish { id: self_id }.to_message()
    ).await?;
    if let Some(Ok(msg)) = ws_rx.next().await {
        let text = msg.to_text()?;
        match MessagePack::from_str(text)? {
            MessagePack::Acknowledge => {}
            pack => {
                panic!("Unexpected message: {:?} from server when establishing connection, expected Acknowledge message.", pack)
            }
        }
    }
    info!("connection to server {} established, self id: {}", &args.target_uri, &self_id);
    while let Some(result) = ws_rx.next().await {
        let message = result?;
        let text = message.to_text()?;
        info!("Received message: {}", text);
        match MessagePack::from_str(text) {
            Ok(MessagePack::AddrRequest) => {
                ws_tx.send(build_ip_addresses_response().to_message()).await
                    .unwrap_or_else(|e| {
                        error!("Failed to send message: {}", e)
                    });
            }
            Ok(MessagePack::Error { message }) => {
                error!("Received error message: {}", message);
            }
            Err(e) => {
                error!("Failed to parse message: {}", e);
            }
            _ => {
                error!("Unexpected message: {}", text);
            }
        }
    }

    Ok(())
}

fn build_ip_addresses_response() -> MessagePack {
    let network_interfaces = list_afinet_netifas().expect("Failed to list network interfaces");

    let mut ip_to_name_map: HashMap<String, IpAddresses> = HashMap::with_capacity(network_interfaces.len());

    for (name, ip) in network_interfaces.iter() {
        match ip_to_name_map.get_mut(name) {
            Some(addresses) => {
                addresses.append_address(*ip);
            }
            None => {
                let mut addresses = IpAddresses::empty(name.to_string());
                addresses.append_address(*ip);
                ip_to_name_map.insert(name.to_string(), addresses);
            }
        }
    }

    MessagePack::AddrResponse {
        adapter_addresses: ip_to_name_map.values()
            .cloned()
            .collect::<Vec<IpAddresses>>()
    }
}