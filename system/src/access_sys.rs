use crate::ctx_sys::CtxSys;

pub struct AccessSys<'a> {
    pub ctx_sys: &'a CtxSys,
}

impl<'a> AccessSys<'a> {
    pub fn new(ctx_sys: &'a CtxSys) -> Self {
        AccessSys { ctx_sys }
    }

    pub fn is_auth(&self) -> bool {
        true
    }

    pub fn is_admin(&self) -> bool {
        true
    }
}
