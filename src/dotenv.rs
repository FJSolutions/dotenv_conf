use std::{collections::HashMap, fs, path::Path};

use crate::ReadConf;

pub struct DotEnvReader {
    hash_map: HashMap<String, String>,
}

impl DotEnvReader {
    pub fn new() -> DotEnvReader {
        let hash_map: HashMap<String, String> = HashMap::new();
        let mut rdr = DotEnvReader { hash_map };
        rdr.read_config();
        rdr
    }
}

impl ReadConf for DotEnvReader {
    fn read_config(&mut self) {
        let file = Path::new("./.env");
        if file.is_file() {
            if let Ok(contents) = fs::read_to_string(file) {
                for line in contents.lines() {
                    if line.starts_with('#') {
                        continue;
                    } else if line.trim().is_empty() {
                        continue;
                    }

                    if let Some(index) = line.find('=') {
                        let (k , v) = line.split_at(index);
                            self.hash_map
                                .insert(k.trim().to_owned(), v[1..].trim().to_owned());
                    }
                }
            }
        }
    }

    fn get_value(&self, key: impl Into<String>) -> Option<String> {
        self.hash_map.get(&key.into()).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_dotenv() {
        let mut rdr = DotEnvReader::new();
        rdr.read_config();

        assert_eq!(rdr.hash_map.len(), 6);
        assert_eq!(rdr.get_value("NAME".to_owned()), Some("Francis".to_owned()));
    }
}
