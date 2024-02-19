use axum::{middleware, Router};
use axum::routing::{get, put};
use axum_embed::{FallbackBehavior, ServeEmbed};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use clap::Parser;
use rust_embed::RustEmbed;
use sea_orm::DatabaseConnection;
use tracing::warn;
use tracing_subscriber::fmt::time::LocalTime;

use clients::Clients;
use public_lib::tracing::TracingLogLevel;

use crate::auth::basic_auth;
use crate::db::setup_db_connection;

mod result;
mod clients;
mod db;
mod entity;
mod auth;
mod migration;


#[derive(RustEmbed, Clone)]
#[folder = "frontend/dist/"]
struct AppWebPages;

#[derive(Parser, Debug)]
#[command(name = "Host Exposer Server")]
#[command(author, version, about)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value = "3030")]
    port: u16,
    /// Password for the server, if not specified, a random password of random-password-length will be generated
    #[arg(long, value_name = "PASSWORD")]
    pwd: Option<String>,
    /// Length of the random password to generate
    #[arg(short, long, default_value = "16")]
    random_password_length: u8,
    /// Maximum Log level
    #[arg(long, ignore_case = true, value_enum, default_value_t)]
    max_log_level: TracingLogLevel,
}

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
    clients: Clients,
    server_base64_password: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let timer = LocalTime::new(time::format_description::well_known::Iso8601::DATE_TIME_OFFSET);
    tracing_subscriber::fmt().with_timer(timer).with_max_level(args.max_log_level).init();

    let db = setup_db_connection().await?;

    let password = match args.pwd {
        Some(pwd) => pwd,
        None => {
            let result = auth::random_password(args.random_password_length);
            warn!("No password specified, generated random password: {}", result);
            result
        }
    };
    let base64_password = BASE64_STANDARD.encode(password);

    let state = AppState { db, clients: Clients::default(), server_base64_password: base64_password };

    let client_rest_router = Router::new()
        .route("/", get(clients::get_clients_information))
        .route("/auth", get(move || async move {}))
        .route("/:id", put(clients::modify_client_name))
        .route_layer(middleware::from_fn_with_state(state.clone(), basic_auth))
        .with_state(state.clone());

    let app = Router::new()
        .route("/expose", get(clients::handle_expose_websocket))
        .nest("/api/client", client_rest_router)
        .nest_service("/", ServeEmbed::<AppWebPages>::with_parameters(
            None,
            FallbackBehavior::NotFound,
            Some("index.html".to_owned()),
        ))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

