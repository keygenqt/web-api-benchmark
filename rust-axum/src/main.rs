use clap::{CommandFactory, Parser};

use crate::{
    app::app_info,
    cli::{AppCLI, Commands},
};

mod app;
mod cli;
mod db;
mod route;
mod utils;

/// Entry point to the application
#[tokio::main]
async fn main() {
    if AppCLI::parse().version {
        println!("Version: {}", app_info::version());
        return;
    }
    match AppCLI::parse().command {
        Some(command) => match command {
            Commands::Start(arg) => cli::start::run(arg).await,
            Commands::Tools(arg) => cli::tools::run(arg).await,
        },
        None => {
            let _ = AppCLI::command().print_help();
        }
    }
}
