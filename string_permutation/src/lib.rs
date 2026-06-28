use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let map_s1: HashMap<char, usize> = generate_map(s1);
    let map_s2: HashMap<char, usize> = generate_map(s2);

    map_s1 == map_s2
}

pub fn generate_map(s: &str) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();

    for key in s.chars() {
        if let Some(count) = map.get(&key) {
            map.insert(key, count +1);
        } else {
            map.insert(key, 1);
        }
    }
    map
}