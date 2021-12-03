use std::fs;
use std::time::Instant;

pub fn part1() {
    let s = fs::read_to_string("./input.txt").unwrap();
    let mut last = 0;
    let mut count: usize = 0;
    let start = Instant::now();
    for line in s.split("\n") {
        if line.len() != 0 {
            let num: usize = line.parse().unwrap();
            if num > last {
                count += 1;
            }
            last = num;
        }
    }
    let elapsed = start.elapsed().as_micros();
    println!("Output is: {}\nTook {} us", count - 1, elapsed);
}