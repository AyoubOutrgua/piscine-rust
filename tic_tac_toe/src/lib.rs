pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let player_x = String::from("player X won");
    let player_o = String::from("player O won");
    let tie = String::from("tie");
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return player_x;
    } else if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return player_o;
    } 
    tie
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }
    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }
    return false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[i][0] == player && table[i][1] == player && table[i][2] == player {
            return true;
        }
    }
    return false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if table[0][i] == player && table[1][i] == player && table[2][i] == player {
            return true;
        }
    }
    return false
}