use actix_web::{post, web, HttpResponse};

use crate::service::user_service;

use crate::modules::cache_log_items::cache_log_items_m::CacheLogItemsM;
use crate::modules::cache_log_items::cache_log_items_r::CacheLogItemsRouteR as R;
use crate::system::error_sys::ErrorSys;

#[post("/cache_log_items/add")]
pub async fn cache_log_items_add_route(
    body: Result<web::Json<R::Add::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log_items/add");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogItemsM::add(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache_log_items/add_many")]
pub async fn cache_log_items_add_many_route(
    body: Result<web::Json<R::AddMany::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log_items/add_many");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogItemsM::add_many(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache_log_items/update")]
pub async fn cache_log_items_update_route(
    body: Result<web::Json<R::Update::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log_items/update");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogItemsM::update(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}


#[post("/cache_log_items/upsert_many")]
pub async fn cache_log_items_upsert_many_route(
    body: Result<web::Json<R::Upsert::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log_items/upsert_many");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogItemsM::upsert_many(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache_log_items/list")]
pub async fn cache_log_items_list_route(
    body: Result<web::Json<R::List::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log_items/list");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogItemsM::list(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache_log_items/get")]
pub async fn cache_log_items_get_route(
    body: Result<web::Json<R::Get::Request>, actix_web::Error>,
) -> Result<HttpResponse, ErrorSys> {
    log::info!("Request from /cache_log_items/get");
    let body = body.map_err(|e| ErrorSys::json_error(e))?;
    let user_data = user_service::get_user_data(body.auth.clone()).await?;
    let response = CacheLogItemsM::get(body, user_data.id).await?;
    Ok(HttpResponse::Ok().json(response))
}
