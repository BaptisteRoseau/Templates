mod config;
mod errors;
mod implementation;
mod logging;
mod models;
mod program;
mod routes;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = config::Config::parse_with_file()?;
    if let Err(error) = program::run(&config).await {
        eprintln!("{}", error);
        exit(1);
    }
    Ok(())
}
