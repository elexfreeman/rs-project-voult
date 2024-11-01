use core::fmt;

use actix_web::{error::ResponseError, error, http::StatusCode, HttpResponse};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    ValidationError,
    NotFoundError,
    AccessError,
}

// Struct type is already defined Option<String> and AppErrorType. We can also define later.
#[derive(Debug)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}
impl AppError {
    pub fn auth_error(e: String) -> Self {
        AppError {
            cause: Some(String::from("Auth error")),
            message: Some(e),
            error_type: AppErrorType::AccessError,
        }
    }

    pub fn json_error(e: error::Error) -> Self {
        AppError {
            cause: Some(String::from("JSON error")),
            message: Some(e.to_string()),
            error_type: AppErrorType::ValidationError,
        }
    }

    // we are handling the none. function name should match field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            AppError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            AppError {
                cause: Some(cause),
                message: None,
                error_type: AppErrorType::ValidationError,
            } => cause.clone(),
            _ => "An unexpected error has occured".to_string(),
        }
    }
    // This db_error is used when we haven't implmented the From trait

    // pub fn db_error(error: impl ToString) -> AppError {
    //     AppError {
    //         cause: Some(error.to_string()),
    //         message: None,
    //         error_type: AppErrorType::DbError,
    //     }
    // }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!(">>> +++ HERE 1 fmt::Display");
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    //error_response and status_code are the provided methods for ResponseError Trait

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => (StatusCode::INTERNAL_SERVER_ERROR),
            AppErrorType::NotFoundError => (StatusCode::NOT_FOUND),
            AppErrorType::ValidationError => (StatusCode::LENGTH_REQUIRED),
            AppErrorType::AccessError => (StatusCode::UNAUTHORIZED),
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::ValidationError,
        }
    }
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!(">>> +++ HERE fmt::Display");
        write!(f, "{:?}", self)
    }
}
impl ResponseError for AppErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

#[cfg(test)]
mod tests {

    use super::{AppError, AppErrorType};
    use actix_web::error::ResponseError;

    #[test]
    fn test_default_db_error() {
        let db_error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::DbError,
        };

        assert_eq!(
            db_error.message(),
            "An unexpected error has occured".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_not_found_error() {
        let db_error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::NotFoundError,
        };

        assert_eq!(
            db_error.message(),
            "The requested item was not found".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_user_db_error() {
        let user_message = "User-facing message".to_string();

        let db_error = AppError {
            message: Some(user_message.clone()),
            cause: None,
            error_type: AppErrorType::DbError,
        };

        assert_eq!(
            db_error.message(),
            user_message,
            "User-facing message should be shown"
        );
    }

    #[test]
    fn test_db_error_status_code() {
        let expected = 500;

        let db_error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::DbError,
        };

        assert_eq!(
            db_error.status_code(),
            expected,
            "Status code for DbError should be {}",
            expected
        );
    }

    #[test]
    fn test_validation_length_status_code() {
        let expected = 411;

        let db_error = AppError {
            message: None,
            cause: None,
            error_type: AppErrorType::ValidationError,
        };

        assert_eq!(
            db_error.status_code(),
            expected,
            "Status code for ValidationError should be {}",
            expected
        );
    }
}
