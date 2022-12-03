use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);
    let count = buffered.lines()
        .map(|l| l.unwrap()
            .split(" | ")
            .last()
            .unwrap()
            .split_whitespace()
            .filter(|digit| digit.len()<=4 || digit.len()==7)
            .count())
        .sum::<usize>()
        ;

    println!("{}", count);
}
