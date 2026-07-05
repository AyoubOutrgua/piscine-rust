pub fn pig_latin(text: &str) -> String {
    let mut res = String::new();
    for word in text.split_whitespace() {
        res.push_str(&pig(word));
    }
    res
}

fn pig(word: &str) -> String {
    let vowels = "aeiou";
    for c in vowels.chars() {
        if word.starts_with(c) {
            return format!("{}ay",word);
        }
    }

    let firs_three: String = word.chars().take(3).collect();
    if firs_three.contains("qu") {
        if word.starts_with("qu") {
            let first_two: String = word.chars().take(2).collect();
            let skip_two: String = word.chars().skip(2).collect();
            return format!("{}{}ay",skip_two,first_two)
        }
        let skip_three: String = word.chars().skip(3).collect();
        return format!("{}{}ay",skip_three,firs_three)
    }

    let mut first = String::new();
    let mut second = String::new();
    let mut is_second = false;
    for c in word.chars() {
        if vowels.contains(c) && !is_second {
            is_second = true;
            second.push(c);
        } else if !is_second {
            first.push(c);
        } else if is_second {
            second.push(c);
        }
    }
    format!("{}{}ay",second,first)
}