use std::fs::File;
use std::io::{BufReader, BufRead};

fn part_one() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut sum = 0;

    for line in buffered.lines() {
        let rucksack = line.unwrap();
        let size = rucksack.len();
        let comp1 = &rucksack[..size/2];
        let comp2 = &rucksack[size/2..];

        for c in comp1.chars() {
            match comp2.find(c) {
                Some(_) => {
                    if c as i32 > 96 {
                        sum += c as i32 - 96;
                    } else {
                        sum += c as i32 - 64 + 26;
                    }
                    break;
                },
                _ => sum += 0,
            }
        }
    }
    sum
}

fn part_two() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut sum = 0;

    let mut iter = buffered.lines();
    while let Some(rucksack1) = iter.next() {
        let rucksack2 = iter.next().unwrap().unwrap();
        let rucksack3 = iter.next().unwrap().unwrap();

        for c in rucksack1.unwrap().chars() {
            if rucksack2.find(c).is_some() && rucksack3.find(c).is_some() {
                if c as i32 > 96 {
                    sum += c as i32 - 96;
                } else {
                    sum += c as i32 - 64 + 26;
                }
                break;
            }
        }
    }
    sum
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
