use std::fs;
use std::time::Instant;

pub fn part2() {
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    let data = fs::read_to_string("./input.txt").unwrap();
    let start = Instant::now();
    for line in data.split('\n') {
        let space_index = line.find(' ').unwrap() + 1;
        let (instruction, movement) = line.split_at(space_index);
        let steps: isize = movement.parse().unwrap();
        match instruction {
            "forward " => {
                forward += steps;
                depth += steps * aim;
            },
            "up " => aim -= steps,
            "down " => aim += steps,
            _ => unreachable!()
        }
    }
    let elapsed = start.elapsed().as_micros();
    println!("Output is: {}\nTook {} us", forward * depth, elapsed);
    
}