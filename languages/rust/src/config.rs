use crate::errors::ConfigParsingError;
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_PORT: u16 = 6969;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Config {
    /// Path to the configuration file
    #[arg(short, long, env)]
    pub config: Option<PathBuf>,

    // The content of the configuration file
    #[clap(skip)]
    pub file: ConfigFile,

    /// Enable debug logging
    #[arg(long, env, default_value_t = false)]
    pub debug: bool,

    /// The IP where to bind the server
    #[arg(short, long, env, default_value_t = DEFAULT_IP)]
    pub ip: IpAddr,

    /// The port where to bind the server
    #[arg(short, long, env, default_value_t = DEFAULT_PORT)]
    pub port: u16,
}


impl Config {
    pub fn parse_with_file() -> Result<Self, ConfigParsingError> {
        let mut output = Config::parse();
        output.file = output.config_file()?;
        output.validate()?;
        Ok(output)
    }

    fn config_file(&self) -> Result<ConfigFile, ConfigParsingError> {
        if let Some(file) = &self.config {
            Ok(serde_yaml::from_str(fs::read_to_string(file)?.as_str())?)
        } else {
            Ok(ConfigFile::default())
        }
    }

    fn validate(&self) -> Result<&Self, ConfigParsingError> {
        // Verify if the configuration parameters dependencies are coherent
        Ok(self)
    }
}

#[derive(Deserialize, Debug, Clone, Default)]
pub(crate) struct ConfigFile {
    // Add configuration attributes here
    // then, implement the "Default" trait
}
