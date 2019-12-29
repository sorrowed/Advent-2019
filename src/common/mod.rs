use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn import(name: &str) -> Vec<i64> {
	let file = File::open(name).unwrap();
	let reader = BufReader::new(file);
	let mut vec = Vec::new();
	for (_, line) in reader.lines().enumerate() {
		vec.push(line.unwrap().parse::<i64>().unwrap())
	}
	return vec;
}

pub fn import_lines(name: &str) -> String {
	fs::read_to_string(name).expect("Error parsing file")
}
