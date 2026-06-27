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
    let mut s = String::new();
    for word in input.split_whitespace() {
        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                s.push(c.to_ascii_uppercase());
                continue;
            }
            s.push(c);
        }
        s.push(' ');
    }
    s.pop();
    s
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