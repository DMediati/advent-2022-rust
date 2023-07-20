use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Weapon {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}


#[derive(Clone, Copy, PartialEq)]
enum MatchResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}


fn rps_beats( weapon: Weapon ) -> Weapon {
    match weapon {
        Weapon::Rock => Weapon::Scissors,
        Weapon::Paper => Weapon::Rock,
        Weapon::Scissors => Weapon::Paper,
    }
}


fn rps_loses_to( weapon: Weapon ) -> Weapon {
    match weapon {
        Weapon::Rock => Weapon::Paper,
        Weapon::Paper => Weapon::Scissors,
        Weapon::Scissors => Weapon::Rock,
    }
}


fn player_char_as_weapon_match_score( player_char: Option<&str>, opponent_weapon: Weapon  ) -> u32 {
    let player_weapon = match player_char {
        Some("X") => Weapon::Rock,
        Some("Y") => Weapon::Paper,
        Some("Z") => Weapon::Scissors,
        None => panic!("Empty player weapon"),
        _ => panic!("Invalid player weapon: {}", player_char.unwrap()),
    };

    let match_result = if opponent_weapon == player_weapon {
        MatchResult::Draw    
    } else if rps_beats(player_weapon) == opponent_weapon {
        MatchResult::Win
    } else {
        MatchResult::Loss
    };

    return (player_weapon as u32) + (match_result as u32);
}


fn player_char_as_result_match_score( player_char: Option<&str>, opponent_weapon: Weapon  ) -> u32 {
    let match_result = match player_char {
        Some("X") => MatchResult::Loss,
        Some("Y") => MatchResult::Draw,
        Some("Z") => MatchResult::Win,
        None => panic!("Empty player weapon"),
        _ => panic!("Invalid player weapon: {}", player_char.unwrap()),
    };

    let player_weapon = if match_result == MatchResult::Draw {
        opponent_weapon
    } else if match_result == MatchResult::Win {
        rps_loses_to(opponent_weapon)
    } else {
        rps_beats(opponent_weapon)
    };

    return (player_weapon as u32) + (match_result as u32);
}


fn main() { 
    let lines: Vec<String> = fs::read_to_string("inputs/02.txt")
                    .unwrap()  
                    .lines()  
                    .map(String::from)  
                    .collect();

    let mut part_one_score = 0;
    let mut part_two_score = 0;
    for line in &lines {
        let mut chars = line.split_whitespace(); 

        let opponent_char = chars.next(); 
        let opponent_weapon = match &opponent_char {
            Some("A") => Weapon::Rock,
            Some("B") => Weapon::Paper,
            Some("C") => Weapon::Scissors,
            None => panic!("Empty opponent weapon"),
            _ => panic!("Invalid opponent weapon: {}", opponent_char.unwrap()),
        };

        let player_char = chars.next();

        part_one_score += player_char_as_weapon_match_score(player_char, opponent_weapon);
        part_two_score += player_char_as_result_match_score(player_char, opponent_weapon);
    }

    println!("Part One Score: {part_one_score}");
    println!("Part Two Score: {part_two_score}");
}
