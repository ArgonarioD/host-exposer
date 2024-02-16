use std::sync::Arc;
use clap::Parser;
use sea_orm::Database;
use uuid::Uuid;
use warp::Filter;
use clients::Clients;

use public_lib::public_lib::set_default_logger_level;
use crate::db::setup_schema;

mod result;
mod clients;
mod db;
mod entity;

#[derive(Parser, Debug)]
#[command(name = "Host Exposer Server")]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value = "3030")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_default_logger_level("info");
    pretty_env_logger::init_timed();
    let args = Args::parse();

    let db = Database::connect("sqlite://data.sqlite?mode=rwc").await?;
    setup_schema(&db).await;
    let db = Arc::new(db);
    let db_filter = warp::any().map(move || db.clone());

    let clients = Clients::default();
    let clients_filter = warp::any().map(move || clients.clone());

    let expose = warp::path("expose")
        .and(warp::ws())
        .and(clients_filter.clone())
        .and(db_filter.clone())
        .map(|ws: warp::ws::Ws, clients, db| {
            ws.on_upgrade(move |socket| clients::handle_connection(socket, clients, db))
        });

    let clients_information = warp::path("client")
        .and(warp::get())
        .and(clients_filter.clone())
        .and(db_filter.clone())
        .and_then(clients::get_clients_information);

    let modify_client_name = warp::path!("client" / Uuid)
        .and(warp::put())
        .and(db_filter.clone())
        .and(warp::body::content_length_limit(100))
        .and(warp::body::json())
        .and_then(clients::modify_client_name);

    let routes = expose
        .or(clients_information)
        .or(modify_client_name);

    warp::serve(routes).run(([0, 0, 0, 0], args.port)).await;
    Ok(())
}