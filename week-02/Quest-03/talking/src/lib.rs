pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
    let last = text.chars().last().unwrap();
    if is_all_upper(text) && last == '?' {
        return "Quiet, I am thinking!";
    }
    if is_all_upper(text) {
        return "There is no need to yell, calm down!";
    }
    if last == '?' {
        return "Sure."
    }
    "Interesting"
}

fn is_all_upper(text: &str) -> bool {
    for c in text.chars() {
        if c.is_alphabetic() && c.is_lowercase() {
            return false;
        }
    }
    if !text.chars().any(|c| c.is_alphabetic()) {
        return false;
    }
    true
}