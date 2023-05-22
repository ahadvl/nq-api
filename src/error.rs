use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RouterError {
    NotFound(String),

    /// Needs the detail of error
    ValidationError(String),
    InternalError,
    Gone(String),

    /// For example:
    /// username is not available
    NotAvailable(String),

    BadRequest(String),
}

impl Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(message) => write!(f, "{}", message),
            Self::ValidationError(detail) => write!(f, "{}", detail),
            Self::InternalError => write!(f, "internal server error"),
            Self::Gone(message) => write!(f, "{}", message),
            Self::NotAvailable(what) => write!(f, "{} is not available", what),
            Self::BadRequest(message) => write!(f, "{}", message),
        }
    }
}

impl Error for RouterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl ResponseError for RouterError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::Gone(_) => StatusCode::GONE,
            Self::NotAvailable(_) => StatusCode::OK,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }
}
