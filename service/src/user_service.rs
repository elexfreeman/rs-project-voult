use chrono::prelude::Utc;
use sea_orm::ActiveValue;

use config::config_sys;
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
pub async fn get_user_data(auth:String) -> Result<TgUserData, ErrorSys> {

    let config = config_sys::get_config().await;
    let token = config.tg_config.token.clone();
    let is_dev = config.is_dev;

    if is_dev {
        let user_data = TgUserData{
            id: 1,
            first_name: String::from("test"),
            last_name: String::from("test"),
            username: String::from("test"),
            language_code: String::from("en"),
            hash: String::from(""),
        };
        add_user(&user_data).await?;
        return Ok(user_data);
    }

    let user_data = decode_tg_user_data(auth, token).map_err(|e| ErrorSys::auth_error(e))?;
    add_user(&user_data).await?;

    return Ok(user_data);
}
