use actix_session::{SessionGetError, SessionInsertError};
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use juniper::{FieldError, IntoFieldError, Value};
use sqlx::Error as SqlxError;
use std::fmt;
use validator::ValidationErrors;

#[derive(Debug, Clone)]
pub enum AppErrorType {
    DbError,
    #[allow(dead_code)]
    NotFoundError,
    InvalidField,
    Authentification,
    Authorization,
    SessionError,
    ValidationError,
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn new(
        message: Option<String>,
        cause: Option<String>,
        error_type: AppErrorType,
    ) -> AppError {
        AppError {
            message,
            cause,
            error_type,
        }
    }
    pub fn message(&self) -> String {
        match self {
            AppError {
                message: Some(message),
                ..
            } => message.clone(),
            AppError {
                error_type: AppErrorType::NotFoundError,
                ..
            } => "The requested item was not found".to_string(),
            AppError {
                error_type: AppErrorType::InvalidField,
                ..
            } => "Invalid field value provided".to_string(),
            AppError {
                error_type: AppErrorType::Authentification,
                ..
            } => "Invalid log in data".to_string(),
            AppError {
                error_type: AppErrorType::Authorization,
                ..
            } => "Authorization failed. Plese authorizate first!".to_string(),
            AppError {
                error_type: AppErrorType::ValidationError,
                ..
            } => "Invalid data validation. Use correct data!".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl IntoFieldError for AppError {
    fn into_field_error(self) -> FieldError {
        FieldError::new(self.message(), Value::null())
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl From<SessionGetError> for AppError {
    fn from(_value: SessionGetError) -> AppError {
        AppError::new(
            Some("Error get session id".to_string()),
            None,
            AppErrorType::SessionError,
        )
    }
}

impl From<SessionInsertError> for AppError {
    fn from(_value: SessionInsertError) -> AppError {
        AppError::new(
            Some("Error insert session id".to_string()),
            None,
            AppErrorType::SessionError,
        )
    }
}

impl From<ValidationErrors> for AppError {
    fn from(_value: ValidationErrors) -> Self {
        AppError::new(None, None, AppErrorType::ValidationError)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message())
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::InvalidField => StatusCode::BAD_REQUEST,
            AppErrorType::Authentification => StatusCode::UNAUTHORIZED,
            AppErrorType::Authorization => StatusCode::FORBIDDEN,
            AppErrorType::SessionError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::ValidationError => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.message())
    }
}
