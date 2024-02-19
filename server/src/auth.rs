use axum::extract::{Request, State};
use axum::http;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use rand::Rng;

use crate::AppState;

pub fn random_password(length: u8) -> String {
    const RANDOM_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ123456789!(),._-?@#[]`~=+*^%";
    let mut rng = rand::thread_rng();
    let password: String = std::iter::repeat(())
        .map(|()| RANDOM_CHARSET[rng.gen_range(0..RANDOM_CHARSET.len())] as char)
        .take(length as usize)
        .collect();
    password
}

pub async fn basic_auth(State(state): State<AppState>, req: Request, next: Next) -> Result<Response, StatusCode> {
    if let Some(authorization) = req.headers().get(http::header::AUTHORIZATION) {
        if let Ok(auth_token) = authorization.to_str() {
            if auth_token == format!("Basic {}", state.server_base64_password) {
                return Ok(next.run(req).await);
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}