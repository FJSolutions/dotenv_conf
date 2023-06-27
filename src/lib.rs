//!

use command_line::ArgsReader;
use dotenv::DotEnvReader;
use environment::EnvReader;
mod command_line;
mod dotenv;
mod environment;

/// Defines the traits a config reader needs to implement.
pub trait ReadConf {
    /// Read the config source data
    fn read_config(&mut self);
    /// Get a String value from the source data.
    fn get_value(&self, key: impl Into<String>) -> Option<String>;
}

#[derive(Debug)]
pub struct ConfVal {
    key: String,
    cmd_line_arg: Option<String>,
    cmd_line_short: Option<String>,
}

impl ConfVal {
    pub fn new(key: impl Into<String>) -> Self {
        ConfVal {
            key: key.into(),
            cmd_line_arg: None,
            cmd_line_short: None,
        }
    }

    pub fn cmd_line_key(&mut self, name: impl Into<String>) -> &Self {
        self.cmd_line_arg = Some(name.into());
        self
    }

    pub fn cmd_line_shortkey(&mut self, name: impl Into<String>) -> &Self {
        self.cmd_line_short = Some(name.into());
        self
    }

    pub fn as_string_option(&self, rdr: &ConfReader) -> Option<String> {
        let mut value = None;

        if let Some(val) = rdr.get_value(self.key.as_str()) {
            value = Some(val);
            // dbg!("(Dot)Env: {:?}", &value);
        };

        if let Some(cmd_key) = &self.cmd_line_arg {
            if let Some(val) = rdr.get_value(cmd_key) {
                value = Some(val);
                // dbg!("Long CommandLine Arg:  {:?}", &value);
            }
        }

        if let Some(cmd_key) = &self.cmd_line_short {
            if let Some(val) = rdr.get_value(cmd_key) {
                value = Some(val);
                // dbg!("Short CommandLine Arg: {:?}", &value);
            }
        }

        value
    }

    pub fn as_string(&self, rdr: &ConfReader) -> Result<String, String> {
        let Some(val) = self.as_string_option(rdr) else {
            return Err(String::from("No key was supplied for the config value."));
        };

        Ok(val)
    }

    // pub fn as_u16_option(&self, rdr: &ConfReader) -> Option<u16> {
    //     // TODO How to differentiate  when the value won't parse and the key(s) aren't found?
    //     match self.as_string_option(rdr) {
    //         None => None,
    //         Some(s) => s.parse::<u16>().ok(),
    //     }
    // }

    // pub fn as_u16(self, _rdr: &ConfReader) -> Result<u16, String> {
    //     todo!()
    // }
}

#[derive(PartialEq, Debug)]
pub enum ConfSource {
    DotEnv,
    Environment,
    CommandLine,
}

pub struct ConfReader {
    dot_env_rdr: Option<DotEnvReader>,
    env_rdr: Option<EnvReader>,
    args_rdr: Option<ArgsReader>,
}

impl Default for ConfReader {
    fn default() -> Self {
        ConfReader::new(vec![
            ConfSource::DotEnv,
            ConfSource::Environment,
            ConfSource::CommandLine,
        ])
    }
}

impl ConfReader {
    pub fn new(sources: Vec<ConfSource>) -> Self {
        let dot_env_rdr = if sources.contains(&ConfSource::DotEnv) {
            // dbg!("New DotEnvRdr");
            Some(DotEnvReader::new())
        } else {
            None
        };
        let env_rdr = if sources.contains(&ConfSource::Environment) {
            // dbg!("New EnvRdr");
            Some(EnvReader::new())
        } else {
            None
        };
        let args_rdr = if sources.contains(&ConfSource::CommandLine) {
            // dbg!("New CommandLineRdr");
            Some(ArgsReader::new())
        } else {
            None
        };

        ConfReader {
            dot_env_rdr,
            env_rdr,
            args_rdr,
        }
    }

    fn get_value(&self, key: &str) -> Option<String> {
        let mut val = None;

        if let Some(rdr) = &self.dot_env_rdr {
            if let Some(v) = rdr.get_value(key) {
                val = Some(v);
                // dbg!("DotEnvReader reader get_value: {:?}", &val);
            }
        };
        if let Some(rdr) = &self.env_rdr {
            if let Some(v) = rdr.get_value(key) {
                val = Some(v);
                // dbg!("EnvReader reader get_value: {:?}", &val);
            }
        };
        if let Some(rdr) = &self.args_rdr {
            if let Some(v) = rdr.get_value(key) {
                val = Some(v);
                // dbg!("ArgsReader reader get_value: {:?}", &val);
            }
        };

        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, Debug)]
    pub struct TestConfig {
        pub db_con_str: String,
        pub port: u16,
        pub use_tls: bool,
        pub uid: Option<String>,
        pub pwd: Option<String>,
    }

    #[test]
    fn create_config_reader() {
        let config = ConfReader::new(vec![
            ConfSource::DotEnv,
            ConfSource::Environment,
            ConfSource::CommandLine,
        ]);

        assert!(config.args_rdr.is_some());
        assert!(config.env_rdr.is_some());
        assert!(config.dot_env_rdr.is_some());
    }

    #[test]
    fn read_config_value() {
        let rdr = ConfReader::default();

        let name = rdr.get_value("NAME");

        assert!(name.is_some());
        assert_eq!(name, Some("Francis".to_owned()));
    }

    #[test]
    fn read_config_option_value() {
        let rdr = ConfReader::default();

        let config = TestConfig {
            uid: ConfVal::new("NAME").as_string_option(&rdr),
            ..TestConfig::default()
        };

        assert!(config.uid.is_some());
        assert_eq!(config.uid, Some("Francis".to_owned()));
    }

    #[test]
    fn read_config_required_string_value() {
        let rdr = ConfReader::default();

        let config = TestConfig {
            db_con_str: ConfVal::new("DB").as_string(&rdr).unwrap(),
            ..TestConfig::default()
        };

        assert_eq!(config.db_con_str, "Postgres".to_owned());
    }
}
