use serde::Deserialize;

/// Common configuration
#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub deployment: DeploymentConfig,
    pub postgres: PostgresConfig,
}

/// Server params
#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub development: bool,
    pub secret: String,
}

/// Server run params
#[derive(Deserialize, Clone)]
pub struct DeploymentConfig {
    pub host: String,
    pub port: i32,
}

/// Connect postgres params
#[derive(Deserialize, Clone)]
pub struct PostgresConfig {
    pub url: String,
    pub user: String,
    pub password: String,
}
