use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Matches {
    matches: Vec<&'static str>,
}

pub fn compare_strings() -> Matches {
    let vec1 = vec!["F5", "F7", "C6", "H5", "G5"];
    let vec2 = vec!["B4", "D4", "G5", "F5", "F7", "C6"];
    let mut result = Vec::new();

    let mut string_map = HashMap::new();
    for s in vec2 {
        string_map.insert(s, true);
    }

    for s in vec1 {
        match string_map.get_key_value(s) {
            None => {}
            Some(_quotient) => {
                result.push(s);
            }
        }
    }

    println!("{:?}", result);

    Matches {
        matches: result.to_vec(),
    }
}
