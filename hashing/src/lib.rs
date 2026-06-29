use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let  sum: i32= list.iter().sum();
    let len =  list.len();
    (sum as f64) / (len as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let len =  list.len();
    let mut arr = list.to_vec();
    arr.sort();
    if len % 2 != 0 {
        return arr[len/2];
    } 
    (arr[(len/2) - 1] + arr[len/2]) / 2    
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();
    for n in list {
        if let Some(v) = map.get(&n) {
            map.insert(*n, v + 1);
        } else {
            map.insert(*n, 1);
        }
    }
    let mut max = 0;
    let mut num: i32 = 0;
    for (key,val) in &map {
        if *val > max {
            max = *val;
            num = *key
        }
    }
    num
}