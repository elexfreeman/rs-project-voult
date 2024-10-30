use actix_web::{web, Error};
use chrono::prelude::Utc;
use infrastructure::entity::users as Users;
use infrastructure::users_sql::UsersSql;
use sea_orm::ActiveValue;

use crate::modules::user::user_r::UserRouteR;

pub struct UserM {}

impl UserM {
    pub async fn add_user(
        request: web::Json<UserRouteR::Add::Request>,
    ) -> Result<UserRouteR::Add::Response, Error> {
        let mut out = UserRouteR::Add::Response { user_id: 0 };
        println!("request: {:?}", request);

        let user = UsersSql::get_by_telegram_id(request.id.clone())
            .await
            .expect("db read error");
        println!("user: {:?}", user);

        if user.len() > 0 {
            out.user_id = user[0].user_id;
        } else {
            let new_user = Users::ActiveModel {
                id: ActiveValue::Set(request.id.clone()),
                user_id: ActiveValue::default(),
                first_name: ActiveValue::Set(request.first_name.clone()),
                last_name: ActiveValue::Set(request.last_name.clone()),
                username: ActiveValue::Set(request.username.clone()),
                language_code: ActiveValue::Set(request.language_code.clone()),
                created_at: ActiveValue::Set(Utc::now().naive_utc()),
                updated_at: ActiveValue::Set(Utc::now().naive_utc()),
            };
            out.user_id = UsersSql::add(new_user).await.expect("db write error");
        }

        Ok(out)
    }
}

