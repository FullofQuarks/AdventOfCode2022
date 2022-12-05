use std::collections::HashMap;

pub fn execute() {
    let mut games: Vec<u16> = Vec::new();
    let choice_mapper: HashMap<char, char> = HashMap::from([
        ('X', 'A'),
        ('Y', 'B'),
        ('Z', 'C')
    ]);
    let game_rules: HashMap<char, char> = HashMap::from([
        ('A', 'C'),
        ('B', 'A'),
        ('C', 'B')
    ]);
    let choice_points: HashMap<char, u16> = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3)
    ]);
    let mut total_score: u16 = 0;

    if let Ok(lines) = crate::utilities::read_lines("src/days/day_02/02.txt") {
        for line in lines {
            let mut chars = line.char_indices();

            // nth consumes all chars to the index
            let opponent_move = chars.nth(0).unwrap().1;
            let raw_suggested_move = chars.nth(1).unwrap().1;
            let suggested_move = choice_mapper.get(&raw_suggested_move).unwrap();
            let mut game_points: u16 = 0;

            if &opponent_move == suggested_move {
                game_points += 3;
            } else if game_rules.get(&suggested_move).unwrap() == &opponent_move {
                game_points += 6;
            }
            let current_game_score = choice_points.get(suggested_move).unwrap() + game_points;

            games.push(current_game_score);
            total_score += current_game_score;
        }
    }

    println!("Part 1 Total game score {}", &total_score);
}

fn part_two() {}