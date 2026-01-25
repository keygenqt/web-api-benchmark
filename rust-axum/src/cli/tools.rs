use std::{fs, path::PathBuf};

use clap::Args;
use colored::Colorize;

use crate::{app::app_config::AppConfig, db::{configure_db, entities::user_entity::UserNewModel, services::user_service::UserService}, exit_error, print_error, print_success, utils::path::get_relative_path_canonicalize};

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
            let contents = fs::read_to_string(path).unwrap();
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
                let data = fs::read_to_string(path).unwrap();
                // Parse json
                let mut list: Vec<UserNewModel> = serde_json::from_str(&data).unwrap();
                // Create db instance
                let pool = configure_db(&config).await;
                // Loop users
                for user in &mut list {
                    // Gen random password for user
                    let password = user.password_update(&config.server.salt);
                    // Insert data
                    match UserService::new(user.clone(), &pool).await {
                        Ok(user) => {
                            print_success!("{} user {}, password: {}", "✓".green(), user.email.blue(), password.cyan())
                        },
                        Err(err) => {
                            print_error!("{} user {}: {}", "✗".red(), user.email.blue(), err.message)
                        },
                    }
                }
            }
            None => exit_error!("not found path to config"),
        }
        return;
    }
}
