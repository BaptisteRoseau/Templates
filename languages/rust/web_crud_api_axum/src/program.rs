use crate::config::Config;
use crate::logging::init_logger;
use crate::routes::routes;
use tokio::net::TcpListener;
use log::info;

pub(crate) async fn run(config: &Config) -> Result<(), anyhow::Error> {
    init_logger(config.debug);
    let routes = routes();
    let listener = TcpListener::bind((config.ip, config.port)).await?;
    info!("Listening on {}:{}", config.ip, config.port);
    axum::serve(listener,routes).await?;
    Ok(())
}
