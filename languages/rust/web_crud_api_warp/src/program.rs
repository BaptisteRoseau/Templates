use crate::config::Config;
use crate::logging::init_logger;
use crate::models::Database;
use crate::routes::routes;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) async fn run(config: &Config) -> Result<(), anyhow::Error> {
    init_logger(config.debug);
    let database: Database = Arc::new(RwLock::new(HashMap::new()));
    let routes = routes(database);
    warp::serve(routes).run((config.ip, config.port)).await;
    Ok(())
}
