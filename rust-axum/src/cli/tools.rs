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
    #[arg(short = 'u', long, value_name = "path")]
    pub users: Option<PathBuf>,

    /// Show this help message and exit
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    pub help: Option<bool>,
}

pub async fn run(arg: ToolsArgs) {
    // load configuration
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
                // read json
                let data = fs::read_to_string(path.clone())
                    .expect(&format!("not read file: {}", path.to_string_lossy()));
                // parse json
                let mut list: Vec<UserModel> = serde_json::from_str(&data).unwrap();
                // create db instance
                let pool = configure_db(&config).await;
                // loop users
                for user in &mut list {
                    // check exist user
                    if let Ok(_) = UserService::find_by_email(
                        pool.get_owned().await.unwrap(),
                        user.email.clone(),
                    )
                    .await
                    {
                        print_error!("{} user {}: already exists", "✗".red(), user.email.blue());
                        continue;
                    };
                    // create hash password
                    user.update_hash();
                    // insert data
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
