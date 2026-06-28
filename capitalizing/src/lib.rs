pub fn capitalize_first(input: &str) -> String {
    let mut s = String::new();
    for (i, c) in input.chars().enumerate() {
        if i == 0 {
            s.push(c.to_ascii_uppercase());
            continue;
        }
        s.push(c);
    }
    s
}

pub fn title_case(input: &str) -> String {
    let mut words = String::new();
    let mut next: bool = false;
    for c in input.chars() {
        if c.is_whitespace() {
            next = true;
        }
        if next {
            words.push(c.to_ascii_uppercase());
            next = false;
            continue;
        }
        words.push(c);
    }
    words
}

pub fn change_case(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars() {
        if c.is_ascii_uppercase() {
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c.to_ascii_uppercase());
        }
    }
    s
}