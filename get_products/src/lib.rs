pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    if arr.len() == 0 {
        return result;
    }
    for i in 0..arr.len() {
        let mut product = 1;
        for j in 0..arr.len() {
            if i != j {
                product *= arr[j];
            }
        }
        result.push(product);
    }
    result
}