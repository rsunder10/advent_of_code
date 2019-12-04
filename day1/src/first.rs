use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn first() {
    let reader = BufReader::new(File::open("input.txt").expect("Cannot open file.txt"));
    let mut total_fuel: i64 = 0;
    for line in reader.lines() {
        let mass: i64 = line.unwrap().parse().unwrap();
        total_fuel = total_fuel + (mass / 3) - 2;
    }
    println!("first {}", total_fuel)
}
