use anyhow::{Ok, Result};

use crate::config::{config_model::{Database, DotEnvyConfig, Server}, stage::Stage};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        time_out: std::env::var("SERVER_TIME_OUT")
            .expect("SERVER_TIME_OUT is invalid")
            .parse()?,
        allow_origins: std::env::var("SERVER_ALLOW_ORIGINS").unwrap_or("*".to_string()),
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());

    Stage::try_from(&stage_str).unwrap_or_default()
}
