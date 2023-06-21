//!

use command_line::ArgsReader;
use dotenv::DotEnvReader;
use environment::EnvReader;
mod command_line;
mod dotenv;
mod environment;

/// Defines the traits a config reader needs to implement.
pub trait ReadConfig {
    /// Read the config source data
    fn read_config(&mut self);
    /// Get a String value from the source data.
    fn get_value(&self, key: impl Into<String>) -> Option<String>;
}

#[derive(PartialEq, Debug)]
pub enum ConfigSource {
    DotEnv,
    Environment,
    CommandLine,
}

pub struct ConfigReader {
    dot_env_rdr: Option<DotEnvReader>,
    env_rdr: Option<EnvReader>,
    args_rdr: Option<ArgsReader>,
}

impl Default for ConfigReader {
    fn default() -> Self {
        ConfigReader::new(vec![
            ConfigSource::DotEnv,
            ConfigSource::Environment,
            ConfigSource::CommandLine,
        ])
    }
}

impl ConfigReader {
    pub fn new(sources: Vec<ConfigSource>) -> Self {
        let dot_env_rdr = if sources.contains(&ConfigSource::Environment) {
            Some(DotEnvReader::new())
        } else {
            None
        };
        let env_rdr = if sources.contains(&ConfigSource::Environment) {
            Some(EnvReader::new())
        } else {
            None
        };
        let args_rdr = if sources.contains(&ConfigSource::CommandLine) {
            Some(ArgsReader::new())
        } else {
            None
        };

        ConfigReader {
            dot_env_rdr,
            env_rdr,
            args_rdr,
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let mut val = None;

        if let Some(rdr) = &self.dot_env_rdr {
            if let Some(v) = rdr.get_value(key) {
                val = Some(v);
            }
        };
        if let Some(rdr) = &self.env_rdr {
            if let Some(v) = rdr.get_value(key) {
                val = Some(v);
            }
        };
        if let Some(rdr) = &self.args_rdr {
            if let Some(v) = rdr.get_value(key) {
                val = Some(v);
            }
        };

        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_config_reader() {
        let config = ConfigReader::new(vec![
            ConfigSource::DotEnv,
            ConfigSource::Environment,
            ConfigSource::CommandLine,
        ]);

        assert!(config.args_rdr.is_some());
        assert!(config.env_rdr.is_some());
        assert!(config.dot_env_rdr.is_some());
    }

    #[test]
    fn read_config_value() {
        let rdr = ConfigReader::default();

        let name = rdr.get("NAME");

        assert!(name.is_some());
        assert_eq!(name, Some("Francis".to_owned()));
    }
}
