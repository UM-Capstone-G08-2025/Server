use std::net::IpAddr;

use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct App {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
    pub api_key: String,
}

// #[derive(Deserialize)]
// pub struct Database {
//     /// Syntax: postgres://user:password@]host/database_name
//     pub url: String,
// }

#[derive(Deserialize)]
pub struct Config {
    pub app: App,
    pub auth: Auth,
    // pub database: Database,
}

impl Config {
    pub fn new() -> Result<Self, figment::Error> {
        Ok(Figment::new()
            .merge(Env::raw().split("__"))
            .extract::<Config>()?)
    }
}
