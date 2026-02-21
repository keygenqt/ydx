use std::path::PathBuf;

use clap::Args;

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ToolsArgs {
    /// Path to the configuration file
    #[arg(short = 'c', long, value_name = "path")]
    pub config: PathBuf,

    /// Show this help message and exit
    #[clap(short='h', long, action = clap::ArgAction::HelpLong)]
    pub help: Option<bool>,
}

pub async fn run(_: ToolsArgs) {
    // @todo
}
