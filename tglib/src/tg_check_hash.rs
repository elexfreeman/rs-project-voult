use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use url::Url;

/// Проверяет целостность `telegramInitData`, используя HMAC-SHA256.
/// - `telegram_init_data`: строка данных инициализации от Telegram
/// - `api_token`: токен API вашего бота
/// Возвращает `true`, если данные валидны, иначе `false`.
pub fn verify_init_data(telegram_init_data: &str, api_token: &str) -> bool {
    // Разбираем строку запроса в виде параметров
    let parsed_url = Url::parse(&format!("https://example.com/?{}", telegram_init_data))
        .expect("Failed to parse telegramInitData as URL");
    let mut params: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    // Извлекаем хэш и удаляем его из параметров
    let hash = match params.remove("hash") {
        Some(hash) => hash,
        None => return false, // Отсутствие хэша означает, что данные некорректны
    };

    // Сортируем ключи параметров по алфавиту и формируем `data_check_string`
    let mut sorted_keys: Vec<String> = params.keys().cloned().collect();
    sorted_keys.sort();
    let data_check_string = sorted_keys
        .iter()
        .map(|key| format!("{}={}", key, params[key]))
        .collect::<Vec<String>>()
        .join("\n");

    // Генерируем секретный ключ: HMAC-SHA256 от `api_token` с использованием "WebAppData"
    let mut mac = Hmac::<Sha256>::new_from_slice("WebAppData".as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(api_token.as_bytes());
    let secret_key = mac.finalize().into_bytes();

    // Вычисляем HMAC-SHA256 от `data_check_string` с использованием `secret_key`
    let mut mac =
        Hmac::<Sha256>::new_from_slice(&secret_key).expect("HMAC can take key of any size");
    mac.update(data_check_string.as_bytes());
    let calculated_hash = hex::encode(mac.finalize().into_bytes());

    // Сравниваем вычисленный хэш с переданным
    calculated_hash == hash
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        let query_string = "user=%7B%22id%22%3A308131758%2C%22first_name%22%3A%22ALEX%22%2C%22last_name%22%3A%22IVANNIKOV.PRO%22%2C%22username%22%3A%22ivannikovPro%22%2C%22language_code%22%3A%22ru%22%2C%22allows_write_to_pm%22%3Atrue%7D&chat_instance=-1857114464680496286&chat_type=private&auth_date=1716232213&hash=7d31991a605ab5e265b40ebbccc09c28bfb59366d2ac5cee9ca288c24a2ed3c3";
        let bot_token_test = "6887590696:AAEf6UL8lV5qhveK_dFNEOtjLTosCrErM8Q";

        if verify_init_data(&query_string, &bot_token_test) {
            println!("Data is valid and from Telegram.");
        } else {
            println!("Data verification failed.");
        }
        assert!(verify_init_data(&query_string, &bot_token_test));
    }

    #[test]
    fn it_works2() {
        let query_string = "user=%7B%22id%22%3A398015313%2C%22first_name%22%3A%22Alex%22%2C%22last_name%22%3A%22J%22%2C%22username%22%3A%22ElexShepard%22%2C%22language_code%22%3A%22ru%22%2C%22allows_write_to_pm%22%3Atrue%7D&chat_instance=8396920176790796100&chat_type=sender&auth_date=1730309229&hash=1a40892a83232c7e070fee6689e6bcc4f772c9ce28a0d52d4e8816bfd871a1c8";
        let bot_token_test = "7673642218:AAH-h1sgk3FJhWng0IqH01A2jWmILKSTJ9k";

        if verify_init_data(&query_string, &bot_token_test) {
            println!("Data is valid and from Telegram.");
        } else {
            println!("Data verification failed.");
        }
        assert!(verify_init_data(&query_string, &bot_token_test));
    }
}
