use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn second() {
    let reader = BufReader::new(File::open("input2.txt").expect("Cannot open file.txt"));
    let mut total_fuel: i64 = 0;
    for line in reader.lines() {
        let mass: i64 = line.unwrap().parse().unwrap();
        total_fuel = total_fuel + get_fuel(mass);
    }

    println!("second {}", total_fuel)
}

fn get_fuel(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        return fuel + get_fuel(fuel);
    }
    return 0;
}
