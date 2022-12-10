use std::fs::File;
use std::io::{BufReader, BufRead};

fn part_one() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut sum = 0;

    for line in buffered.lines() {
        let round = line.unwrap();
        let mut split = round.split(',');
        let elf1: Vec<i32> = split.next().unwrap().split('-').map(|s| s.parse().unwrap()).collect();
        let elf2: Vec<i32> = split.next().unwrap().split('-').map(|s| s.parse().unwrap()).collect();

        if (elf1[0] <= elf2[0] && elf1[1] >= elf2[1]) || (elf2[0] <= elf1[0] && elf2[1] >= elf1[1]) {
            sum += 1;
        }
    }

    sum
}

fn part_two() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut sum = 0;

    for line in buffered.lines() {
        let round = line.unwrap();
        let mut split = round.split(',');
        let elf1: Vec<i32> = split.next().unwrap().split('-').map(|s| s.parse().unwrap()).collect();
        let elf2: Vec<i32> = split.next().unwrap().split('-').map(|s| s.parse().unwrap()).collect();

        if elf1[0] <= elf2[0] && elf2[0] <= elf1[1] || elf2[0] <= elf1[0] && elf1[0] <= elf2[1] {
            sum += 1;
        }
    }

    sum
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
