use chrono::prelude::Utc;
use sea_orm::ActiveValue;
use actix_web::HttpRequest;

use config::config_sys;
use system::ctx_sys::CtxSys;
use infrastructure::entity::users as Users;
use infrastructure::users_sql::UsersSql;
use system::error_sys::ErrorSys;
use tglib::tg_user_data::{decode_tg_user_data, TgUserData};

pub async fn add_user(tg_user_data: &TgUserData) -> Result<i32, ErrorSys> {
    let user = UsersSql::get_by_telegram_id(tg_user_data.id.clone()).await?;

    if user.len() == 0 {
        let new_user = Users::ActiveModel {
            id: ActiveValue::Set(tg_user_data.id.clone()),
            user_id: ActiveValue::default(),
            first_name: ActiveValue::Set(tg_user_data.first_name.clone()),
            last_name: ActiveValue::Set(tg_user_data.last_name.clone()),
            username: ActiveValue::Set(tg_user_data.username.clone()),
            language_code: ActiveValue::Set(tg_user_data.language_code.clone()),
            created_at: ActiveValue::Set(Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(Utc::now().naive_utc()),
        };
        UsersSql::add(new_user).await?;
    }
    return Ok(1);
}

/**
 *  Прасит _auth header
 *  сверяет telegram token
 *  если новый пользователь заносит в db
 *  отдает данные о пользователе
 */
pub async fn get_user_data(req: HttpRequest) -> Result<TgUserData, ErrorSys> {
    let ctx = CtxSys::new(req);
    let auth_header = ctx.get_auth_header().await?;
    let config = config_sys::get_config().await;
    let token = config.tg_config.token.clone();

    let user_data = decode_tg_user_data(auth_header, token).map_err(|e| ErrorSys::auth_error(e))?;
    add_user(&user_data).await?;

    return Ok(user_data);
}
