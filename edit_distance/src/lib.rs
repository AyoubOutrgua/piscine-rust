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
