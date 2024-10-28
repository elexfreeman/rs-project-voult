use actix_web::{http, Error, HttpResponse};

// https://actix.rs/docs/errors/
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
};

use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
enum SysError {
    #[display("internal error")]
    InternalError,

    #[display("bad request")]
    BadClientData,

    #[display("timeout")]
    Timeout,
}

impl error::ResponseError for SysError {
    //log::error!("Error {:?}", self);
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            SysError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            SysError::BadClientData => StatusCode::BAD_REQUEST,
            SysError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

pub fn response_error(e: Error) -> HttpResponse {
    log::error!("Error {:?}", e);
    HttpResponse::Ok()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(e.to_string())
}
