use std::collections::HashMap;

fn input() -> &'static str {
	""
}

pub fn test() {
	let source = "B)C,C)D,D)E,E)F,B)G,G)H,D)I,E)J,J)K,K)L";

	let mut orbits = source
		.split(",")
		.map(|t| (t[0..=0].to_string(), t[2..=2].to_string()))
		.collect::<HashMap<_, _>>();

	// Add the COM
	orbits.insert("COM".to_string(), "B".to_string());
}

pub fn part1() {}

pub fn part2() {}
