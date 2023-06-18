use std::{collections::HashMap, fs, path::Path};

/// The method that trues to read values from a `.env` file in the present working directory.
pub fn read_default(hash_map: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
    let file = Path::new("./.env");
    if file.is_file() {
        if let Ok(contents) = fs::read_to_string(file) {
            for line in contents.lines() {
                if line.starts_with('#') {
                    continue;
                }

                let kvp: Vec<&str> = line.split('=').collect();
                if kvp.len() == 2 {
                    hash_map.insert(kvp[0].trim().to_owned(), kvp[1].trim().to_owned());
                }
            }
        }
    }

    hash_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_dotenv() {
        let mut hash_set: HashMap<String, String> = HashMap::new();
        let result = super::read_default(&mut hash_set);

        assert_eq!(result.len(), 1);
        assert_eq!(result["NAME"], "Francis");
    }
}
