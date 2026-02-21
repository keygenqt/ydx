use clap::ArgAction;
use clap::Parser;
use clap::Subcommand;
use clap::builder::Styles;

use crate::app::app_info::AppInfo;
use crate::cli::start::StartArgs;
use crate::cli::tools::ToolsArgs;

pub mod start;
pub mod tools;

#[derive(Parser)]
#[command(
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true
)]
#[command(about=AppInfo::default().description)]
#[command(styles=Styles::styled()
        .header(clap_cargo::style::HEADER)
        .usage(clap_cargo::style::USAGE)
        .literal(clap_cargo::style::LITERAL)
        .placeholder(clap_cargo::style::PLACEHOLDER)
        .error(clap_cargo::style::ERROR)
        .valid(clap_cargo::style::VALID)
        .invalid(clap_cargo::style::INVALID))]
pub struct AppCLI {
    /// Print version information and exit
    #[arg(short, long, default_value_t = false)]
    pub version: bool,

    /// Commands
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Show this help message and exit
    #[arg(short, long, action = ArgAction::HelpLong)]
    pub help: Option<bool>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start server API
    #[command(short_flag = 's')]
    Start(StartArgs),

    /// Tools for server
    #[command(short_flag = 't')]
    Tools(ToolsArgs),
}
