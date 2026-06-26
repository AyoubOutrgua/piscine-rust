pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let x: f64 = c as f64;
    (c, x.exp(), x.abs().ln())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let copy = b.clone();
    let mut nums: Vec<f64> = Vec::new();

    for n in copy {
        let x = n as f64;
        nums.push(x.abs().ln());
    }
    (b, nums)

}

pub fn str_function(a: String) -> (String, String) {
    let copy = a.clone();
    let nums = copy.split_whitespace();
    let mut res = String::new();
    for n in nums {
        let x: f64 = n.parse().unwrap();
        res.push_str(&x.exp().to_string());
        res.push(' ');
    }
    res.pop();
    (a, res)
}