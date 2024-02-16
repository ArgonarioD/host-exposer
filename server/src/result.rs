use sea_orm::DbErr;
use thiserror::Error;
use warp::reject::Reject;

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

impl Reject for HEError {}