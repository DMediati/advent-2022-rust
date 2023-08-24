use std::fs;


/**
--- Day 5: Supply Stacks ---
The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates,
but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over,
the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, 
and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top.
Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. 
In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 
In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, 
so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
*/

fn main() {
    let lines: Vec<String> = fs::read_to_string("inputs/05.txt")
                    .unwrap()  
                    .lines()  
                    .map(String::from)  
                    .collect();
   
    let (bottom_stack_index, bottom_stack_line) = lines.iter().enumerate().find(|(_, l)| l.starts_with(" 1")).unwrap();    
    let num_stacks = bottom_stack_line.trim().split_whitespace().count();

    println!("{}", num_stacks);
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks { // This is only running once for some reason???
        stacks.push(Vec::new());
    }

    let raw_rev_stack = lines[..bottom_stack_index].iter().rev();

    for line in raw_rev_stack {
        println!("{}", line);
        let mut i = 0;
        line.chars().collect::<Vec<char>>().chunks(4).for_each(|v| {
            if v[1] != ' ' {
                stacks[i].push(v[1]);
            }
            i += 1;
        });
    }

    println!("Start");
    println!("{:?}", stacks);

    lines.iter().skip(bottom_stack_index + 2).for_each(|m| {
        let instruction: Vec<&str> = m.split_whitespace().collect::<Vec<&str>>();
        let amount = instruction[1].parse::<u8>().unwrap();
        let start = instruction[3].parse::<usize>().unwrap() - 1;
        let end = instruction[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let value = stacks.get_mut(start).unwrap().pop().unwrap();
            stacks.get_mut(end).unwrap().push(value);
        }
    });

    println!("End");
    println!("{:?}", stacks);

    println!("Answer:");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }

    println!("");
}

