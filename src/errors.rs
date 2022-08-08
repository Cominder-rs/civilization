use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::Display;
use sea_orm::DbErr;
use validator::ValidationErrors;

#[derive(Display, Debug)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError(String),

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Incorrect payload: {}", _0)]
    ValidationError(String),

    #[display(fmt = "Internal Server Error")]
    DatabaseError(String),
}

impl From<ValidationErrors> for ServiceError {
    fn from(e: ValidationErrors) -> Self {
        ServiceError::ValidationError(e.to_string())
    }
}

impl From<DbErr> for ServiceError {
    fn from(e: DbErr) -> Self {
        ServiceError::DatabaseError(e.to_string())
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServiceError::InternalServerError(e) => {
                tracing::debug!(e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error. Please, try again later!".to_string(),
                )
            }
            ServiceError::BadRequest(e) => {
                tracing::debug!(e);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Bad request. Check your payload: {e}"),
                )
            }
            ServiceError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            ServiceError::ValidationError(e) => {
                tracing::debug!(e);
                (StatusCode::BAD_REQUEST, format!("Invalid payload! {e}"))
            }
            ServiceError::DatabaseError(e) => {
                tracing::debug!(e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error. Please, try again later!".to_string(),
                )
            }
        };

        (status, error_message).into_response()
    }
}
