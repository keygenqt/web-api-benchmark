use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub deployment: DeploymentConfig,
    pub postgres: PostgresConfig,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub development: bool,
}

#[derive(Deserialize, Clone)]
pub struct DeploymentConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Clone)]
pub struct PostgresConfig {
    pub url: String,
    pub user: String,
    pub password: String,
}
