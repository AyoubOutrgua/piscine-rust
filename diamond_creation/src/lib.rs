pub fn get_diamond(c: char) -> Vec<String> {
    let max_index = (c as u8 - b'A') as usize;
    let mut diamond = Vec::<String>::new();
    
    for i in 0..=max_index {
        let mut s = String::new();
        let current_char = (b'A' + i as u8) as char;
        let out_space = " ".repeat(max_index - i);
        let mut inside_space = String::new();
        if i != 0 {
            inside_space = " ".repeat((2 * i )- 1);
        }
        s.push_str(&out_space);
        s.push(current_char);
        s.push_str(&inside_space);
        if i != 0 {
            s.push(current_char);
        }
        s.push_str(&out_space);
        diamond.push(s);
    }
    let mut rev_diamond = diamond.clone();
    rev_diamond.pop();
    rev_diamond.reverse();
    
    for d in rev_diamond {
        diamond.push(d);
    }
    diamond
}