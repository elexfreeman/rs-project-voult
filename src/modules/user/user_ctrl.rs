use actix_web::{post, web, HttpRequest, HttpResponse};

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

    async fn init(
        &self,
        request: web::Json<R::Add::Request>,
    ) -> Result<R::Add::Response, AppError> {
        self.check_user_data()?;
        UserM::add_user(request).await
    }
}

#[post("/user/init")]
pub async fn user_init_route(
    body: Result<web::Json<R::Add::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    log::info!("Request from /user/init");
    let body = body.map_err(|e| AppError::json_error(e))?;
    let ctx = CtxSys::new(req);
    let ctrl = UserCtrl::new(&ctx);
    let response = ctrl.init(body).await?;
    Ok(HttpResponse::Ok().json(response))
}
