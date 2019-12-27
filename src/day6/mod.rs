use crate::common::import_lines;

struct Orbit {
	center: String,
	sattelite: String,
}

struct Body {
	name: String,
	orbiting: Option<usize>,
	sattelites: Vec<usize>,
}

impl std::fmt::Display for Body {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{},{}", self.name, self.sattelites.len())
	}
}

/*
 * This counts all orbits that the given body adds (incuding itself!)
 */
fn count_orbits_recursive(body: &Body, bodies: &Vec<Body>) -> i32 {
	let mut orbits = 0;
	for child_index in &body.sattelites {
		orbits += count_orbits_recursive(&bodies[child_index.clone()], bodies);
	}
	orbits + 1
}

fn build_body_tree(input: &str) -> Vec<Body> {
	// Parse the input into an Orbit collection
	let orbits = input
		.lines()
		.map(|t| t.split(")").collect::<Vec<&str>>())
		.map(|t| Orbit {
			center: t[0].to_string(),
			sattelite: t[1].to_string(),
		})
		.collect::<Vec<Orbit>>();

	// Collect all unique bodies in a vector
	let mut bodies = orbits
		.iter()
		.map(|orbit| Body {
			name: orbit.sattelite.clone(),
			orbiting: None,
			sattelites: Vec::<usize>::new(),
		})
		.collect::<Vec<Body>>();

	// Build the tree index by finding the sattelite for each orbit and then try to locate the body it orbits
	for orbit in &orbits {
		let sattelite_index = bodies
			.iter()
			.position(|body| body.name == orbit.sattelite)
			.expect("Sattelite not found in bodies, something is fishy...");

		// If there is a center body, make this body orbiting that body and add the body as sattelite to that center
		if let Some(m) = bodies.iter().position(|body| body.name == orbit.center) {
			bodies[sattelite_index].orbiting = Some(m);
			bodies[m].sattelites.push(sattelite_index);
		}
	}
	bodies
}

pub fn test() {
	let source = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";

	let bodies = build_body_tree(source);

	let mut total_orbits = 0;
	for body in &bodies {
		total_orbits += count_orbits_recursive(body, &bodies);
	}
	println!("Day6 test has {} total orbits", total_orbits)
}

pub fn part1() {
	let bodies = build_body_tree(&import_lines("src/day6/input.txt"));

	let mut total_orbits = 0;
	for body in &bodies {
		total_orbits += count_orbits_recursive(body, &bodies);
	}
	println!("Day6 part 1 has {} total orbits", total_orbits)
}

pub fn part2() {
	// let source = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
	// let bodies = build_bodies(source);
	let bodies = build_body_tree(&import_lines("src/day6/input.txt"));

	let san_index = bodies
		.iter()
		.position(|body| body.name == "SAN")
		.expect("No SAN found");
	let you_index = bodies
		.iter()
		.position(|body| body.name == "YOU")
		.expect("No YOU found");

	// Collect from SAN to COM, start at parent of SAN
	let mut san = &bodies[(&bodies[san_index])
		.orbiting
		.expect("Body has no parent, badness")];
	
	let mut san_transfers = Vec::<usize>::new();
	while let Some(m) = san.orbiting {
		san_transfers.push(m);
		san = &bodies[san.orbiting.expect("Body has no parent, badness")];
	}

	// Collect from YOU to COM, start at parent of YOU
	let mut you = &bodies[(&bodies[you_index])
		.orbiting
		.expect("Body has no parent, badness")];

	let mut you_transfers = Vec::<usize>::new();
	while let Some(m) = you.orbiting {
		you_transfers.push(m);
		you = &bodies[you.orbiting.expect("Body has no parent, badness")];
	}

	// Traverse the transfers SAN made and search for the first common body with YOU
	for (san_index, san_location) in san_transfers.iter().enumerate() {
		if let Some(you_index) = you_transfers.iter().position(|l| l == san_location) {
			// As this is the first common position for SAN and YOU, both movements combined brings YOU to SAN.
			// Were are interested in the number of moves so need to add 1 for each of san_index and you_index
			println!(
				"Day6 part 2 has {} transfers from YOU to SAN",
				san_index + 1 + you_index + 1
			);
			break;
		}
	}
}
