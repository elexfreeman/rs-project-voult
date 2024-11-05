use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder};
use system::error_sys::ErrorSys;
use system::pg_connect_sys::db_connect;

use crate::entity::contractors as Contractors;

pub struct ContractorsSql {}

impl ContractorsSql {
    pub async fn list_contractor_by_owner_id(
        owner_id: i32,
    ) -> Result<Vec<Contractors::Model>, ErrorSys> {
        let db_conn = db_connect().await;
        let contractors = Contractors::Entity::find()
            .filter(Contractors::Column::OwnerId.eq(owner_id))
            .order_by_asc(Contractors::Column::UpdatedAt)
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()));
        return contractors;
    }

    pub async fn one_contractor_by_id(
        contractor_id: i32,
        owner_id: i32,
    ) -> Result<Contractors::Model, ErrorSys> {
        let db_conn = db_connect().await;
        let contractors = Contractors::Entity::find()
            .filter(Contractors::Column::OwnerId.eq(owner_id))
            .filter(
                Condition::all()
                    .add(Contractors::Column::OwnerId.eq(owner_id))
                    .add(Contractors::Column::Id.eq(contractor_id)),
            )
            .all(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()))?;
        if contractors.len() == 0 {
            return Err(ErrorSys::not_found_error(format!(
                "Contractor with id {} not found",
                contractor_id
            )));
        }
        let out = Some(contractors[0].clone());
        return Ok(out.unwrap());
    }

    pub async fn add(data: Contractors::ActiveModel) -> Result<i32, ErrorSys> {
        let db_conn = db_connect().await;
        let res = Contractors::Entity::insert(data)
            .exec(&db_conn.db)
            .await
            .map_err(|e| ErrorSys::db_error(e.to_string()))?;
        Ok(res.last_insert_id)
    }

    pub async fn update(data: Contractors::ActiveModel) -> Result<Contractors::Model, ErrorSys> {
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
    async fn test_contractors_sql() {
        let owner_id = 1;
        let new_contractor = Contractors::ActiveModel {
            id: ActiveValue::default(),
            caption: ActiveValue::Set(String::from("some title")),
            description: ActiveValue::Set(String::from("some title")),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            owner_id: ActiveValue::Set(owner_id),
        };

        let new_contractor_id = ContractorsSql::add(new_contractor).await.expect("erorr");
        assert!(new_contractor_id > 0);

        let contractor = ContractorsSql::one_contractor_by_id(new_contractor_id, owner_id).await;
        assert!(contractor.is_ok());

        let contractor_list = ContractorsSql::list_contractor_by_owner_id(owner_id)
            .await
            .unwrap();
        assert!(contractor_list.len() > 0);
    }
}
