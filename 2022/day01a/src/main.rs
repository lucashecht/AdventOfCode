use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
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

    println!("{}", max);
}
