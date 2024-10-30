use actix_web::{post, web, Error, HttpRequest, HttpResponse};

use crate::system::fa_action;
use crate::system::ctx_sys::CtxSys;
use crate::system::error_s::response_error;

use crate::modules::user::user_m::UserM;
use crate::modules::user::user_r::UserRouteR as R;

struct UserCtrl<'a> {
    ctx_sys: &'a CtxSys,
}

impl<'a> UserCtrl<'a> {
    fn new(ctx_sys: &'a CtxSys) -> Self {
        Self { ctx_sys }
    }

    async fn init(
        &self,
        request: web::Json<R::Add::Request>,
    ) -> Result<HttpResponse, Error> {
        log::info!("Request from /user/init");
        let user_header = self.ctx_sys.get_header("Content-Type");
        println!("Headers {:?}",Some(user_header));

        fa_action!(
            UserM::add_user(request),
            R::Add::Response,
            response_error
        )
        .await
    }

}

#[post("/user/init")]
pub async fn user_init_route(
    body: web::Json<R::Add::Request>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let ctx = CtxSys::new(req);
    let ctrl = UserCtrl::new(&ctx);
    ctrl.init(body).await
}