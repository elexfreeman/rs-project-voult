use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::system::ctx_sys::CtxSys;
use crate::service::user_service;

use crate::modules::project::project_m::ProjectM;
use crate::modules::project::project_r::ProjectRouteR as R;
use crate::system::error_sys::ErrorSys;

#[post("/project/add")]
pub async fn project_add_route(
    body: Result<web::Json<R::Add::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /project/add");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let ctx = CtxSys::new(req);
    let auth_header = ctx.get_auth_header().await?;
    let user_data = user_service::get_user_data(auth_header).await?;
    let response = ProjectM::add(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/project/list")]
pub async fn project_list_route(
    body: Result<web::Json<R::List::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /project/list");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let ctx = CtxSys::new(req);
    let auth_header = ctx.get_auth_header().await?;
    let user_data = user_service::get_user_data(auth_header).await?;
    let response = ProjectM::list(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/project/get")]
pub async fn project_get_route(
    body: Result<web::Json<R::Get::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /project/get");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let ctx = CtxSys::new(req);
    let auth_header = ctx.get_auth_header().await?;
    let user_data = user_service::get_user_data(auth_header).await?;
    let response = ProjectM::get(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}
