use std::str::FromStr;

use super::keyboard_event_handler::KeyCode;

pub struct KeyCombination {
    combination: Vec<KeyCode>,
    command: String,
}

fn parce_combination(combination: String) -> Result<Vec<KeyCode>, ()> {
    let keys_result: Vec<Result<KeyCode, ()>> =
        combination.split(" + ").map(|key_string| todo!()).collect();
    let mut keys = Vec::new();
    for key in keys_result {
        if let Ok(key) = key {
            keys.push(key);
            continue;
        }
        return Err(());
    }
    Ok(keys)
}
