pub fn first_subword(mut s: String) -> String {
    let mut word = String::new();
    for (i,c) in s.chars().enumerate() {
        if (c.is_ascii_uppercase() && i != 0) || c == '_' {
            break;
        }
        word.push(c)
    }
    word
}