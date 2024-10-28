use std::sync::MutexGuard;
use std::sync::Mutex;
use actix_web::{ web::Data, HttpRequest};

use crate::ctx_data_sys::CtxDataSys;

pub struct CtxSys{
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

    pub fn get_sys_data(&self) -> MutexGuard<'_, CtxDataSys>{
        self.req
            .app_data::<Data<Mutex<CtxDataSys>>>()
            .unwrap()
            .lock()
            .unwrap()
    }
}
