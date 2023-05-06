use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use diesel::r2d2;
use log::error;


#[derive(Debug)]
pub struct TodoApiError {
    pub kind: TodoApiErrorKind,
}

#[derive(Debug)]
pub enum TodoApiErrorKind {
    InternalError,
    DieselError(String),
    R2D2Error(String),
}

impl From<&str> for TodoApiError {
    fn from(_: &str) -> Self {
        TodoApiError {
            kind: TodoApiErrorKind::InternalError,
        }
    }
}

impl fmt::Display for TodoApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TodoApiErrorKind::InternalError => write!(f, "Internal server error"),
            TodoApiErrorKind::DieselError(msg) => write!(f, "Diesel error: {}", msg),
            TodoApiErrorKind::R2D2Error(msg) => write!(f, "R2D2 error: {}", msg),
        }
    }
}

impl ResponseError for TodoApiError {
    fn error_response(&self) -> HttpResponse {
        match self.kind {
            TodoApiErrorKind::InternalError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            TodoApiErrorKind::DieselError(_) => {
                HttpResponse::InternalServerError().json("Diesel error")
            }
            TodoApiErrorKind::R2D2Error(_) => {
                HttpResponse::InternalServerError().json("R2D2 error")
            }
        }
    }
}

impl From<diesel::result::Error> for TodoApiError {
    fn from(error: diesel::result::Error) -> Self {
        error!("Diesel error: {}", error);
        TodoApiError {
            kind: TodoApiErrorKind::DieselError(error.to_string()),
        }
    }
}

impl From<r2d2::Error> for TodoApiError {
    fn from(error: r2d2::Error) -> Self {
        TodoApiError {
            kind: TodoApiErrorKind::R2D2Error(error.to_string()),
        }
    }
}

impl From<diesel::r2d2::PoolError> for TodoApiError {
    fn from(error: diesel::r2d2::PoolError) -> Self {
        TodoApiError {
            kind: TodoApiErrorKind::R2D2Error(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error as DieselError;

    #[test]
    fn test_diesel_error_conversion() {
        let diesel_error = DieselError::RollbackTransaction;
        let todo_api_error: TodoApiError = diesel_error.into();
        assert!(matches!(todo_api_error.kind, TodoApiErrorKind::DieselError(_)));
    }

    #[test]
    fn test_r2d2_error_conversion() {
        let r2d2_error = diesel::r2d2::Error::ConnectionError(diesel::ConnectionError::BadConnection("R2D2 error".to_string()));
        let todo_api_error: TodoApiError = r2d2_error.into();
        assert!(matches!(todo_api_error.kind, TodoApiErrorKind::R2D2Error(_)));
    }
}
