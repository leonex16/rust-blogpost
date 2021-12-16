use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        const INTERNAL_SERVER_ERROR: &str = "Internal Server Error, Please try again later";
        const JWKS_FETCH_ERROR: &str = "Could not fetch JWKS";

        match self {
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json(INTERNAL_SERVER_ERROR)
            }
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json(JWKS_FETCH_ERROR)
            }
        }
    }
}
