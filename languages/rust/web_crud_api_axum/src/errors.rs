use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigParsingError {
    #[error("Error while reading config file")]
    Disconnect(#[from] std::io::Error),

    #[error("Config has an invalid YAML format")]
    Parsing(#[from] serde_yaml::Error),
}
