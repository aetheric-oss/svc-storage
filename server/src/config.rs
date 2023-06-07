//! # Config
//!
//! Define and implement config options for module

use anyhow::Result;
use config::{ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// struct holding configuration options
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// deadpool configuration object
    pub pg: deadpool_postgres::Config,
    #[serde(default)]
    /// boolean using tls or not
    pub use_tls: bool,
    /// path to the db ca certificate used for psql db connections
    pub db_ca_cert: String,
    /// optional path to the client certificate used for psql db authentication
    pub db_client_cert: Option<String>,
    /// optional path to the client key used for psql db authentication
    pub db_client_key: Option<String>,
    /// port number to listen on for our gRPC server
    pub docker_port_grpc: u16,
    /// path to log configuration YAML file
    pub log_config: String,
}

impl Default for Config {
    fn default() -> Self {
        log::warn!("Creating Config object with default values.");
        Self::new()
    }
}

impl Config {
    /// Create new configuration object with default values
    pub fn new() -> Self {
        Config {
            docker_port_grpc: 50051,
            log_config: String::from("log4rs.yaml"),
            pg: deadpool_postgres::Config::new(),
            use_tls: true,
            db_ca_cert: "".to_string(),
            db_client_cert: None,
            db_client_key: None,
        }
    }

    /// Create a new `Config` object using environment variables
    pub fn try_from_env() -> Result<Self, ConfigError> {
        // read .env file if present
        dotenv().ok();

        config::Config::builder()
            .set_default("docker_port_grpc", 50051)?
            .set_default("log_config", String::from("log4rs.yaml"))?
            .set_default("use_tls", true)?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_default() {
        let config = Config::default();

        assert_eq!(config.docker_port_grpc, 50051);
        assert_eq!(config.log_config, String::from("log4rs.yaml"));
        assert_eq!(config.use_tls, true);
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("DOCKER_PORT_GRPC", "6789");
        std::env::set_var("LOG_CONFIG", "config_file.yaml");

        let config = Config::try_from_env().unwrap();

        assert_eq!(config.use_tls, true);
    }
}
