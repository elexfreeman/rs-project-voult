use std::collections::HashMap;
use url::Url;
use crate::tg_check_hash;

#[derive(Debug)]
pub struct TgUserData {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub language_code: String,
    pub hash: String,
}

pub fn decode_tg_user_data(tgdata: String, tg_token: String ) -> Result<TgUserData, String> {
    if !tg_check_hash::verify_init_data(&tgdata, &tg_token) {
        println!("Invalid token: {}", tgdata.clone());
        return Err(format!("Invalid token: {}", tgdata.clone()));
    }
    // Разбираем строку запроса в виде параметров
    let parsed_tgdata = Url::parse(&format!("https://example.com/?{}", tgdata))
        .expect("Failed to parse telegramInitData as URL");
    let params: HashMap<String, String> = parsed_tgdata.query_pairs().into_owned().collect();

    let mut user_data = TgUserData {
        id: 0,
        first_name: String::from(""),
        last_name: String::from(""),
        username: String::from(""),
        language_code: String::from(""),
        hash: String::from(""),
    };
    let hash_o = params.get("hash");
    let hash = match hash_o {
        Some(hash) => hash.clone(),
        None => return Err(String::from("Hash not found")),
    };

    let user_o = params.get("user");
    let user = match user_o {
        Some(user) => user.clone(),
        None => return Err(String::from("User headers data not found")),
    };
    let user_json: serde_json::Value = serde_json::from_str(&user).expect("user parse error");

    user_data.id = user_json["id"].as_i64().unwrap() as i32;
    user_data.first_name = String::from(
        user_json["first_name"]
            .as_str()
            .expect("first_name parse error"),
    );
    user_data.last_name = String::from(
        user_json["last_name"]
            .as_str()
            .expect("last_name parse error"),
    );
    user_data.username = String::from(
        user_json["username"]
            .as_str()
            .expect("username parse errro"),
    );
    user_data.language_code = String::from(
        user_json["language_code"]
            .as_str()
            .expect("language_code parse error"),
    );
    user_data.hash = hash;


    Ok(user_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bot_token_test = "7673642218:AAH-h1sgk3FJhWng0IqH01A2jWmILKSTJ9k";
        let uri = String::from("user=%7B%22id%22%3A398015313%2C%22first_name%22%3A%22Alex%22%2C%22last_name%22%3A%22J%22%2C%22username%22%3A%22ElexShepard%22%2C%22language_code%22%3A%22ru%22%2C%22allows_write_to_pm%22%3Atrue%7D&chat_instance=8396920176790796100&chat_type=sender&auth_date=1730309229&hash=1a40892a83232c7e070fee6689e6bcc4f772c9ce28a0d52d4e8816bfd871a1c8");
        let user_data = decode_tg_user_data(uri, String::from(bot_token_test)).unwrap();
        println!(">>> {:?}", user_data);
        assert!(user_data.first_name == "Alex");
    }
}
