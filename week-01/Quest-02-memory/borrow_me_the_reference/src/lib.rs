pub fn delete_and_backspace(s: &mut String) {
    let mut new_str = String::new();
    let mut count: u32 = 0;

    for c in s.chars() {
        if count > 0 {
            if c != '+' {
                count -= 1;
            } else {
                count += 1;
            }
            continue;
        }

        if c == '-' {
            new_str.pop();
        } else if c == '+' {
            count += 1;
        } else {
            new_str.push(c);
        }
    }

    *s = new_str;
}

pub fn do_operations(v: &mut [String]) {
    for val in v {
        let result: i32;
        
        if val.contains('+') {
            let parts: Vec<&str> = val.split('+').collect();

            result = parts[0].parse::<i32>().unwrap() + parts[1].parse::<i32>().unwrap();
        } else {
            let parts: Vec<&str> = val.split('-').collect();

            result = parts[0].parse::<i32>().unwrap() - parts[1].parse::<i32>().unwrap();
        }

        *val = result.to_string();
    }
}