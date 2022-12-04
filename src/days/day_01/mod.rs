pub fn execute() {
    let mut file_lines: Vec<u32> = vec![0];
    let mut current_elf = 0;
    if let Ok(lines) = crate::utilities::read_lines("src/days/day_01/01-01.txt") {
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
    for elf in &file_lines {
        if elf > &max[0] {
            max[2] = max[1];
            max[1] = max[0];
            max[0] = *elf;
        }
        else if elf > &max[1] {
            max[2] = max[1];
            max[1] = *elf;
        }
        else if elf > &max[2] {
            max[2] = *elf;
        }
    }

    for elf in max {
        println!("{} ", elf);
    }
}