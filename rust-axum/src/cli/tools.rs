use clap::Args;
use colored::Colorize;
use std::{fs, path::PathBuf};

use crate::{
    app::app_config::AppConfig,
    db::{configure_db, models::user_model::UserModel, services::user_service::UserService},
    exit_error, print_error, print_success,
    utils::path::get_relative_path_canonicalize,
};

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ToolsArgs {
    /// Path to the configuration file
    #[arg(short = 'c', long, value_name = "path")]
    pub config: PathBuf,

    /// Import users from JSON
    #[arg(long, value_name = "path")]
    pub users: Option<PathBuf>,

    /// Show this help message and exit
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    pub help: Option<bool>,
}

#[allow(unused)]
pub async fn run(arg: ToolsArgs) {
    let config = match get_relative_path_canonicalize(&arg.config) {
        Some(path) => {
            let contents = fs::read_to_string(path.clone())
                .expect(&format!("not read file: {}", path.to_string_lossy()));
            match hocon_rs::Config::parse_str::<AppConfig>(&contents, None) {
                Ok(result) => result,
                Err(_) => exit_error!("error read config"),
            }
        }
        None => exit_error!("not found path to config"),
    };
    if let Some(users) = arg.users {
        match get_relative_path_canonicalize(&users) {
            Some(path) => {
                // Read json
                let data = fs::read_to_string(path.clone())
                    .expect(&format!("not read file: {}", path.to_string_lossy()));
                // Parse json
                let mut list: Vec<UserModel> = serde_json::from_str(&data).unwrap();
                // Create db instance
                let pool = configure_db(&config).await;
                // Loop users
                for user in &mut list {
                    // Gen random password for user
                    user.update_hash();
                    // Insert data
                    match UserService::insert(pool.get_owned().await.unwrap(), user).await {
                        Ok(_) => {
                            print_success!("{} user {}", "✓".green(), user.email.blue(),)
                        }
                        Err(err) => {
                            print_error!(
                                "{} user {}: {}",
                                "✗".red(),
                                user.email.blue(),
                                err.message
                            )
                        }
                    }
                }
            }
            None => exit_error!("not found path to config"),
        }
        return;
    }
}
