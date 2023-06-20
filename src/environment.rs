use crate::ReadConfig;

pub struct EnvConfigReader {}

impl EnvConfigReader {
    fn new() -> EnvConfigReader {
        EnvConfigReader {}
    }
}

impl ReadConfig for EnvConfigReader {
    fn read_config(&mut self) {}

    fn get_value(&self, key: String) -> Option<String> {
        match std::env::var(key) {
            Ok(val) => Some(val),
            _ => None,
        }
    }
}
