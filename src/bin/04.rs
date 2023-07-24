use std::fs;

fn main() {
    let parsed_values: Vec<(u8, u8, u8, u8)> = fs::read_to_string("inputs/04.txt")
                    .unwrap()  
                    .lines()  
                    .map(|line| {
                        let (l, r) = line.split_once(",").unwrap();
                        let ((a,b),(c,d)) = (l.split_once("-").unwrap(), r.split_once("-").unwrap());
                        (
                            a.parse::<u8>().unwrap(),
                            b.parse::<u8>().unwrap(),
                            c.parse::<u8>().unwrap(),
                            d.parse::<u8>().unwrap(),
                        )
                    }).collect();
    
    let part_one_result = parsed_values.iter().filter(|(a,b,c,d)| (a <= c && b >= d) || (c <= a && d >= b))
                                       .count();

    let part_two_result = parsed_values.iter().filter(|(a,b,c,d)| (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b))
                                       .count();

    println!("Part One Result: {part_one_result}");
    println!("Part Two Result: {part_two_result}");
}
