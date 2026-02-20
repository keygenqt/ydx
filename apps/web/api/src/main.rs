use clap::{CommandFactory, Parser};

use crate::{
    cli::{AppCLI, Commands},
    utils::constants,
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
        println!("Version: {}", constants::APP_VERSION);
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
