use std::time::Duration;
use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use tokio::sync::OnceCell;

// Структура, которая будет синглтоном
pub struct DbConnectSys {
    pub db: DatabaseConnection,
}

impl DbConnectSys {
    async fn new() -> Self {
        let config = crate::config_sys::get_config().await;
        crate::config_sys::print_config(&config);
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}?currentSchema=public",
            config.pg_config.db_user,
            config.pg_config.db_password,
            config.pg_config.db_host,
            config.pg_config.db_port,
            config.pg_config.db_name
        );
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("public"); // Setting default PostgreSQL schema
        let db = Database::connect(opt).await.unwrap();
        DbConnectSys { db }
    }
}

static ONCE: OnceCell<DbConnectSys> = OnceCell::const_new();

pub async fn db_connect() -> &'static DbConnectSys {
    ONCE.get_or_init(|| async { 
        let db = DbConnectSys::new().await;
        db
    })
        .await
}
