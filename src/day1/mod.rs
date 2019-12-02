use crate::common;

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

pub fn part1() {
	let input = common::import("../../src/day1/input.txt");
	let mut sum = 0;
	for mass in input {
		sum += fuel_required(mass);
	}
	println!("Day 1 part 1 : Total fuel required is {} ", sum);
}

pub fn part2() {
	let input = common::import("../../src/day1/input.txt");
	let mut sum = 0;
	for mass in input {
		sum += fuel_required_including_fuel(fuel_required(mass));
	}
	println!(
		"Day 1 part 2 : Total fuel required (including fuel) is {}",
		sum
	);
}
