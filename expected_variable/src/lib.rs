use case::CaseExt;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let comp_lower = compared.to_lowercase();
        if comp_lower != compared.to_camel().to_lowercase() 
        && comp_lower != compared.to_snake().to_lowercase() {
        return None;
    }
    let expected_len = expected.len() as f64;
    if expected_len == 0.0 { return None; }
    let distance = edit_distance(&comp_lower, &expected.to_lowercase()) as f64;
    let alikeness = ((1.0 - (distance / expected_len)) * 100.0).round();
    (alikeness > 50.0).then(|| format!("{}%", alikeness as i32))
}


pub fn edit_distance(source: &str, target: &str) -> usize {
    let x = target.chars().count();
    let y = source.chars().count();
    let tb_s: Vec<char> = source.chars().collect();
    let tb_t: Vec<char> = target.chars().collect();
    let mut table = vec![vec![0; x + 1]; y + 1];
    for i in 0..x+1 {
        table[0][i] = i;
    }
    for j in 0..y+1 {
        table[j][0] = j;
    }
    for i in 1..=y {
        for j in 1..=x {
            if tb_s[i - 1] == tb_t[j - 1] {
                table[i][j] = table[i - 1][j - 1];
            } else {
                let top = table[i - 1][j];
                let left = table[i][j - 1];
                let daig = table[i - 1][j - 1];
                let min = left.min(top).min(daig);
                table[i][j] = min + 1;
            }
        }
    }
    let res = table[y][x];
    res
}