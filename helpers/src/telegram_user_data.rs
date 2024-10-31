use percent_encoding::percent_decode_str;

#[derive(Debug)]
pub struct TgUserData {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub language_code: String,
    pub hash: String,
}

#[derive(Debug)]
struct KeyValue {
    key: String,
    value: String,
}

fn format_user_data_0(data: &str) -> KeyValue {
    let parts: Vec<&str> = data.splitn(2, ":").collect();
    if parts.len() < 2 {
        return KeyValue {
            key: String::from(""),
            value: String::from(""),
        };
    };
    let key = parts[0];
    let value = parts[1];
    let value = value.replace("\"", "");
    KeyValue {
        key: key.to_string(),
        value: value.to_string(),
    }
}

fn format_user_data_1(data: &str) -> KeyValue {
    let parts: Vec<&str> = data.splitn(2, "=").collect();
    if parts.len() < 2 {
        return KeyValue {
            key: String::from(""),
            value: String::from(""),
        };
    };
    let key = parts[0];
    let value = parts[1];
    let value = value.replace("\"", "");
    KeyValue {
        key: key.to_string(),
        value: value.to_string(),
    }
}

pub fn decode_tg_user_data(uri: String) ->Result<TgUserData, String> {
    let decoded_str = percent_decode_str(&uri).decode_utf8_lossy();
    let a = decoded_str.as_ref();
    let parts: Vec<&str> = a.splitn(2, '}').collect();
    if parts.len() < 2 {
        return Err(String::from("Invalid URI format"));
    }

    let part0 = parts[0];

    let b = part0.replace("{", "");
    let c: Vec<KeyValue> = b.split(",").map(format_user_data_0).collect();
    let mut user_data = TgUserData {
        id: 0,
        first_name: String::from(""),
        last_name: String::from(""),
        username: String::from(""),
        language_code: String::from(""),
        hash: String::from(""),
    };

    for item in c.iter() {
        match item
            .key
            .as_str()
            .replace("\\", "")
            .replace("\"", "")
            .to_string()
            .as_str()
        {
            "id" => user_data.id = item.value.parse().unwrap(),
            "first_name" => user_data.first_name = item.value.clone(),
            "last_name" => user_data.last_name = item.value.clone(),
            "username" => user_data.username = item.value.clone(),
            "language_code" => user_data.language_code = item.value.clone(),
            _ => (),
        }
    }
    let part1 = &parts[1][1..];
    let a: Vec<KeyValue> = part1.split("&").map(format_user_data_1).collect();
    for item in a.iter() {
        match item
            .key
            .as_str()
            .replace("\\", "")
            .replace("\"", "")
            .to_string()
            .as_str()
        {
            "hash" => user_data.hash = item.value.parse().unwrap(),
            _ => (),
        }
    }
    Ok(user_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let uri = String::from("%7B%22id%22%3A398015313%2C%22first_name%22%3A%22Alex%22%2C%22last_name%22%3A%22J%22%2C%22username%22%3A%22ElexShepard%22%2C%22language_code%22%3A%22ru%22%2C%22allows_write_to_pm%22%3Atrue%7D&chat_instance=8396920176790796100&chat_type=sender&auth_date=1730309229&hash=1a40892a83232c7e070fee6689e6bcc4f772c9ce28a0d52d4e8816bfd871a1c8");
        let user_data = decode_tg_user_data(uri).expect("Error");
        println!(">>> {:?}", user_data);
        assert!(user_data.first_name == "Alex");
    }
}
