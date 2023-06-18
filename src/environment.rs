use std::collections::HashMap;

pub fn read_from_env<'a>(
    hash_map: &'a mut HashMap<String, String>,
    key: &str,
) -> &'a mut HashMap<String, String> {
    hash_map
}
