pub fn execute() {
    let mut games: Vec<u32> = vec![0];
    let mut current_game = 0;
    if let Ok(lines) = crate::utilities::read_lines("src/days/day_02/02-Test.txt") {
        for line in lines {
            if let Ok(game) = &line {
                let string = line.unwrap();
                let mut chars = string.char_indices();

                // nth consumes all chars to the index
                let opponent_move = chars.nth(0).unwrap().1;
                let suggested_move = chars.nth(1).unwrap().1;
                if opponent_move == 'A' {
                    if suggested_move == 'Y' {
                        games[current_game] = 6;
                    }
                }
            }
            current_game += 1;
        }
    }
}