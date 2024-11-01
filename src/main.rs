use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use helpers;

use system;
use config::config_sys;

mod modules;
use modules::user::user_ctrl;

mod interfaces;

use crate::system::ctx_data_sys::CtxDataSys;
use system::pg_connect_sys::db_connect;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));
    let cfg = config::config_sys::get_config().await;
    config::config_sys::print_config(&cfg);
    let app_port = cfg.app_config.port;

    let user_data = web::Data::new(CtxDataSys {
        sample_string: "default_value".to_string(),
    });

    log::info!(
        "starting HTTP server at http://0.0.0.0:{}",
        app_port.to_string()
    );

    crate::config_sys::print_config(&cfg);
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        cfg.pg_config.db_user,
        cfg.pg_config.db_password,
        cfg.pg_config.db_host,
        cfg.pg_config.db_port,
        cfg.pg_config.db_name
    );
    println!("dobconnect {}", db_url);
    db_connect().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .expose_any_header()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(user_data.clone())
            .service(user_ctrl::user_init_route)
    })
    .workers(4)
    .bind(format!("0.0.0.0:{}", app_port))?
    .run()
    .await
}
