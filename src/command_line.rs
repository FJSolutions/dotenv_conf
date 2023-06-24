use std::{collections::HashMap, env};

use crate::ReadConf;

pub struct ArgsReader {
    hash_map: HashMap<String, String>,
}

impl ArgsReader {
    pub fn new() -> ArgsReader {
        let hash_map: HashMap<String, String> = HashMap::new();
        let mut rdr = ArgsReader { hash_map };
        rdr.read_config();
        rdr
    }
}

impl ReadConf for ArgsReader {
    fn read_config(&mut self) {
        let args: Vec<String> = env::args().collect();
        let mut index: usize = 1;
        'l: loop {
            match read_key_value(&args, index) {
                Some((i, k, v)) => {
                    self.hash_map.insert(k, v);
                    index = i;
                }
                None => break 'l,
            }
        }
    }

    fn get_value(&self, key: impl Into<String>) -> Option<String> {
        self.hash_map.get(&key.into()).cloned()
    }
}

fn read_key_value(args: &[String], index: usize) -> Option<(usize, String, String)> {
    if let Some(k) = args.get(index) {
        if let Some(v) = args.get(index + 1) {
            if v.starts_with("--") || v.starts_with('-') {
                Some((index + 1, k.to_owned(), String::new()))
            } else {
                Some((index + 2, k.to_owned(), v.to_owned()))
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rdr = ArgsReader::new();
        rdr.read_config();

        dbg!(&rdr.hash_map);
        // assert_eq!(result.len(), 0);
    }
}
