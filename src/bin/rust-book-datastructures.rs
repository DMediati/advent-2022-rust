use std::{env, collections::HashMap};


fn median_and_mode(mut input: Vec<i32>) -> (f32, Vec<i32>) {
    let n = input.len();
    assert!(n > 0, "Input length must be greater than 0!");

    input.sort_unstable();
    println!("{:?}", &input);

    let median: f32;
    let mut mode: Vec<i32> = Vec::new();

    if n % 2 == 0 {
       median = ( ( input[n/2] + input[n/2 - 1] ) as f32 ) / 2.0;
    } else {
       median = input[n/2] as f32; 
    }

    let mut map: HashMap<i32,i32> = HashMap::new();
    for i in input.iter() {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max_count: i32 = 0;
    for (k,v) in map.iter() {
        if v > &max_count {
            mode.clear();
            max_count = *v;
            mode.push(*k);
        } else if v == &max_count {
            mode.push(*k); 
        }
    }

    return (median, mode);
}


fn pig_latin(input: &str) -> String {
    let mut output: String = String::from("");
    const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
    for word in input.split_whitespace() {
        if !output.is_empty() {
            output += " ";
        }

        if word.is_empty() {
            continue;
        }

        let first_char = word.chars().next().unwrap();
        let translated_word: String = if VOWELS.contains(&first_char) {
            format!("{}{}", word ,"-hay")
        } else {
           let remaining_word: String = word.chars().skip(1).take(word.len()).collect();
           format!("{}-{}ay", remaining_word, &first_char)
        };
        output += &translated_word;
    }

    return output;
}


fn run_type_interface() {
    println!("Welcome to the Rust Employee Typing Interface!");

}


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    println!("\n------------------------------------------\n");

    if &args[1] == "m" {
        println!("Running Median/Mode Code on Input:");
        let input = &args[2];
        let input: Vec<i32> = input.split(",").map( |s| s.parse::<i32>().unwrap() ).collect();
        println!("{:?}", &input);
        let (median, mode) = median_and_mode(input);
        println!("Median: {median}, Mode: {:?}", mode);
    } else if &args[1] == "p" {
        println!("Pig-Latin:");
        let input = &args[2];
        println!("Original : {:?}", &input);
        let output = pig_latin(input);
        println!("Pig-Latin: {}", output);
    } else if &args[1] == "t" {
        run_type_interface();
    }
}
