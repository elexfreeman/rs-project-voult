use actix_web::App;
use actix_web::{web::Data, HttpRequest};
use helpers::telegram_user_data::{decode_tg_user_data, TgUserData};
use std::sync::Mutex;
use std::sync::MutexGuard;

use crate::ctx_data_sys::CtxDataSys;
use crate::error::AppError;

pub struct CtxSys {
    pub req: HttpRequest,
}

impl<'a> CtxSys {
    pub fn new(req: HttpRequest) -> Self {
        CtxSys { req }
    }

    pub fn get_header(&self, name: &str) -> Option<String> {
        self.req
            .headers()
            .get(name)
            .map(|header| header.to_str().unwrap().to_string())
    }

    pub fn get_cookie(&self, name: &str) -> Option<String> {
        self.req
            .cookie(name)
            .map(|cookie| cookie.value().to_string())
    }

    pub fn get_sys_data(&self) -> MutexGuard<'_, CtxDataSys> {
        self.req
            .app_data::<Data<Mutex<CtxDataSys>>>()
            .unwrap()
            .lock()
            .unwrap()
    }

    pub fn get_user_data(&self) -> Result<TgUserData, AppError> {
        let user_data_headers = &self.get_header("_auth");
        match user_data_headers {
            Some(user_data_str) => {
                return decode_tg_user_data(String::from(user_data_str)).map_err(|e| AppError {
                    message: None,
                    cause: Some(e),
                    error_type: crate::error::AppErrorType::ValidationError,
                });
            }
            None => {
                Err( AppError {
                    message: None,
                    cause: Some(String::from("User data not found")),
                    error_type: crate::error::AppErrorType::ValidationError,
                })
            }
        }
    }
}
