use sea_orm::{ActiveModelTrait, ColumnTrait,Condition, EntityTrait, QueryFilter, QueryOrder};
use system::error_sys::ErrorSys;
use system::pg_connect_sys::db_connect;

use crate::entity::cache_log_items as CacheLogItems;

pub struct CacheLogItemsSql {}

impl CacheLogItemsSql {
    pub async fn list_by_cache_log_id(
        cache_log_id: i32,
    ) -> Result<Vec<CacheLogItems::Model>, ErrorSys> {
        let db_conn = db_connect().await;
        let cache_log = CacheLogItems::Entity::find()
            .filter(
                Condition::all()
                    .add(CacheLogItems::Column::CacheLogId.eq(cache_log_id))
                    .add(CacheLogItems::Column::IsDelete.eq(false)),
            )
            .filter(CacheLogItems::Column::CacheLogId.eq(cache_log_id))
            .order_by_asc(CacheLogItems::Column::UpdatedAt)
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()));
        return cache_log;
    }

    pub async fn one_by_id_and_cache_log_id(cache_log_items_id: i32, cache_log_id: i32) -> Result<CacheLogItems::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let items = CacheLogItems::Entity::find()
            .filter(
                Condition::all()
                    .add(CacheLogItems::Column::CacheLogId.eq(cache_log_id))
                    .add(CacheLogItems::Column::Id.eq(cache_log_items_id)),
            )
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()))?;
        if items.len() == 0 {
            return Err(ErrorSys::not_found_error(format!(
                "cache_log with id {} not found",
                cache_log_items_id
            )));
        }
        let out = Some(items[0].clone());
        return Ok(out.unwrap());
    }

    pub async fn add(data: CacheLogItems::ActiveModel) -> Result<i32, ErrorSys> {
        let db_conn = db_connect().await;
        let res = CacheLogItems::Entity::insert(data)
            .exec(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()))?;
        Ok(res.last_insert_id)
    }

    pub async fn add_many(data:Vec<CacheLogItems::ActiveModel>) -> Result<i32, ErrorSys> {
        let db_conn = db_connect().await;
        let res = CacheLogItems::Entity::insert_many(data)
            .exec(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()))?;
        Ok(res.last_insert_id)
    }

    pub async fn update(
        data: CacheLogItems::ActiveModel,
    ) -> Result<CacheLogItems::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let out = data
            .update(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()));
        out
    }
}
