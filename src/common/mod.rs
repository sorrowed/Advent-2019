use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn import(name: &str) -> Vec<i32> {
	let file = File::open(name).unwrap();
	let reader = BufReader::new(file);
	let mut vec = Vec::new();
	for (_, line) in reader.lines().enumerate() {
		vec.push(line.unwrap().parse::<i32>().unwrap())
	}
	return vec;
}
