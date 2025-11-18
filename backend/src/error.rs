use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error joining game")]
    JoinError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}
