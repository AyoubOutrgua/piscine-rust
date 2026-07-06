pub fn scytale_cipher(message: &str, i: usize) -> String {
    let mut res = String::new();
    if message.is_empty() {
        return res;
    }
    let mut chunks: Vec<Vec<char>> = Vec::new();
    let chars: Vec<char> = message.chars().collect();

    for chunk in chars.chunks(i) {
        chunks.push(chunk.to_vec());
    }
    for j in 0..i {
        for k in 0..chunks.len() {
            if let Some(c) = chunks[k].get(j) {
                res.push(*c);
            } else {
                res.push(' ');
            }
        }
    }
    res.trim_end_matches(' ').to_string()
}