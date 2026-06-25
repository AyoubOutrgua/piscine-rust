pub fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }
    factorial(num - 1) * num
}