use std::fs;
use std::time::Instant;

pub fn part1() {
    let mut forward: usize = 0;
    let mut depth: usize = 0;

    let data = fs::read_to_string("./input.txt").unwrap();
    let start = Instant::now();
    for line in data.split('\n') {
        let space_index = line.find(' ').unwrap() + 1;
        let (instruction, movement) = line.split_at(space_index);
        let steps: usize = movement.parse().unwrap();
        match instruction {
            "forward " => forward += steps,
            "up " => depth -= steps,
            "down " => depth += steps,
            _ => unreachable!()
        }
    }
    let elapsed = start.elapsed().as_micros();
    println!("Output is: {}\nTook {} us", forward * depth, elapsed);
    
}