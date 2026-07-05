pub fn num_to_ordinal(x: u32) -> String {
    if x % 100 == 11 || x % 100 == 12 || x % 100 == 13 {
        return format!("{}th",x);
    }
    match x % 10 {
        1 => format!("{}st",x),
        2 => format!("{}nd",x),
        3 => format!("{}rd",x),
        _ => format!("{}th",x),
    }
}