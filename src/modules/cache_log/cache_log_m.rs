use actix_web::web;
use chrono::prelude::Utc;
use infrastructure::entity::cache_log;
use infrastructure::cache_log_sql::CacheLogSql;
use infrastructure::projects_sql::ProjectsSql;
use sea_orm::ActiveValue;
use system::error_sys::ErrorSys;

use crate::modules::cache_log::cache_log_r::CacheLogRouteR as R;

pub struct CacheLogM {}

impl CacheLogM {
    pub async fn add(
        request: web::Json<R::Add::Request>,
        owner_id: i32,
    ) -> Result<R::Add::Response, ErrorSys> {
        let project = ProjectsSql::one_project_by_id(request.project_id, owner_id).await?;
        let project_id = project.id;

        let new_cache_log = cache_log::ActiveModel {
            id: ActiveValue::default(),
            caption: ActiveValue::Set(request.caption.clone()),
            description: ActiveValue::Set(request.description.clone()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            project_id: ActiveValue::Set(project_id),
            contractor_id: ActiveValue::Set(request.contractor_id),
            is_delete: ActiveValue::default(),
        };
        let cache_log_id = CacheLogSql::add(new_cache_log).await?;

        let out = R::Add::Response { cache_log_id };
        return Ok(out);
    }

    pub async fn update(
        request: web::Json<R::Update::Request>,
        owner_id: i32,
    ) -> Result<R::Update::Response, ErrorSys> {
        let project = ProjectsSql::one_project_by_id(request.project_id, owner_id).await?;
        let project_id = project.id;
        let item = CacheLogSql::one_cache_log_by_id(request.id, project_id).await?;
        let mut pear: cache_log::ActiveModel = item.into();
        pear.caption = ActiveValue::Set(request.caption.clone());
        pear.description = ActiveValue::Set(request.description.clone());
        let cache_log_id = CacheLogSql::update(pear).await?.id;

        let out = R::Update::Response { cache_log_id };
        return Ok(out);
    }

    pub async fn list(
        request: web::Json<R::List::Request>,
        owner_id: i32,
    ) -> Result<R::List::Response, ErrorSys> {
        let project = ProjectsSql::one_project_by_id(request.project_id, owner_id).await?;
        let project_id = project.id;
        let list = CacheLogSql::list_cache_log_by_project_id(project_id).await?;
        let out = R::List::Response {
            list: list
                .iter()
                .map(|p| R::List::CacheLog {
                    id: p.id,
                    caption: p.caption.clone(),
                    description: p.description.clone(),
                    created_at: p.created_at.to_string(),
                    contractor_id: p.contractor_id,
                    project_id: p.project_id,
                })
                .collect(),
        };
        Ok(out)
    }

    pub async fn get(
        request: web::Json<R::Get::Request>,
        owner_id: i32,
    ) -> Result<R::Get::Response, ErrorSys> {
        let project = ProjectsSql::one_project_by_id(request.project_id, owner_id).await?;
        let project_id = project.id;
        let out = CacheLogSql::one_cache_log_by_id(request.id, project_id).await?;
        return Ok(R::Get::Response {
            id: out.id,
            caption: out.caption,
            description: out.description,
            created_at: out.created_at.to_string(),
            contractor_id: out.contractor_id,
            project_id: out.project_id,
        });
    }
}
