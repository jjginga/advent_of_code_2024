use std::collections::HashMap;

pub fn create_map(input: &[String]) -> HashMap<(i16, i16), char> {
    let mut map = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            map.insert(((row) as i16, (col) as i16), ch);
        }
    }
    map
}