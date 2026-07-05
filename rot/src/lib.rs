pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();
    let k = key.rem_euclid(26);
    for c in input.chars() {
        if c.is_alphabetic() && c.is_uppercase() {
            let base = 'A' as u8;
            let new = (((c as u8 - base + k as u8) % 26) + base) as char;
            res.push(new);
        }else if c.is_alphabetic() && c.is_lowercase() {
            let base = 'a' as u8;
            let new = (((c as u8 - base + k as u8) % 26) + base) as char;
            res.push(new);
        } else {
            res.push(c);
        }
    }
    res
}