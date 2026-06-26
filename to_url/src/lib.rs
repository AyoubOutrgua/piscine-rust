pub fn to_url(s: &str) -> String {
    let mut res = String::new();
    for c in s.chars() {
        if c == ' ' {
            res.push_str("%20");
        } else {
            res.push(c);
        }
    }
    res
}