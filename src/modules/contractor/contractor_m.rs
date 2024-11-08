use actix_web::web;
use chrono::prelude::Utc;
use infrastructure::entity::contractors;
use infrastructure::contractors_sql::ContractorsSql;
use sea_orm::ActiveValue;
use system::error_sys::ErrorSys;

use crate::modules::contractor::contractor_r::ContractorRouteR as R;

pub struct ContractorM {}

impl ContractorM {
    pub async fn add(
        request: web::Json<R::Add::Request>,
        owner_id: i32,
    ) -> Result<R::Add::Response, ErrorSys> {
        let new_contractor = contractors::ActiveModel {
            id: ActiveValue::default(),
            caption: ActiveValue::Set(request.caption.clone()),
            description: ActiveValue::Set(request.description.clone()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            owner_id: ActiveValue::Set(owner_id),
            is_delete: ActiveValue::default(),
        };
        let contractor_id = ContractorsSql::add(new_contractor).await?;

        let out = R::Add::Response { contractor_id };
        return Ok(out);
    }

    pub async fn update(
        request: web::Json<R::Update::Request>,
        owner_id: i32,
    ) -> Result<R::Update::Response, ErrorSys> {
        let item = ContractorsSql::one_contractor_by_id(request.id, owner_id).await?;
        let mut pear: contractors::ActiveModel = item.into();
        pear.caption = ActiveValue::Set(request.caption.clone());
        pear.description = ActiveValue::Set(request.description.clone());
        let contractor_id = ContractorsSql::update(pear).await?.id;

        let out = R::Update::Response { contractor_id };
        return Ok(out);
    }

    pub async fn list(
        _request: web::Json<R::List::Request>,
        owner_id: i32,
    ) -> Result<R::List::Response, ErrorSys> {
        let list = ContractorsSql::list_contractor_by_owner_id(owner_id).await?;
        let out = R::List::Response {
            list: list
                .iter()
                .map(|p| R::List::Contractor {
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
        let out = ContractorsSql::one_contractor_by_id(request.id, owner_id).await?;
        return Ok(R::Get::Response {
            id: out.id,
            caption: out.caption,
            description: out.description,
            created_at: out.created_at.to_string(),
        });
    }
}
