use std::fs;
use std::time::Instant;

pub fn part2() {
    let mut array: Vec<usize> = Vec::with_capacity(3);
    let mut previous_sum: usize = 0;
    let s = fs::read_to_string("./input.txt").unwrap();
    let mut count = 0;
    let start = Instant::now();
    for line in s.split('\n') {
        let num: usize = line.parse().unwrap();
        array.push(num);
        let mut sum = 0;
        if array.len() == 3 {
            for indx in 0..3 {
                sum += array[indx];
            }

            if sum > previous_sum {
                count += 1;
            }
            previous_sum = sum;
            array.remove(0);
        }
    }
    let elapsed = start.elapsed().as_micros();
    println!("Output is: {}\nTook {} us", count - 1, elapsed);
}