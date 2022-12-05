use std::collections::HashMap;

pub fn execute() {
    let mut part_one_game_scores: Vec<u16> = Vec::new();
    let mut part_two_game_scores: Vec<u16> = Vec::new();
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
    let mut part_one_total_score: u16 = 0;
    let mut part_two_total_score: u16 = 0;

    let games = crate::utilities::read_lines("src/days/day_02/02.txt").expect("Didn't get games");
    for game in games.iter() {
        let mut chars = game.char_indices();

        // nth consumes all chars to the index
        let opponent_move = chars.nth(0).unwrap().1;
        let raw_suggested_move = chars.nth(1).unwrap().1;
        let suggested_move = choice_mapper.get(&raw_suggested_move).unwrap();

        let part_one_game_score = get_points_from_moves(opponent_move, *suggested_move, &game_rules);
        part_one_game_scores.push(part_one_game_score);
        part_one_total_score += part_one_game_score;

        let decided_choice: &char;
        if *suggested_move == 'A' {
            decided_choice = game_rules.get(&opponent_move).unwrap();
        } else if *suggested_move == 'B' {
            decided_choice = &opponent_move;
        } else {
            let winning = game_rules.iter().find(|&(_x,y)| y == &opponent_move).expect("No matching");
            decided_choice = winning.0;
        }
        let part_two_game_score = get_points_from_moves(opponent_move, *decided_choice, &game_rules);
        part_two_game_scores.push(part_two_game_score);
        part_two_total_score += part_two_game_score;
    }


    println!("Part 1 Total game score {}", &part_one_total_score);
    println!("Part 2 Total game score {:?}", &part_two_total_score)
}

fn get_points_from_moves(first_move: char, second_move: char, game_rules: &HashMap<char, char>) -> u16 {
    let mut total_points: u16 = 0;
    let choice_points: HashMap<char, u16> = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3)
    ]);
    if first_move == second_move {
        total_points += 3;
    } else if game_rules.get(&second_move).unwrap() == &first_move {
        total_points += 6;
    }

    choice_points.get(&second_move).unwrap() + total_points
}