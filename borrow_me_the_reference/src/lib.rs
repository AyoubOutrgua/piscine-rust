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
        if val.contains('+') {
            let x = val.split('+');
            let mut sum: usize = 0;
            for n in x {
                let num: usize = n.parse().unwrap();
                sum += num;
            }
            *val = sum.to_string();
        } else if val.contains('-') {
            let x = val.split('-');
            let mut min: usize = 0;
            for n in x {
                let num: usize = n.parse().unwrap();
                min -= num;
            }
            *val = min.to_string();
        }
    }
}