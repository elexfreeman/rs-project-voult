use actix_web::{post, web, HttpRequest, HttpResponse, Responder};

use crate::system::ctx_sys::CtxSys;

use crate::modules::user::user_m::UserM;
use crate::modules::user::user_r::UserRouteR as R;
use crate::system::error::AppError;

struct UserCtrl<'a> {
    ctx_sys: &'a CtxSys,
}

impl<'a> UserCtrl<'a> {
    fn new(ctx_sys: &'a CtxSys) -> Self {
        Self { ctx_sys }
    }

    fn check_user_data(&self) -> Result<(), AppError> {
        let tg_user_data = self.ctx_sys.get_user_data();
        match tg_user_data {
            Ok(_user_data) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn init(&self, request: web::Json<R::Add::Request>) -> Result<R::Add::Response, AppError> {
        log::info!("Request from /user/init");
        let auth = self.check_user_data();
        match auth {
            Ok(_) => UserM::add_user(request).await,
            Err(e) => Err(e),
        }
    }
}

#[post("/user/init")]
pub async fn user_init_route(
    body: web::Json<R::Add::Request>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let ctx = CtxSys::new(req);
    let ctrl = UserCtrl::new(&ctx);
    let response: Result<R::Add::Response, AppError> = ctrl.init(body).await;
    response
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(|e| e)
}

pub async fn user_test(
    body: web::Json<R::Add::Request>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let ctx = CtxSys::new(req);
    let ctrl = UserCtrl::new(&ctx);
    let response: Result<R::Add::Response, AppError> = ctrl.init(body).await;
    response
        .map(|data| HttpResponse::Ok().json(data))
        .map_err(|e| e)
}
