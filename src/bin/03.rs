use std::{fs, collections::HashSet};


fn char_to_value(c: &char) -> u8 {
    assert!(c.is_ascii_alphabetic(), "Expected alphabetic character. Got {c} instead.");
    if c.is_ascii_uppercase() {
        return *c as u8 - b'A' + 27;
    } else {
        return *c as u8 - b'a' + 1;
    }
}


fn main() {
    let lines: Vec<String> = fs::read_to_string("inputs/03.txt")
                    .unwrap()  
                    .lines()  
                    .map(String::from)  
                    .collect();
    
    let mut part_one_result: u32 = 0;
    let mut part_two_result: u32 = 0;

    let mut line_one_set: HashSet<char> = HashSet::new();
    let mut line_two_set: HashSet<char> = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        let second_half_start = line.len() / 2;
        let first_half = &line[..second_half_start];
        let second_half = &line[second_half_start..];

        let mut letter_count: HashSet<char> = HashSet::new();

        for c in first_half.chars() {
            letter_count.insert(c);
        }

        // Finds letter that exists in both groups
        for c in second_half.chars() {
            if letter_count.contains(&c) {
                part_one_result += char_to_value(&c) as u32;
                break;
            }
        }

        // Find common letter in all 3 lines
        for c in line.chars() {
            match i % 3 {
                0 => { line_one_set.insert(c); },
                1 => { line_two_set.insert(c); },
                2 => {
                    if line_one_set.contains(&c) && line_two_set.contains(&c) {
                        line_one_set.clear();
                        line_two_set.clear();
                        part_two_result += char_to_value(&c) as u32;
                        break;
                    }
                },
                _ => panic!("Should never be here!"),
            }
        }
    }

    println!("Part One Result: {part_one_result}");
    println!("Part Two Result: {part_two_result}");
}
