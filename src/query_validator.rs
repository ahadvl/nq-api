// TODO: Create Validator struct

use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use std::fmt::Display;

#[derive(Debug)]
pub enum QueryErrorKind {
    Length,
}

#[derive(Debug)]
pub struct QueryError {
    kind: QueryErrorKind,
    message: String,
}

impl QueryError {
    pub fn new(kind: QueryErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "query error: Kind: {:?} Message: {}",
            self.kind, self.message
        )
    }
}

impl error::ResponseError for QueryError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.message.clone())
    }

    fn status_code(&self) -> StatusCode {
        match self.kind {
            QueryErrorKind::Length => StatusCode::BAD_REQUEST,
        }
    }
}

pub trait QueryValidator {
    fn validate(&self) -> Result<(), QueryError>
    where
        Self: Sized;
}
