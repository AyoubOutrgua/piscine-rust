pub fn is_pangram(s: &str) -> bool {
    let str = s.to_lowercase();
    for c in 'a'..='z' {
        if !str.contains(c) {
            return false;
        }
    }
    true
}