use crate::ReadConfig;

pub struct EnvReader {}

impl EnvReader {
    pub fn new() -> EnvReader {
        EnvReader {}
    }
}

impl ReadConfig for EnvReader {
    fn read_config(&mut self) {}

    fn get_value(&self, key: impl Into<String>) -> Option<String> {
        match std::env::var(key.into()) {
            Ok(val) => Some(val),
            _ => None,
        }
    }
}
