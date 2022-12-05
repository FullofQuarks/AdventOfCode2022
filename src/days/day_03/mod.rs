use std::collections::{HashMap, HashSet};

pub fn execute() {
    let rucksacks = crate::utilities::read_lines("src/days/day_03/03.txt")
        .expect("There are no rucksacks");

    let alphabet_priority_map = get_alphabet_priority();

    let mut common_items: Vec<char> = Vec::new();
    for rucksack in rucksacks {
        let items = rucksack.chars();
        let size_of_compartment: usize = rucksack.chars().count() / 2;
        let mut first_compartment_items: HashSet<char> = HashSet::new();
        for (index, item) in items.enumerate() {
            if index < size_of_compartment {
                first_compartment_items.insert(item);
            } else {
                if first_compartment_items.contains(&item) {
                    common_items.push(item);
                    break;
                }
            }
        }
    }

    let mut total_priority: u16 = 0;
    for common_item in common_items.iter() {
        total_priority += *alphabet_priority_map.get(common_item).unwrap() as u16;
    }
    println!("Total priority: {}", total_priority);
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