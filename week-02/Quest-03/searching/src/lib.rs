pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().rposition(|&n| n == key)
}