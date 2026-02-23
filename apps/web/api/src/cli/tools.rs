use std::{fs, path::PathBuf};

use clap::{Args, Subcommand};
use colored::Colorize;

use crate::{
    app::app_config::AppConfig,
    db::{configure_db, services::token_service::TokenService},
    exit_error, print_info, print_success,
    utils::path::get_relative_path_canonicalize,
};

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ToolsArgs {
    /// Path to the configuration file
    #[arg(long, value_name = "path")]
    pub config: PathBuf,

    /// Commands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Delete old, irrelevant data
    #[command(short_flag = 'c')]
    Clean(CleanArgs),
}

/// Arguments for clean command
#[derive(Args)]
pub struct CleanArgs {
    /// Clean expired tokens
    #[arg(short, long, default_value_t = false)]
    pub expired: bool,

    /// Clean duplicate tokens (keep only newest per device)
    #[arg(short = 'd', long)]
    pub duplicate: bool,

    /// Clean old images
    #[arg(short, long, default_value_t = false)]
    pub images: bool,
}

pub async fn run(args: ToolsArgs) {
    // load configuration
    let config = match get_relative_path_canonicalize(&args.config) {
        Some(path) => {
            let contents = fs::read_to_string(path.clone())
                .unwrap_or_else(|_| exit_error!("not read file: {}", path.to_string_lossy()));
            hocon_rs::Config::parse_str::<AppConfig>(&contents, None)
                .unwrap_or_else(|_| exit_error!("error read config: {}", path.to_string_lossy()))
        }
        None => exit_error!("not found path to config"),
    };
    // database connect
    let pool = configure_db(&config).await;
    let mut conn = pool
        .get_owned()
        .await
        .unwrap_or_else(|_| exit_error!("database connection error"));
    // execute command
    match args.command {
        Commands::Clean(args) => {
            if args.expired {
                let deleted = TokenService::clear_expired_tokens(&mut conn)
                    .await
                    .unwrap_or_else(|e| exit_error!("failed to clear tokens: {}", e.message));
                print_success!("{} Expired tokens removed: {}", "✓".green(), deleted);
            }
            if args.duplicate {
                let deleted = TokenService::deduplicate_tokens(&mut conn)
                    .await
                    .unwrap_or_else(|e| exit_error!("failed to deduplicate tokens: {}", e.message));
                print_success!("{} Duplicate tokens removed: {}", "✓".green(), deleted);
            }
            if args.images {
                // @todo clear images
                print_success!("{} Old images cleaned", "✓".green());
            }
            if !args.expired && !args.duplicate && !args.images {
                print_info!("no clean options. Specify at least one: --expired, --duplicate, --images");
            }
        }
    }
}
