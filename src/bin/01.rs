use std::fs;

fn main() {
    let lines: Vec<String> = fs::read_to_string("inputs/01.txt")
                    .unwrap()  
                    .lines()  
                    .map(String::from)  
                    .collect();

    let mut maxes = [-1; 3];
    let mut index_of_smallest = 0;
    let mut amount: i32 = 0; 
    for line in &lines {
        if line == "" {
            if amount > maxes[index_of_smallest] {
                maxes[index_of_smallest] = amount;
            }
            amount = 0;
            for (i, max) in maxes.iter().enumerate() {
                if max <= &maxes[index_of_smallest] { 
                    index_of_smallest = i;
                }
            }
        } else {
            amount = amount + line.parse::<i32>().unwrap();
        }
    }
    println!("{:?} -> Sum: {}", maxes, maxes.iter().sum::<i32>());
}
