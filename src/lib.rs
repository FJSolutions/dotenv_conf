//!
mod command_line;
mod dotenv;
mod environment;

/// Defines the traits a config reader needs to implement.
pub trait ReadConfig {
    /// Read the config source data
    fn read_config(&mut self);
    /// Get a String value from the source data.
    fn get_value(&self, key: String) -> Option<String>;
}

#[derive(PartialEq, Debug)]
pub enum ConfigSource {
    DotEnv,
    Environment,
    CommandLine,
}

#[derive(PartialEq, Debug)]
pub struct ConfigReader {
    sources: Vec<ConfigSource>,
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
        // TODO Create Readers for each supplied source
        ConfigReader { sources }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        None
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
        assert_eq!(config.sources, ConfigReader::default().sources);
    }

    #[test]
    fn read_config_value() {
        let rdr = ConfigReader::default();

        let name = rdr.get("Name");
        assert!(name.is_some());
    }
}
