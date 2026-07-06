pub fn scytale_cipher(message: &str, i: usize) -> String {
    let chars: Vec<char> = message.chars().collect();
    let mut res = String::new();
    for j in 0..i {
        let mut k = j;
        while k <= chars.len() {
            if let Some(c) = chars.get(k) {
                println!("in **** {}", c);
                res.push(*c);
            } else {
                res.push(' ');
                println!("out ----");
            }
            k += i;
        }
    }
    res
}