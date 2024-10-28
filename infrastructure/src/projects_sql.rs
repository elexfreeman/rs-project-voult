use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use system::pg_connect_sys::db_connect;

use crate::entity::projects as Projects;

pub struct ProjectsSql {}

impl ProjectsSql {
    pub async fn get_project_by_owner_id(
        owner_id: i32,
    ) -> Result<Vec<Projects::Model>, sea_orm::DbErr> {
        let db_conn = db_connect().await;
        Projects::Entity::find_by_id(1).one(&db_conn.db).await?;
        let projects = Projects::Entity::find()
            .filter(Projects::Column::OwnerId.eq(owner_id))
            .all(&db_conn.db)
            .await?;
        Ok(projects)
    }

    pub async fn get_project_by_id(id: i32) -> Result<Option<Projects::Model>, sea_orm::DbErr> {
        let db_conn = db_connect().await;
        let out = Projects::Entity::find_by_id(id)
            .one(&db_conn.db)
            .await
            .expect("DB error");
        Ok(out)
    }

    pub async fn add_project(data: Projects::ActiveModel) -> Result<i32, sea_orm::DbErr> {
        let db_conn = db_connect().await;
        let res = Projects::Entity::insert(data).exec(&db_conn.db).await?;
        Ok(res.last_insert_id)
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::Utc;
    use sea_orm::ActiveValue;

    use super::*;

    #[tokio::test]
    async fn test_add_project_get_project() {
        let owner_id = 1;
        let new_project = Projects::ActiveModel {
            id: ActiveValue::default(),
            title: ActiveValue::Set(String::from("some title")),
            description: ActiveValue::Set(String::from("some title")),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            owner_id: ActiveValue::Set(owner_id),
        };

        let new_project_id = ProjectsSql::add_project(new_project).await.expect("erorr");
        assert!(new_project_id > 0);

        let project = ProjectsSql::get_project_by_id(new_project_id).await;
        assert!(project.is_ok());
        let project = project.unwrap();
        assert!(project.is_some());
        let project = project.unwrap();
        assert_eq!(project.owner_id, owner_id);
    }
}
