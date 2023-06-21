use std::{collections::HashMap, fs, path::Path};

use crate::ReadConfig;

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

impl ReadConfig for DotEnvReader {
    fn read_config(&mut self) {
        let file = Path::new("./.env");
        if file.is_file() {
            if let Ok(contents) = fs::read_to_string(file) {
                for line in contents.lines() {
                    if line.starts_with('#') {
                        continue;
                    }

                    let kvp: Vec<&str> = line.split('=').collect();
                    if kvp.len() == 2 {
                        self.hash_map
                            .insert(kvp[0].trim().to_owned(), kvp[1].trim().to_owned());
                    }
                }
            }
        }
    }

    fn get_value(&self, key: impl Into<String>) -> Option<String> {
        self.hash_map.get(&key.into()).map(|val| val.clone())
    }
}

/// The method that trues to read values from a `.env` file in the present working directory.
// pub fn read_default(hash_map: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
//     let file = Path::new("./.env");
//     if file.is_file() {
//         if let Ok(contents) = fs::read_to_string(file) {
//             for line in contents.lines() {
//                 if line.starts_with('#') {
//                     continue;
//                 }

//                 let kvp: Vec<&str> = line.split('=').collect();
//                 if kvp.len() == 2 {
//                     hash_map.insert(kvp[0].trim().to_owned(), kvp[1].trim().to_owned());
//                 }
//             }
//         }
//     }

//     hash_map
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_dotenv() {
        let mut rdr = DotEnvReader::new();
        rdr.read_config();

        assert_eq!(rdr.hash_map.len(), 1);
        assert_eq!(rdr.get_value("NAME".to_owned()), Some("Francis".to_owned()));
    }
}
