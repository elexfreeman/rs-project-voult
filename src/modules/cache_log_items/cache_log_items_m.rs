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

    pub async fn add_many(
        request: web::Json<R::AddMany::Request>,
        owner_id: i32,
    ) -> Result<R::AddMany::Response, ErrorSys> {
        if request.items.len() == 0 {
            return Err(ErrorSys::not_found_error(String::from("items is empty")));
        }
        let cache_log_id = request.items[0].cache_log_id;

        let cache_log = CacheLogSql::one_cache_log_by_id(cache_log_id).await?;
        let _project = ProjectsSql::one_project_by_id(cache_log.project_id, owner_id).await?;
        let cache_log_id = cache_log.id;
        let items_vec = request
            .items
            .iter()
            .map(|item| {
                let new_cache_log_items = cache_log_items::ActiveModel {
                    id: ActiveValue::default(),
                    caption: ActiveValue::Set(item.caption.clone()),
                    price: ActiveValue::Set(item.price),
                    count: ActiveValue::Set(item.count),
                    cache_log_id: ActiveValue::Set(cache_log_id),
                    created_at: ActiveValue::Set(Utc::now().naive_utc()),
                    updated_at: ActiveValue::Set(Utc::now().naive_utc()),
                    is_delete: ActiveValue::default(),
                };
                new_cache_log_items
            })
            .collect();
        let cache_log_items_id = CacheLogItemsSql::add_many(items_vec).await?;

        let out = R::AddMany::Response { cache_log_items_id };
        return Ok(out);
    }

    pub async fn upsert_many(
        request: web::Json<R::Upsert::Request>,
        owner_id: i32,
    ) -> Result<R::Upsert::Response, ErrorSys> {
        if request.items.len() == 0 {
            return Err(ErrorSys::not_found_error(String::from("items is empty")));
        }
        let cache_log_id = request.items[0].cache_log_id;

        let cache_log = CacheLogSql::one_cache_log_by_id(cache_log_id).await?;
        let _project = ProjectsSql::one_project_by_id(cache_log.project_id, owner_id).await?;
        let cache_log_id = cache_log.id;
        let mut items_add: Vec<cache_log_items::ActiveModel> = Vec::new();
        let mut items_update: Vec<cache_log_items::ActiveModel> = Vec::new();
        for req_item in &request.items {
            if req_item.id > 0 {
                let db_item =
                    CacheLogItemsSql::one_by_id_and_cache_log_id(req_item.id, cache_log_id).await?;
                let mut pear: cache_log_items::ActiveModel = db_item.into();
                pear.caption = ActiveValue::Set(req_item.caption.clone());
                pear.price = ActiveValue::Set(req_item.price);
                pear.count = ActiveValue::Set(req_item.count);
                pear.updated_at = ActiveValue::Set(Utc::now().naive_utc());
                items_update.push(pear);

            } else {
                let new_cache_log_item = cache_log_items::ActiveModel {
                    id: ActiveValue::default(),
                    caption: ActiveValue::Set(req_item.caption.clone()),
                    price: ActiveValue::Set(req_item.price),
                    count: ActiveValue::Set(req_item.count),
                    cache_log_id: ActiveValue::Set(cache_log_id),
                    created_at: ActiveValue::Set(Utc::now().naive_utc()),
                    updated_at: ActiveValue::Set(Utc::now().naive_utc()),
                    is_delete: ActiveValue::default(),
                };
                items_add.push(new_cache_log_item);
            }
        }

        if items_add.len() > 0 {
            CacheLogItemsSql::add_many(items_add).await?;
        }
        if items_update.len() > 0 {
            for update_item in items_update {
                CacheLogItemsSql::update(update_item).await?;
            }
        }

        let out = R::Upsert::Response {};
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
