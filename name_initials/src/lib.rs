pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for v in names {
        res.push(v.chars().next().unwrap().to_string())
    }
    res
}