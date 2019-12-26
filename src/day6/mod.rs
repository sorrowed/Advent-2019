use std::fs;

struct Orbit {
	center: String,
	sattelite: String,
}

struct Body {
	name: String,
	parent: Option<usize>,
	children: Vec<usize>,
}

impl std::fmt::Display for Body {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{},{}", self.name, self.children.len())
	}
}

pub fn import(name: &str) -> String {
	fs::read_to_string(name).expect("Error parsing file")
}

/*
 * This counts all orbits that the given body adds (incuding itself)
 */
fn count_orbits_recursive(body: &Body, bodies: &Vec<Body>) -> i32 {
	let mut current = 0;
	for child_index in &body.children {
		current += count_orbits_recursive(&bodies[child_index.clone()], bodies);
	}
	current + 1
}

fn build_bodies(input: &str) -> Vec<Body> {
	let orbits = input
		.lines()
		.map(|t| t.split(")").collect::<Vec<&str>>())
		.map(|t| Orbit {
			center: t[0].to_string(),
			sattelite: t[1].to_string(),
		})
		.collect::<Vec<Orbit>>();

	// Assemble all unique bodies
	let mut bodies = Vec::<Body>::new();
	for orbit in &orbits {
		let body = Body {
			name: orbit.sattelite.clone(),
			parent: None,
			children: Vec::<usize>::new(),
		};

		bodies.push(body);
	}

	// For each orbit find the sattelite and try to find the parent
	for orbit in &orbits {
		let sattelite_index = bodies
			.iter()
			.position(|b| b.name == orbit.sattelite)
			.unwrap();

		// If there is a parent, fill it as parent an also add the sattelite as child to the parent
		match bodies.iter().position(|b| b.name == orbit.center) {
			Some(m) => {
				bodies[sattelite_index].parent = Some(m);
				bodies[m].children.push(sattelite_index);
			}
			_ => {}
		}
	}
	bodies
}

pub fn test() {
	let source = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";

	let bodies = build_bodies(source);
	// for body in &bodies {
	// 	println!("{}", &body);
	// }

	let mut total_orbits = 0;
	for body in &bodies {
		total_orbits += count_orbits_recursive(body, &bodies);
	}
	println!("{}", total_orbits)
}

pub fn part1() {
	let bodies = build_bodies(&import("src/day6/input.txt"));

	let mut total_orbits = 0;
	for body in &bodies {
		total_orbits += count_orbits_recursive(body, &bodies);
	}
	println!("{}", total_orbits)
}

pub fn part2() {
	let source = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";

	let bodies = build_bodies(source); // &import("src/day6/input.txt")

	let san_index = bodies
		.iter()
		.position(|b| b.name == "SAN")
		.expect("No SAN found");
	let you_index = bodies
		.iter()
		.position(|b| b.name == "YOU")
		.expect("No YOU found");

	let mut san = &bodies[san_index];
	san = &bodies[san.parent.unwrap()];
	print!("SAN: {}",san);

	let mut you = &bodies[you_index];
	you = &bodies[you.parent.unwrap()];
	print!("YOU: {}",you);

	// Collect from SAN to COM
	let mut san_transfers = Vec::<usize>::new();
	while let Some(m) = san.parent {
		san_transfers.push(m);
		san = &bodies[san.parent.unwrap()];
	}

	// Collect from SAN to COM
	let mut you_transfers = Vec::<usize>::new();
	while let Some(m) = you.parent {
		you_transfers.push(m);
		you = &bodies[you.parent.unwrap()];
	}

	// Traverse the transfers SAN made and search for the first common body with YOU
	for (san_index, san_location) in san_transfers.iter().enumerate() {
		match you_transfers.iter().position(|l| l == san_location) {
			Some(you_index) => {
				println!("Total transfers {}", san_transfers.len() - san_index + you_transfers.len() - you_index);
				break;
			}
			_ => {}
		}
	}
}
