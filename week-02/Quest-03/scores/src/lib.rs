pub fn score(s: &str) -> usize {
    let one = ['A', 'E' , 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'];
    let two = ['D', 'G'];
    let three = ['B', 'C', 'M', 'P'];
    let four = ['F', 'H', 'V', 'W', 'Y'];
    let five = ['K'];
    let eight = ['J', 'X'];
    let ten = ['Q', 'Z'];
    let mut res = 0;
    for c in s.chars() {
        if one.contains(&c.to_ascii_uppercase()) {
            res += 1;
        } else if two.contains(&c.to_ascii_uppercase()) { 
            res += 2;
        } else if three.contains(&c.to_ascii_uppercase()) { 
            res += 3;
        } else if four.contains(&c.to_ascii_uppercase()) { 
            res += 4;
        } else if five.contains(&c.to_ascii_uppercase()) { 
            res += 5;
        } else if eight.contains(&c.to_ascii_uppercase()) { 
            res += 8;
        } else if ten.contains(&c.to_ascii_uppercase()) { 
            res += 10;
        }
    }
    res
}