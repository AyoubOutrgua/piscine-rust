pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut vec: Vec<Box<u32>> = Vec::new();
    for word in s.split_whitespace() {
        let num_u32 = if word.ends_with('k') {
            let n = word.trim_end_matches('k');
            let num: f64 = n.parse().unwrap();
            (num * 1000.0) as u32
        } else {
            word.parse().unwrap()
        };
        let b = Box::new(num_u32);
        vec.push(b);
    }
    vec
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for vb in a {
        vec.push(*vb);
    }
    vec
}