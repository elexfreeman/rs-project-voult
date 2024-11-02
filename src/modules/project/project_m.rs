use actix_web::web;
use chrono::prelude::Utc;
use infrastructure::entity::projects;
use infrastructure::projects_sql::ProjectsSql;
use sea_orm::ActiveValue;
use system::error_sys::ErrorSys;

use crate::modules::project::project_r::ProjectRouteR as R;

pub struct ProjectM {}

impl ProjectM {
    pub async fn add(
        request: web::Json<R::Add::Request>,
        owner_id: i32,
    ) -> Result<R::Add::Response, ErrorSys> {
        let new_project = projects::ActiveModel {
            id: ActiveValue::default(),
            caption: ActiveValue::Set(request.caption.clone()),
            description: ActiveValue::Set(request.description.clone()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            owner_id: ActiveValue::Set(owner_id),
        };
        let project_id = ProjectsSql::add(new_project).await?;

        let out = R::Add::Response { project_id };
        return Ok(out);
    }

    pub async fn list(
        _request: web::Json<R::List::Request>,
        owner_id: i32,
    ) -> Result<R::List::Response, ErrorSys> {
        let list = ProjectsSql::get_project_list_by_owner_id(owner_id).await?;
        let out = R::List::Response {
            list: list
                .iter()
                .map(|p| R::List::Project {
                    id: p.id,
                    caption: p.caption.clone(),
                    description: p.description.clone(),
                    created_at: p.created_at.to_string(),
                })
                .collect(),
        };
        Ok(out)
    }

    pub async fn get(
        request: web::Json<R::Get::Request>,
        owner_id: i32,
    ) -> Result<R::Get::Response, ErrorSys> {
        let out = ProjectsSql::get_project_by_id(request.id, owner_id).await?;
        return Ok(R::Get::Response {
            id: out.id,
            caption: out.caption,
            description: out.description,
            created_at: out.created_at.to_string(),
        });
    }
}
