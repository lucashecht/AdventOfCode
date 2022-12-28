use std::fs::read_to_string;

const NUM_STACKS: usize = 9;

fn parse_stacks(stack_input: &str) -> Vec<Vec<u8>> {
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); NUM_STACKS];

    let stack_lines = stack_input.lines().rev().map(str::as_bytes);

    for line in stack_lines.skip(1) {
        for i in 0..NUM_STACKS {
            if line[1+4*i] != ' ' as u8 {
                stacks[i].push(line[1+4*i]);
            }
        }
    }

    stacks
}

fn create_answer(stacks: Vec<Vec<u8>>) -> String {
    let mut answer = String::new();
    for mut stack in stacks {
        answer.push(stack.pop().unwrap() as char);
    }

    answer
}

fn part_one() -> String {
    let input = read_to_string("input.txt").expect("Couldn't open input.txt");

    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stack_input);

    for line in instruction_input.lines() {
        let instruction: Vec<&str> = line.split(' ').collect(); 
        let count = instruction[1].parse::<i32>().unwrap();
        let stack_src = instruction[3].parse::<usize>().unwrap();
        let stack_dst = instruction[5].parse::<usize>().unwrap();

        for _ in 0..count {
            match stacks[stack_src-1].pop() {
                Some(x) => stacks[stack_dst-1].push(x),
                None => (),
            }
        }
    }

    create_answer(stacks)
}

fn part_two() -> String {
    let input = read_to_string("input.txt").expect("Couldn't open input.txt");

    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stack_input);

    for line in instruction_input.lines() {
        let instruction: Vec<&str> = line.split(' ').collect(); 
        let count = instruction[1].parse::<usize>().unwrap();
        let stack_src = instruction[3].parse::<usize>().unwrap();
        let stack_dst = instruction[5].parse::<usize>().unwrap();

        let stack_temp = stacks[stack_src-1].clone();
        let crates = &stack_temp[(stack_temp.len()-count)..];
        stacks[stack_src-1].truncate(stack_temp.len()-count);
        stacks[stack_dst-1].extend_from_slice(crates);
    }

    create_answer(stacks)
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
