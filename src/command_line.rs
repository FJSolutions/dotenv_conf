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
        // dbg!("{}", &args);
        let mut index: usize = 1;
        'l: loop {
            match read_key_value(&args, index) {
                Some((i, k, v)) => {
                    // dbg!("Command line argument: ({}, {})", &k, &v);
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
    // dbg!("read_key_value: {} = {}", &index, &args);
    if let Some(k) = args.get(index) {
        if let Some(v) = args.get(index + 1) {
            if k.starts_with("--") {
                // println!("Double dash command line argument");
                Some((index + 1, k[2..].to_owned(), v.to_owned()))
            } else if k.starts_with("-") {
                // println!("Single dash command line argument");
                Some((index + 1, k[1..].to_owned(),  v.to_owned()))
            } else {
                // println!("No dash command line argument");
                Some((index + 1, k.to_owned(), v.to_owned()))
            }
        } else {
            // It's a switch not a value option
            if k.starts_with("--") {
                println!("Double dash command line argument with no value");
                Some((index + 1, k[2..].to_owned(), String::new()))
            } else if k.starts_with("-") {
                println!("Single dash command line argument with no value");
                Some((index + 1, k[1..].to_owned(), String::new()))
            }else{
                Some((index + 1, k.to_owned(), String::new()))
            }
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
