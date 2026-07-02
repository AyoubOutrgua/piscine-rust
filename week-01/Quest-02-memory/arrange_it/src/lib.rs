pub fn arrange_phrase(phrase: &str) -> String {
    let mut res = vec![String::new(); phrase.split_whitespace().count()];
    for word in phrase.split_whitespace() {
        println!("{}", word);
        let mut new_word = String::with_capacity(word.chars().count());
        let mut index: usize = 0;
        for c in word.chars() {
            if c.is_numeric() {
                index = c.to_digit(10).unwrap() as usize;
                continue;
            }
            new_word.push(c);
        }
        res[index-1] = new_word;
    }
    res.join(" ")
}