pub fn scytale_cipher(message: &str, i: usize) -> String {
    let chars: Vec<char> = message.chars().collect();
    let mut res = String::new();
    for j in 0..i {
        let mut k = j;
        while k < chars.len() {
            res.push(chars[k]);
            k += i;
        }
    }
    res
}