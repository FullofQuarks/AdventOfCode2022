use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut file_lines: Vec<u32> = vec![0];
    let mut current_elf = 0;
    if let Ok(lines) = read_lines("src/days/01/01-01.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    current_elf += 1;
                    file_lines.push(0);
                }
                else {
                    file_lines[current_elf] = file_lines[current_elf] + ip.parse::<u32>().unwrap();
                }
            }
        }
    }

    let mut max: Vec<u32> = vec![0, 0, 0];
    for &elf in file_lines.iter() {
        if elf > max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = elf;
        }
        else if elf > max[1] {
            max[2] = max[1];
            max[1] = elf;
        }
        else if elf > max[2] {
            max[2] = elf;
        }
    }

    for elf in max {
        println!("{} ", elf);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}