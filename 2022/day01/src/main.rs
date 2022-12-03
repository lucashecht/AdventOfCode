use std::fs::File;
use std::io::{BufReader, BufRead};

fn part_one() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut max = 0;
    let mut sum = 0;

    for line in buffered.lines() {
        if line.as_ref().unwrap().is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.unwrap().parse::<i32>().unwrap();
        }
    }

    max
}

fn part_two() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut sums = Vec::new();
    let mut sum = 0;

    for line in buffered.lines() {
        if line.as_ref().unwrap().is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.unwrap().parse::<i32>().unwrap();
        }
    }

    sums.sort();
    sums.reverse();
    sums[..3].iter().sum()
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
