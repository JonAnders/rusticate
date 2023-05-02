use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub struct TodoApiError {
    pub kind: TodoApiErrorKind,
}

#[derive(Debug)]
pub enum TodoApiErrorKind {
    InternalError,
    MutexLockError,
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
        match self.kind {
            TodoApiErrorKind::InternalError => write!(f, "Internal server error"),
            TodoApiErrorKind::MutexLockError => write!(f, "Failed to lock the mutex"),
        }
    }
}

impl ResponseError for TodoApiError {
    fn error_response(&self) -> HttpResponse {
        match self.kind {
            TodoApiErrorKind::InternalError => {
                HttpResponse::InternalServerError().json("Internal server error")
            }
            TodoApiErrorKind::MutexLockError => {
                HttpResponse::InternalServerError().json("Failed to lock the mutex")
            }
        }
    }
}
