use crate::config::Config;
use crate::logging::init_logger;
use log::info;

pub(crate) fn run(config: &Config) -> Result<(), anyhow::Error> {
    init_logger(config.debug);
    info!("Hello World !");
    Ok(())
}
