use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn execute() {
    let elf_pairs: Vec<String> = crate::utilities::read_lines("src/days/day_04/04.txt")
        .expect("There are no sections");

    let mut encompassed_sections: Vec<(HashSet<u16>, HashSet<u16>)> = Vec::new();
    let mut any_overlapped_sections: Vec<(HashSet<u16>, HashSet<u16>)> = Vec::new();
    for pair in elf_pairs.iter() {
        let (first, second): (HashSet<u16>, HashSet<u16>) = parse_pairs(pair);
        let combined_length: usize = first.clone().union(&second).count();
        let encompassed = combined_length == first.len() || combined_length == second.len();
        if encompassed {
            encompassed_sections.push((first.clone(), second.clone()));
        }
        if combined_length != (first.len() + second.len()) {
            any_overlapped_sections.push((first.clone(), second.clone()));
        }
    }

    println!("Part 1 number of sections: {}", encompassed_sections.len());
    println!("Part 2 number of sections: {}", any_overlapped_sections.len());
}

fn parse_pairs(pair: &String) -> (HashSet<u16>, HashSet<u16>) {
    let pairs: Vec<&str> = pair.split(',').collect();
    let section_one: Vec<u16> = pairs[0]
        .split('-')
        .map(|c: &str| c.parse::<u16>().unwrap())
        .collect();
    let section_two: Vec<u16> = pairs[1]
        .split('-')
        .map(|c: &str| c.parse::<u16>().unwrap())
        .collect();

    let section_one_range: RangeInclusive<u16> = section_one[0]..=section_one[1];
    let section_two_range: RangeInclusive<u16> = section_two[0]..=section_two[1];
    let set_one: HashSet<u16> = HashSet::from_iter(section_one_range);
    let set_two: HashSet<u16> = HashSet::from_iter(section_two_range);

    (set_one, set_two)
}