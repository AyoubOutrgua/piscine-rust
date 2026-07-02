use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut map: HashMap<&'a str, usize> = HashMap::new();

    for key in words {
        if let Some(count) = map.get(key) {
            map.insert(key, count +1);
        } else {
            map.insert(key, 1);
        }
    }
    map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}