use actix_web::{post, web, HttpResponse};

use crate::service::user_service;

use crate::modules::cache_log::cache_log_m::CacheLogM;
use crate::modules::cache_log::cache_log_r::CacheLogRouteR as R;
use crate::system::error_sys::ErrorSys;

#[post("/cache_log/add")]
pub async fn cache_log_add_route(
    body: Result<web::Json<R::Add::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log/add");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogM::add(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache_log/update")]
pub async fn cache_log_update_route(
    body: Result<web::Json<R::Update::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log/update");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogM::update(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache_log/list")]
pub async fn cache_log_list_route(
    body: Result<web::Json<R::List::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log/list");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogM::list(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache_log/get")]
pub async fn cache_log_get_route(
    body: Result<web::Json<R::Get::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log/get");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogM::get(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}
