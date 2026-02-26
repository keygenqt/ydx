use axum::{ServiceExt, extract::Request};
use clap::Args;
use std::{fs, path::PathBuf};
use tower::ServiceBuilder;
use tower_http::normalize_path::NormalizePathLayer;

use crate::{
    app::{app_config::AppConfig, app_tracing::configure_tracing},
    exit_error, route,
    utils::path::get_relative_path_canonicalize,
};

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct StartArgs {
    /// Path to the configuration file
    #[arg(long, value_name = "path")]
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
            let route: String = format!("{}:{}", config.deployment.host, config.deployment.port);
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
            // get router
            let router = route::configure_route(&config).await;
            // route normalize: /foo/ is redirected to /foo
            let app = ServiceBuilder::new()
                .layer(NormalizePathLayer::trim_trailing_slash())
                .service(router);
            // service normalize
            let service = ServiceExt::<Request>::into_make_service(app);
            // start server
            axum::serve(listener, service).await.unwrap();
        }
        None => exit_error!("not found path to config"),
    }
}
