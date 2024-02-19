use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tokio::sync::mpsc::error::SendError;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HEError {
    #[allow(dead_code)]
    #[error("an io error occurred: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("an error occurred while performing message communication: {0}")]
    Message(String),
    #[error("an error occurred while performing database operations: {0:?}")]
    Db(#[from] DbErr)
}

impl <T> From<SendError<T>> for HEError {
    fn from(e: SendError<T>) -> Self {
        HEError::Message(format!("Error sending message: {}", e))
    }
}

impl IntoResponse for HEError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}