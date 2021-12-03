use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);

    let measurements: Vec<i32> = buffered.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect();
    println!("{}", measurements.windows(2).filter(|w| w[0] < w[1]).count());
}

