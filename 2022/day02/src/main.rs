use std::fs::File;
use std::io::{BufReader, BufRead};

fn part_one() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut sum = 0;

    for line in buffered.lines() {
        let round = line.unwrap();
        let mut split = round.split(" ");
        let opponent = split.next().unwrap().to_string(); 
        let player = split.next().unwrap().to_string(); 

        sum += match player.as_str() {
            "X" =>
                // Rock
                match opponent.as_str() {
                    "C" => 6 + 1,
                    "A" => 3 + 1,
                    _ => 1,
                },
            "Y" => 
                // Paper
                match opponent.as_str() {
                    "A" => 6 + 2,
                    "B" => 3 + 2,
                    _ => 2,
                },
            "Z" =>
                // Scissors
                match opponent.as_str() {
                    "B" => 6 + 3,
                    "C" => 3 + 3,
                    _ => 3,
                }
            _ => 0
        };
    }

    sum
}

fn part_two() -> i32 {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let mut sum = 0;

    for line in buffered.lines() {
        let round = line.unwrap();
        let mut split = round.split(" ");
        let opponent = split.next().unwrap().to_string(); 
        let player = split.next().unwrap().to_string(); 

        sum += match player.as_str() {
            "X" =>
                // lose
                match opponent.as_str() {
                    "A" => 3, // scissors
                    "B" => 1,
                    _ => 2,
                },
            "Y" => 
                // draw
                match opponent.as_str() {
                    "A" => 1 + 3,
                    "B" => 2 + 3,
                    _ => 3 + 3,
                },
            "Z" =>
                // win
                match opponent.as_str() {
                    "A" => 2 + 6,
                    "B" => 3 + 6,
                    _ => 1 + 6,
                }
            _ => 0
        };
    }

    sum
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
