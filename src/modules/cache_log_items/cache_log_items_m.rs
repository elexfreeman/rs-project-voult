use actix_web::web;
use chrono::prelude::Utc;
use infrastructure::cache_log_items_sql::CacheLogItemsSql;
use infrastructure::cache_log_sql::CacheLogSql;
use infrastructure::entity::cache_log_items;
use infrastructure::projects_sql::ProjectsSql;
use sea_orm::ActiveValue;
use system::error_sys::ErrorSys;

use crate::modules::cache_log_items::cache_log_items_r::CacheLogItemsRouteR as R;

pub struct CacheLogItemsM {}

impl CacheLogItemsM {
    pub async fn add(
        request: web::Json<R::Add::Request>,
        owner_id: i32,
    ) -> Result<R::Add::Response, ErrorSys> {
        let cache_log = CacheLogSql::one_cache_log_by_id(request.cache_log_id).await?;
        let _project = ProjectsSql::one_project_by_id(cache_log.project_id, owner_id).await?;
        let cache_log_id = cache_log.id;

        let new_cache_log_items = cache_log_items::ActiveModel {
            id: ActiveValue::default(),
            caption: ActiveValue::Set(request.caption.clone()),
            price: ActiveValue::Set(request.price),
            count: ActiveValue::Set(request.count),
            cache_log_id: ActiveValue::Set(cache_log_id),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            is_delete: ActiveValue::default(),
        };
        let cache_log_items_id = CacheLogItemsSql::add(new_cache_log_items).await?;

        let out = R::Add::Response { cache_log_items_id };
        return Ok(out);
    }

    pub async fn update(
        request: web::Json<R::Update::Request>,
        owner_id: i32,
    ) -> Result<R::Update::Response, ErrorSys> {
        let cache_log = CacheLogSql::one_cache_log_by_id(request.cache_log_id).await?;
        let _project = ProjectsSql::one_project_by_id(cache_log.project_id, owner_id).await?;
        let cache_log_id = cache_log.id;

        let item = CacheLogItemsSql::one_by_id_and_cache_log_id(request.id, cache_log_id).await?;

        let mut pear: cache_log_items::ActiveModel = item.into();
        pear.caption = ActiveValue::Set(request.caption.clone());
        pear.price = ActiveValue::Set(request.price);
        pear.count = ActiveValue::Set(request.count);

        let cache_log_items_id = CacheLogItemsSql::update(pear).await?.id;

        let out = R::Update::Response { cache_log_items_id };
        return Ok(out);
    }

    pub async fn list(
        request: web::Json<R::List::Request>,
        owner_id: i32,
    ) -> Result<R::List::Response, ErrorSys> {
        let cache_log = CacheLogSql::one_cache_log_by_id(request.cache_log_id).await?;
        let _project = ProjectsSql::one_project_by_id(cache_log.project_id, owner_id).await?;
        let cache_log_id = cache_log.id;

        let list = CacheLogItemsSql::list_by_cache_log_id(cache_log_id).await?;
        let out = R::List::Response {
            list: list
                .iter()
                .map(|p| R::List::CacheLogItems {
                    id: p.id,
                    caption: p.caption.clone(),
                    price: p.price,
                    count: p.count,
                    created_at: p.created_at.to_string(),
                    updated_at: p.updated_at.to_string(),
                    cache_log_id: p.cache_log_id,

                })
                .collect(),
        };
        Ok(out)
    }

    pub async fn get(
        request: web::Json<R::Get::Request>,
        owner_id: i32,
    ) -> Result<R::Get::Response, ErrorSys> {
        let cache_log = CacheLogSql::one_cache_log_by_id(request.cache_log_id).await?;
        let _project = ProjectsSql::one_project_by_id(cache_log.project_id, owner_id).await?;
        let cache_log_id = cache_log.id;

        let out = CacheLogItemsSql::one_by_id_and_cache_log_id(request.id, cache_log_id).await?;
        return Ok(R::Get::Response {
            id: out.id,
            caption: out.caption,
            price: out.price,
            count: out.count,
            created_at: out.created_at.to_string(),
            updated_at: out.updated_at.to_string(),
            cache_log_id: out.cache_log_id,
        });
    }
}