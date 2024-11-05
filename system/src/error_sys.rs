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
pub struct ErrorSys {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}
impl ErrorSys {
    pub fn db_error(e: String) -> Self {
        ErrorSys {
            cause: Some(String::from("DB error")),
            message: Some(e),
            error_type: AppErrorType::DbError,
        }
    }

    pub fn auth_error(e: String) -> Self {
        ErrorSys {
            cause: Some(String::from("Auth error")),
            message: Some(e),
            error_type: AppErrorType::AccessError,
        }
    }

    pub fn not_found_error(e: String) -> Self {
        ErrorSys {
            cause: Some(String::from("Not found error")),
            message: Some(e),
            error_type: AppErrorType::NotFoundError,
        }
    }

    pub fn json_error(e: error::Error) -> Self {
        ErrorSys {
            cause: Some(String::from("JSON error")),
            message: Some(e.to_string()),
            error_type: AppErrorType::ValidationError,
        }
    }

    // we are handling the none. function name should match field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            ErrorSys {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            ErrorSys {
                cause: _,
                message: None,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            ErrorSys {
                cause: Some(cause),
                message: None,
                error_type: AppErrorType::ValidationError,
            } => cause.clone(),
            _ => "An unexpected error has occured".to_string(),
        }
    }
}
impl fmt::Display for ErrorSys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrorSys {
    //error_response and status_code are the provided methods for ResponseError Trait

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::ValidationError => StatusCode::LENGTH_REQUIRED,
            AppErrorType::AccessError => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        println!("Error: {}", self);
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl From<ValidationErrors> for ErrorSys {
    fn from(error: ValidationErrors) -> ErrorSys {
        ErrorSys {
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

    use super::{ErrorSys, AppErrorType};
    use actix_web::error::ResponseError;

    #[test]
    fn test_default_db_error() {
        let db_error = ErrorSys {
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
        let db_error = ErrorSys {
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

        let db_error = ErrorSys {
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

        let db_error = ErrorSys {
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

        let db_error = ErrorSys {
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
