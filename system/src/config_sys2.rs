use once_cell::sync::Lazy;
use std::sync::Mutex;

use std::env;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub port: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PgConfig {
    pub db_host: String,
    pub db_port: i32,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
}

#[derive(Debug, Serialize, Deserialize ,Clone)]
pub struct ConfigSys {
    pub pg_config: PgConfig,
    pub app_config: AppConfig,
}

// Реализация статической переменной с ленивой инициализацией
static INSTANCE: Lazy<Mutex<ConfigSys>> = Lazy::new(|| {
    let config = crate::config_sys::load_config();
    match config {
        Ok(config) => Mutex::new(config),
        Err(e) => {
            println!("Error loading config: {:?}", e);
            panic!("Error loading config");
        }   
    }
});

impl ConfigSys {
    pub fn get_pg_uri(&self) -> String {
        format!(
            "mongodb://{}:{}",
            self.pg_config.db_host, self.pg_config.db_port
        )
    }

    // Метод для доступа к экземпляру синглтона
    pub fn get_instance() -> std::sync::MutexGuard<'static, ConfigSys> {
        INSTANCE.lock().unwrap()
    }

    // Пример метода для изменения состояния синглтона
//    pub fn set_value(&mut self, value: i32) {
//        self.value = value;
//    }
}

pub fn get_config_file_name(args: &[String]) -> Option<String> {
    if let Some(index) = args.iter().position(|arg| arg == "--config") {
        if index + 1 < args.len() {
            return Some(args[index + 1].clone());
        }
    }
    None
}

pub fn load_config() -> Result<ConfigSys, std::io::Error> {
    let mut config_file_name = String::from("config.json");
    let config_file_path = String::from("");

    let args: Vec<String> = env::args().collect();
    let config_file_from_args = get_config_file_name(&args);

    if config_file_from_args.is_some() {
        config_file_name = config_file_from_args.unwrap();
    }
    let file_path = config_file_path + &config_file_name;
    println!("Use config: {:?}", file_path);

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

pub fn print_config(config: &ConfigSys) {
    println!("Config: {:?}", config);
}