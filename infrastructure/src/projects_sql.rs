use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use system::error_sys::ErrorSys;
use system::pg_connect_sys::db_connect;

use crate::entity::projects as Projects;

pub struct ProjectsSql {}

impl ProjectsSql {
    pub async fn list_project_by_owner_id(
        owner_id: i32,
    ) -> Result<Vec<Projects::Model>, ErrorSys> {
        let db_conn = db_connect().await;
        let projects = Projects::Entity::find()
            .filter(Projects::Column::OwnerId.eq(owner_id))
            .order_by_asc(Projects::Column::UpdatedAt)
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()));
        return projects;
    }

    pub async fn one_project_by_id(
        project_id: i32,
        owner_id: i32,
    ) -> Result<Projects::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let projects = Projects::Entity::find()
            .filter(Projects::Column::OwnerId.eq(owner_id))
            .filter(
                Condition::all()
                    .add(Projects::Column::OwnerId.eq(owner_id))
                    .add(Projects::Column::Id.eq(project_id)),
            )
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()))?;
        if projects.len() == 0 {
            return Err(ErrorSys::not_found_error(format!(
                "Project with id {} not found",
                project_id
            )));
        }
        let out = Some(projects[0].clone());
        return Ok(out.unwrap());
    }

    pub async fn add(data: Projects::ActiveModel) -> Result<i32, ErrorSys> {
        let db_conn = db_connect().await;
        let res = Projects::Entity::insert(data)
            .exec(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()))?;
        Ok(res.last_insert_id)
    }

    pub async fn update(data: Projects::ActiveModel) -> Result<Projects::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let out = data.update(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()));
        out
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::Utc;
    use sea_orm::ActiveValue;

    use super::*;

    #[tokio::test]
    async fn test_projects_sql() {
        let owner_id = 1;
        let new_project = Projects::ActiveModel {
            id: ActiveValue::default(),
            caption: ActiveValue::Set(String::from("some title")),
            description: ActiveValue::Set(String::from("some title")),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            owner_id: ActiveValue::Set(owner_id),
            is_delete: ActiveValue::default(),
        };

        let new_project_id = ProjectsSql::add(new_project).await.expect("erorr");
        assert!(new_project_id > 0);

        let project = ProjectsSql::one_project_by_id(new_project_id, owner_id).await;
        assert!(project.is_ok());

        let project_list = ProjectsSql::list_project_by_owner_id(owner_id)
            .await
            .unwrap();
        assert!(project_list.len() > 0);
    }
}
