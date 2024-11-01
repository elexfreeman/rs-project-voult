use tokio::sync::OnceCell;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSys {
    pub pg_config: PgConfig,
    pub app_config: AppConfig,
}

static ONCE: OnceCell<ConfigSys> = OnceCell::const_new();

pub async fn get_config() -> &'static ConfigSys {
    ONCE.get_or_init(|| async {
        let config = crate::config_sys::load_config();
        config.unwrap()
    })
    .await
}


impl ConfigSys {
    pub fn get_pg_uri(&self) -> String {
        format!(
            "mongodb://{}:{}",
            self.pg_config.db_host, self.pg_config.db_port
        )
    }
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

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}

pub fn print_config(config: &ConfigSys) {
    println!("Config: {:?}", config);
}
