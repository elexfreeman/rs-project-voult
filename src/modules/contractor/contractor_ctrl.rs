use actix_web::{post, web, HttpRequest, HttpResponse};

use crate::service::user_service;

use crate::modules::contractor::contractor_m::ContractorM;
use crate::modules::contractor::contractor_r::ContractorRouteR as R;
use crate::system::error_sys::ErrorSys;

#[post("/contractor/add")]
pub async fn contractor_add_route(
    body: Result<web::Json<R::Add::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /contractor/add");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(req).await?;
    let response = ContractorM::add(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/contractor/update")]
pub async fn contractor_update_route(
    body: Result<web::Json<R::Update::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /contractor/update");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(req).await?;
    let response = ContractorM::update(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/contractor/list")]
pub async fn contractor_list_route(
    body: Result<web::Json<R::List::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /contractor/list");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(req).await?;
    let response = ContractorM::list(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/contractor/get")]
pub async fn contractor_get_route(
    body: Result<web::Json<R::Get::Request>, actix_web::Error>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /contractor/get");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(req).await?;
    let response = ContractorM::get(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}
