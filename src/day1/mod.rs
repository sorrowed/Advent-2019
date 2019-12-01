use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_required(mass: i32) -> i32 {
    (mass / 3) as i32 - 2
}

fn fuel_required_including_fuel(mass: i32) -> i32 {
    let fuel = fuel_required(mass);

    if fuel <= 0 {
        mass
    } else {
        mass + fuel_required_including_fuel(fuel)
    }
}

fn import(name: &str) -> Vec<i32> {
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        vec.push(line.unwrap().parse::<i32>().unwrap())
    }
    return vec;
}

pub fn part1() {
    let input = import("../../src/day1/input.txt");
    let mut sum = 0;
    for mass in input {
        sum += fuel_required(mass);
    }
    println!("Day 1 part 1 : Total fuel required is {} ", sum);
}

pub fn part2() {
    let input = import("../../src/day1/input.txt");
    let mut sum = 0;
    for mass in input {
        sum += fuel_required_including_fuel(fuel_required(mass));
    }
    println!(
        "Day 1 part 2 : Total fuel required (including fuel) is {}",
        sum
    );
}
