use std::{collections::HashMap, env};

pub fn read_from_args<'a>(
    hash_map: &'a mut HashMap<String, String>,
) -> &'a mut HashMap<String, String> {
    let args: Vec<String> = env::args().collect();
    let mut index: usize = 1;
    'l: loop {
        match read_key_value(&args, index) {
            Some((i, k, v)) => {
                hash_map.insert(k, v);
                index = i;
            }
            None => break 'l,
        }
    }
    hash_map
}

fn read_key_value(args: &Vec<String>, index: usize) -> Option<(usize, String, String)> {
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
        let mut hash_map: HashMap<String, String> = HashMap::new();
        let result = read_from_args(&mut hash_map);

        dbg!(&result);
        assert_eq!(result.len(), 0);
    }
}
