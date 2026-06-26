pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::with_capacity(names.len());
    for name in names {
        let words = name.split_whitespace();
        let mut caracters = String::new();
        for c in words {
            let first_char = c.chars().next().unwrap();
            caracters.push(first_char);
            caracters.push_str(". ");
        }
        caracters.pop();
        res.push(caracters);
    }
    res
}