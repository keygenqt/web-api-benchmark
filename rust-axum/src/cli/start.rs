use clap::Args;
use std::{fs, path::PathBuf};

use crate::{
    app::{app_config::AppConfig, app_tracing::configure_tracing},
    exit_error, route,
    utils::path::get_relative_path_canonicalize,
};

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct StartArgs {
    /// Path to the configuration file
    #[arg(short = 'c', long, value_name = "path")]
    pub config: PathBuf,

    /// Show this help message and exit
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    pub help: Option<bool>,
}

#[allow(unused)]
pub async fn run(arg: StartArgs) {
    match get_relative_path_canonicalize(&arg.config) {
        Some(path) => {
            let contents = fs::read_to_string(path).unwrap();
            let config: AppConfig = match hocon_rs::Config::parse_str(&contents, None) {
                Ok(result) => result,
                Err(_) => exit_error!("error read config"),
            };
            // register tracing
            configure_tracing(&config);
            // init tcp listener
            let route = format!("{}:{}", config.deployment.host, config.deployment.port);
            let listener = tokio::net::TcpListener::bind(route).await.unwrap();
            // print start message
            if config.server.development {
                tracing::debug!(
                    "The server has started. Listening on http://{}",
                    listener.local_addr().unwrap()
                );
            } else {
                println!(
                    "The server has started. Listening on http://{}",
                    listener.local_addr().unwrap()
                )
            }
            // start server
            let _ = axum::serve(listener, route::configure_route(&config).await).await;
        }
        None => exit_error!("not found path to config"),
    }
}
