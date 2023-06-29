//!

use command_line::ArgsReader;
use dotenv::DotEnvReader;
use environment::EnvReader;
mod command_line;
mod dotenv;
mod environment;

/// Defines the traits a config reader needs to implement.
trait ReadConf {
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
        self.as_string(rdr).ok()
    }

    pub fn as_string(&self, rdr: &ConfReader) -> Result<String, String> {
        let mut value = None;

        // Check if there was a short command line argument supplied for this value
        if let Some(val) = rdr.get_value(&self.key) {
            value = Some(val);
            // dbg!("(Dot)Env: {:?}", &value);
        };

        // Check if there was a command line argument (--arg) supplied for this value
        if let Some(cmd_key) = &self.cmd_line_arg {
            if let Some(val) = rdr.get_value(cmd_key) {
                value = Some(val);
                // dbg!("Long CommandLine Arg:  {:?}", &value);
            }
        }

        // Check if there was a short command line argument (-a) supplied for this value
        if let Some(cmd_key) = &self.cmd_line_short {
            if let Some(val) = rdr.get_value(cmd_key) {
                value = Some(val);
                // dbg!("Short CommandLine Arg: {:?}", &value);
            }
        }

        // No value found
        let Some(value) = value else {
            return Err(format!("No value was found for the key '{}'", &self.key).to_string());
        };

        // Empty value found
        if value.is_empty() {
            return Err(format!("An empty string was found for the key '{}'", &self.key).to_string());
        };

        Ok(value)
    }

    pub fn as_bool_option(&self, rdr: &ConfReader) -> Option<bool> {
        self.as_bool(rdr).ok()
    }

    pub fn as_bool(&self, rdr: &ConfReader) -> Result<bool, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.to_lowercase().parse::<bool>() else {
            return Err(format!("'{}' could not be parsed as a `bool` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_u8_option(&self, rdr: &ConfReader) -> Option<u8> {
        self.as_u8(rdr).ok()
    }

    pub fn as_u8(&self, rdr: &ConfReader) -> Result<u8, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<u8>() else {
            return Err(format!("'{}' could not be parsed as a `u8` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_u16_option(&self, rdr: &ConfReader) -> Option<u16> {
        self.as_u16(rdr).ok()
    }

    pub fn as_u16(&self, rdr: &ConfReader) -> Result<u16, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<u16>() else {
            return Err(format!("'{}' could not be parsed as a `u16` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_u32_option(&self, rdr: &ConfReader) -> Option<u32> {
        self.as_u32(rdr).ok()
    }

    pub fn as_u32(&self, rdr: &ConfReader) -> Result<u32, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<u32>() else {
            return Err(format!("'{}' could not be parsed as a `u32` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_u64_option(&self, rdr: &ConfReader) -> Option<u64> {
        self.as_u64(rdr).ok()
    }

    pub fn as_u64(&self, rdr: &ConfReader) -> Result<u64, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<u64>() else {
            return Err(format!("'{}' could not be parsed as a `u64` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_u128_option(&self, rdr: &ConfReader) -> Option<u128> {
        self.as_u128(rdr).ok()
    }

    pub fn as_u128(&self, rdr: &ConfReader) -> Result<u128, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<u128>() else {
            return Err(format!("'{}' could not be parsed as a `u128` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_i8_option(&self, rdr: &ConfReader) -> Option<i8> {
        self.as_i8(rdr).ok()
    }

    pub fn as_i8(&self, rdr: &ConfReader) -> Result<i8, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<i8>() else {
            return Err(format!("'{}' could not be parsed as an `i8` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_i16_option(&self, rdr: &ConfReader) -> Option<i16> {
        self.as_i16(rdr).ok()
    }

    pub fn as_i16(&self, rdr: &ConfReader) -> Result<i16, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<i16>() else {
            return Err(format!("'{}' could not be parsed as an `i16` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_i32_option(&self, rdr: &ConfReader) -> Option<i32> {
        self.as_i32(rdr).ok()
    }

    pub fn as_i32(&self, rdr: &ConfReader) -> Result<i32, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<i32>() else {
            return Err(format!("'{}' could not be parsed as an `i32` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_i64_option(&self, rdr: &ConfReader) -> Option<i64> {
        self.as_i64(rdr).ok()
    }

    pub fn as_i64(&self, rdr: &ConfReader) -> Result<i64, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<i64>() else {
            return Err(format!("'{}' could not be parsed as an `i64` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_i128_option(&self, rdr: &ConfReader) -> Option<i128> {
        self.as_i128(rdr).ok()
    }

    pub fn as_i128(&self, rdr: &ConfReader) -> Result<i128, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<i128>() else {
            return Err(format!("'{}' could not be parsed as an `i128` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_f32_option(&self, rdr: &ConfReader) -> Option<f32> {
        self.as_f32(rdr).ok()
    }

    pub fn as_f32(&self, rdr: &ConfReader) -> Result<f32, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<f32>() else {
            return Err(format!("'{}' could not be parsed as an `f32` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }

    pub fn as_f64_option(&self, rdr: &ConfReader) -> Option<f64> {
        self.as_f64(rdr).ok()
    }

    pub fn as_f64(&self, rdr: &ConfReader) -> Result<f64, String> {
        let value = self.as_string(rdr)?;

        let Ok(value) = value.parse::<f64>() else {
            return Err(format!("'{}' could not be parsed as an `f64` for the key: '{}'", value, self.key));
        };
        
        Ok(value)
    }
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

    fn get_value(&self, key: impl Into<String>) -> Option<String> {
        let mut val = None;
        let key = key.into();

        if let Some(rdr) = &self.dot_env_rdr {
            if let Some(v) = rdr.get_value(&key) {
                val = Some(v);
                // dbg!("DotEnvReader reader get_value: {:?}", &val);
            }
        };
        if let Some(rdr) = &self.env_rdr {
            if let Some(v) = rdr.get_value(&key) {
                val = Some(v);
                // dbg!("EnvReader reader get_value: {:?}", &val);
            }
        };
        if let Some(rdr) = &self.args_rdr {
            if let Some(v) = rdr.get_value(&key) {
                val = Some(v);
                // println!("ArgsReader reader get_value: {} = {:?}", &key, &val);
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
        let config = ConfReader::default();

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
    fn read_config_option_none() {
        let rdr = ConfReader::default();

        let config = TestConfig {
            uid: ConfVal::new("NONE").as_string_option(&rdr),
            ..TestConfig::default()
        };

        assert!(config.uid.is_none());
        assert_eq!(config.uid, None);
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

    #[test]
    fn read_config_required_u16_value() {
        let rdr = ConfReader::default();

        let config = TestConfig {
            // db_con_str: ConfVal::new("DB").as_string(&rdr).unwrap(),
            port: ConfVal::new("PORT").as_u16(&rdr).unwrap(),
            ..TestConfig::default()
        };

        // assert_eq!(config.db_con_str, "Postgres".to_owned());
        assert_eq!(config.port, 5432);
    }

    #[test]
    fn read_config_required_bool_value() {
        let rdr = ConfReader::default();

        let config = TestConfig {
            // db_con_str: ConfVal::new("DB").as_string(&rdr).unwrap(),
            use_tls: ConfVal::new("TLS").as_bool(&rdr).unwrap(),
            ..TestConfig::default()
        };

        // assert_eq!(config.db_con_str, "Postgres".to_owned());
        assert_eq!(config.use_tls, false);
    }
}
