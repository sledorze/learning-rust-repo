use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Clone, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailed,

    // Auth
    AuthFailedNoAuthTokenCookie,
    AuthFailedTokenWrongFormat,
    AuthFailedCtxNotInRequestExt,

    // Models
    TicketDeleteFailedIdNotFound { id: u64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - IntoResponse - {self:?} - ", "HANDLER");

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Error::LoginFailed => (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAILED),
            Error::AuthFailedNoAuthTokenCookie
            | Error::AuthFailedTokenWrongFormat
            | Error::AuthFailedCtxNotInRequestExt => {
                (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH)
            }
            Error::TicketDeleteFailedIdNotFound { .. } => {
                (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMS)
            }
            // fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAILED,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
