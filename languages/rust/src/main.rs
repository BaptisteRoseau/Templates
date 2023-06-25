mod config;
mod errors;
mod logging;
mod program;
use std::process::exit;

fn main() -> Result<(), anyhow::Error> {
    let config = config::Config::parse_with_file()?;
    if let Err(error) = program::run(&config) {
        eprintln!("{}", error);
        exit(1);
    }
    Ok(())
}
