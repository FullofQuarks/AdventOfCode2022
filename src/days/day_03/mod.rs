use std::collections::{HashMap, HashSet};

pub fn execute() {
    let rucksacks = crate::utilities::read_lines("src/days/day_03/03.txt")
        .expect("There are no rucksacks");

    let alphabet_priority_map = get_alphabet_priority();

    let mut common_items: Vec<char> = Vec::new();
    let mut part_two_rucksack_occurrences: [HashSet<char>; 3] = Default::default();
    let mut number_in_group: u8 = 0;
    let mut part_two_badges: Vec<char> = Vec::new();
    for rucksack in rucksacks {
        if number_in_group == 3 {
            part_two_badges.push(get_common_letter_from_hashset_group(&part_two_rucksack_occurrences));
            number_in_group = 0;
        }
        part_two_rucksack_occurrences[number_in_group as usize].clear();

        let items = rucksack.chars();
        for item in items.into_iter() {
            part_two_rucksack_occurrences[number_in_group as usize].insert(item);
        }

        number_in_group += 1;
    }
    part_two_badges.push(get_common_letter_from_hashset_group(&part_two_rucksack_occurrences));

    let mut total_priority: u16 = 0;
    let mut part_two_priority: u16 = 0;
    for common_item in common_items.iter() {
        total_priority += *alphabet_priority_map.get(common_item).unwrap() as u16;
    }
    for badge in part_two_badges.iter() {
        part_two_priority += *alphabet_priority_map.get(badge).unwrap() as u16;
    }
    println!("Part 1 total priority: {}", total_priority);
    println!("Part 2 total priority: {}", part_two_priority);
}

fn get_alphabet_priority() -> HashMap<char, u8> {
    let lower_alphabet: [char; 26] = (b'a'..=b'z')
        .map(|c| c as char)
        .collect::<Vec<_>>()
        .try_into()
        .expect("oops");
    let upper_alphabet: [char; 26] = (b'A'..=b'Z')
        .map(|c| c as char)
        .collect::<Vec<_>>()
        .try_into()
        .expect("oops");
    let mut alphabet_priority_map: HashMap<char, u8> = HashMap::new();
    let mut starting_priority: u8 = 1;
    for letter in lower_alphabet.iter().chain(upper_alphabet.iter()) {
        alphabet_priority_map.insert(*letter, starting_priority);
        starting_priority += 1;
    }

    alphabet_priority_map
}

fn get_common_letter_from_hashset_group(group: &[HashSet<char>; 3]) -> char {
    let first_second_intersect: HashSet<_> = group[0].intersection(&group[1]).map(|x: &char| *x).collect();
    let intersect_third: HashSet<_> = first_second_intersect.intersection(&group[2]).collect();
    let mut return_char: char = char::default();
    for &a in intersect_third.iter() {
        return_char = *a;
    }

    return_char
}