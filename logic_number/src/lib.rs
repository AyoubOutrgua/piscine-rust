pub fn number_logic(num: u32) -> bool {
    let s = num.to_string();
    let digits = s.len() as u32;
    let mut sum: u32 = 0;
    for c in s.chars() {
        let n = c.to_digit(10).unwrap();
        sum += n.pow(digits);
    }
    sum == num
}