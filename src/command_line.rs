use std::{collections::HashMap, env};

use crate::ReadConfig;

pub struct CmdLineCfgReader {
    hash_map: HashMap<String, String>,
}

impl CmdLineCfgReader {
    fn new() -> CmdLineCfgReader {
        let hash_map: HashMap<String, String> = HashMap::new();
        CmdLineCfgReader { hash_map }
    }
}

impl ReadConfig for CmdLineCfgReader {
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

    fn get_value(&self, key: String) -> Option<String> {
        self.hash_map.get(&key).cloned()
    }
}

// pub fn read_from_args(hash_map: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
//     let args: Vec<String> = env::args().collect();
//     let mut index: usize = 1;
//     'l: loop {
//         match read_key_value(&args, index) {
//             Some((i, k, v)) => {
//                 hash_map.insert(k, v);
//                 index = i;
//             }
//             None => break 'l,
//         }
//     }
//     hash_map
// }

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
        let mut rdr = CmdLineCfgReader::new();
        rdr.read_config();

        dbg!(&rdr.hash_map);
        // assert_eq!(result.len(), 0);
    }
}
