use std::cmp::max;
use std::io::{self, BufRead};

fn mass_to_fuel(mass: u64) -> u64 {
    max(mass / 3, 2) - 2
}

fn mass_to_fuel_recursive(mass: u64) -> u64 {
    let fuel = mass_to_fuel(mass);
    if fuel <= 6 {
        return fuel;
    }

    fuel + mass_to_fuel_recursive(fuel)
}

fn part_one() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let sum: u64 = lines
        .map(|res| res.unwrap())
        .map(|line| line.parse::<u64>().unwrap())
        .map(mass_to_fuel)
        .sum();

    println!("\n{}", sum);
}

fn part_two() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let sum: u64 = lines
        .map(|res| res.unwrap())
        .map(|line| line.parse::<u64>().unwrap())
        .map(mass_to_fuel_recursive)
        .sum();

    println!("\n{}", sum);
}

fn main() {
    part_two();
}
