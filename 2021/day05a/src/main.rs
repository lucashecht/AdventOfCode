use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let input = File::open("input.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);
    
    let mut floor = [[0u8; 1000]; 1000];
    let mut dangerous = 0;
    
    for line in buffered.lines() {
        if let [x1, y1, x2, y2] = line
            .unwrap()
            .split(" -> ")
            .map(|i| i.split(",").collect::<Vec<&str>>())
            .flatten()
            .map(|s: &str| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..]{

            let mut x = [x1, x2];
            let mut y = [y1, y2];
            x.sort();
            y.sort();

            // vertical lines 
            if x1==x2 {
                for i in y[0]..y[1]+1 {
                    if floor[i][x1] == 1 {
                        dangerous += 1;
                    }
                    floor[i][x1] += 1;
                }
            // horizontal lines
            } else if y1==y2 {
                for j in x[0]..x[1]+1 {
                    if floor[y1][j] == 1 {
                        dangerous += 1;
                    }
                    floor[y1][j] += 1;
                }
            } else {
                continue;
            }
        }
    }
    println!("{}", dangerous);
}
