use actix_web::{web::Data, HttpRequest};
use std::sync::Mutex;
use std::sync::MutexGuard;

use crate::ctx_data_sys::CtxDataSys;
use crate::error_sys::ErrorSys;

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


    pub async fn get_auth_header(&self) -> Result<String, ErrorSys> {
        let user_data_headers = self.get_header("_auth");
        match user_data_headers {
            Some(user_data_str) => Ok(user_data_str),
            None => Err(ErrorSys::auth_error(String::from("No user data"))),
        }
    }
}
