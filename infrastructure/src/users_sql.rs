use sea_orm::{ColumnTrait, DeleteResult, EntityTrait, QueryFilter};
use system::pg_connect_sys::db_connect;

use crate::entity::users as Users;

pub struct UsersSql {}

impl UsersSql {

    pub async fn get_by_telegram_id(owner_id: i32) -> Result<Vec<Users::Model>, sea_orm::DbErr> {
        let db_conn = db_connect().await;
        let list = Users::Entity::find()
            .filter(Users::Column::Id.eq(owner_id))
            .all(&db_conn.db)
            .await?;
        Ok(list)
    }
    pub async fn get_by_user_id(id: i32) -> Result<Option<Users::Model>, sea_orm::DbErr> {
        let db_conn = db_connect().await;
        let out = Users::Entity::find_by_id(id)
            .one(&db_conn.db)
            .await
            .expect("DB error");
        Ok(out)
    }

    pub async fn add(data: Users::ActiveModel) -> Result<i32, sea_orm::DbErr> {
        let db_conn = db_connect().await;
        let res = Users::Entity::insert(data).exec(&db_conn.db).await?;
        Ok(res.last_insert_id)
    }

    pub async fn delete(id: i32) -> Result<DeleteResult, sea_orm::DbErr> {
        let db_conn = db_connect().await;
        Users::Entity::delete_by_id(id).exec(&db_conn.db).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::prelude::Utc;
    use sea_orm::ActiveValue;

    use super::*;

    #[tokio::test]
    async fn test_users_sql() {
        let new_user = Users::ActiveModel {
            id: ActiveValue::Set(12),
            user_id: ActiveValue::default(),
            first_name: ActiveValue::Set(String::from("some title")),
            last_name: ActiveValue::Set(String::from("some title")),
            username: ActiveValue::Set(String::from("some title")),
            language_code: ActiveValue::Set(String::from("some title")),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
        };

        let new_id = UsersSql::add(new_user).await.expect("erorr");
        assert!(new_id > 0);

        let user = UsersSql::get_by_user_id(new_id).await;
        assert!(user.is_ok());
        let user = user.unwrap();
        assert!(user.is_some());

        let delete_result = UsersSql::delete(new_id).await;
        assert!(delete_result.is_ok());
        let delete_result = delete_result.unwrap();
        assert_eq!(delete_result.rows_affected, 1);
    }
}
