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
            .map_err(|e| ErrorSys::db_error(format!("CacheLogItemsSql.list_by_cache_log_id {}", e.to_string())));
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
            .map_err(|e| ErrorSys::db_error(format!("CacheLogItemsSql.one_by_id_and_cache_log_id {}", e.to_string())))?;
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
            .map_err(|e| ErrorSys::db_error(format!("CacheLogItemsSql.add {}", e.to_string())))?;
        Ok(res.last_insert_id)
    }

    pub async fn add_many(data:Vec<CacheLogItems::ActiveModel>) -> Result<i32, ErrorSys> {
        let db_conn = db_connect().await;
        let res = CacheLogItems::Entity::insert_many(data)
            .exec(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("CacheLogItemsSql.add_many {}", e.to_string())))?;
        Ok(res.last_insert_id)
    }

    pub async fn update(
        data: CacheLogItems::ActiveModel,
    ) -> Result<CacheLogItems::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let out = data
            .update(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("CacheLogItemsSql.update {}", e.to_string())));
        out
    }

    pub async fn update_many(data:Vec<CacheLogItems::ActiveModel>) -> Result<i32, ErrorSys> {
        let db_conn = db_connect().await;
        let res = CacheLogItems::Entity::insert_many(data)
            .exec(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("CacheLogItemsSql.update_many {}", e.to_string())))?;
        Ok(res.last_insert_id)
    }

}

#[cfg(test)]
mod tests {
    use chrono::prelude::Utc;
    use sea_orm::ActiveValue;

    use crate::entity::cache_log_items as CacheLogItems;

    use super::*;

    #[tokio::test]
    async fn test_update_sql() {
        let owner_id = 1;
        let new_item = CacheLogItems::ActiveModel {
            id: ActiveValue::default(),
            caption: ActiveValue::Set(String::from("some caption")),

            price: ActiveValue::Set(11.0),
            count: ActiveValue::Set(11),
            cache_log_id: ActiveValue::Set(owner_id),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            is_delete: ActiveValue::default(),
        };
        let id = CacheLogItemsSql::add(new_item).await.unwrap();
        assert!(id > 1);

        let item = CacheLogItemsSql::one_by_id_and_cache_log_id(id, owner_id).await;
        assert!(item.is_ok());

        let item_un = item.unwrap();

        let update_cache_log_item = CacheLogItems::ActiveModel {
            id: ActiveValue::Set(item_un.id),
            caption: ActiveValue::Set(item_un.caption.clone()),
            price: ActiveValue::Set(88.4),
            count: ActiveValue::Set(item_un.count),
            cache_log_id: ActiveValue::Set(owner_id),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            is_delete: ActiveValue::default(),
        };
                log::info!(">>>>> update_item={:?}", update_cache_log_item);

        let _ = CacheLogItemsSql::update(update_cache_log_item).await;   

    }
}
