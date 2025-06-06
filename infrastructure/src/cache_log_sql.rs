use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use system::error_sys::ErrorSys;
use system::pg_connect_sys::db_connect;

use crate::entity::cache_log as CacheLog;

pub struct CacheLogSql {}

impl CacheLogSql {
    pub async fn list_cache_log_by_project_id(
        project_id: i32,
    ) -> Result<Vec<CacheLog::Model>, ErrorSys> {
        let db_conn = db_connect().await;
        let cache_log = CacheLog::Entity::find()
            .filter(CacheLog::Column::ProjectId.eq(project_id))
            .order_by_asc(CacheLog::Column::UpdatedAt)
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("list_cache_log_by_project_id.one_cache_log_by_id {}", e.to_string())));
        return cache_log;
    }

    pub async fn one_cache_log_by_id(
        cache_log_id: i32,
    ) -> Result<CacheLog::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let cache_log = CacheLog::Entity::find()
            .filter(CacheLog::Column::Id.eq(cache_log_id))
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("cache_log_sql.one_cache_log_by_id {}", e.to_string())))?;
        if cache_log.len() == 0 {
            return Err(ErrorSys::not_found_error(format!(
                "cache_log_sql.one_cache_log_by_id cache_log with id {} not found",
                cache_log_id
            )));
        }
        let out = Some(cache_log[0].clone());
        return Ok(out.unwrap());
    }

    pub async fn one_cache_log_by_id_and_project_id(
        cache_log_id: i32,
        project_id: i32,
    ) -> Result<CacheLog::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let cache_log = CacheLog::Entity::find()
            .filter(CacheLog::Column::ProjectId.eq(project_id))
            .filter(
                Condition::all()
                    .add(CacheLog::Column::ProjectId.eq(project_id))
                    .add(CacheLog::Column::Id.eq(cache_log_id)),
            )
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("cache_log_sql.one_cache_log_by_id_and_project_id {}", e.to_string())))?;
        if cache_log.len() == 0 {
            return Err(ErrorSys::not_found_error(format!(
                "cache_log with id {} not found",
                cache_log_id
            )));
        }
        let out = Some(cache_log[0].clone());
        return Ok(out.unwrap());
    }

    pub async fn add(data: CacheLog::ActiveModel) -> Result<i32, ErrorSys> {
        let db_conn = db_connect().await;
        let res = CacheLog::Entity::insert(data)
            .exec(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("cache_log_sql.add {}", e.to_string())))?;
        Ok(res.last_insert_id)
    }

    pub async fn update(data: CacheLog::ActiveModel) -> Result<CacheLog::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let out = data.update(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(format!("cache_log_sql.update {}", e.to_string())));
        out
    }
}