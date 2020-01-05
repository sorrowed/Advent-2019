use math;
use std::collections::HashMap;

#[derive(Clone)]
struct Chemical {
	name: String,
	amount: i32,
}

impl Chemical {
	/*
	7 A
	*/
	pub fn parse(input: &str) -> Chemical {
		let tokens = input.split(" ").collect::<Vec<_>>();

		Chemical {
			name: tokens[1].trim().to_string(),
			amount: tokens[0]
				.trim()
				.parse::<i32>()
				.expect("Chemical parse error"),
		}
	}
}

impl std::fmt::Display for Chemical {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}({})", self.name, self.amount)
	}
}

#[derive(Clone)]
struct Reaction {
	target_chemical: Chemical,
	source_chemicals: Vec<Chemical>,
}

impl std::fmt::Display for Reaction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}\t<-\t", self.target_chemical).unwrap();
		for source in &self.source_chemicals {
			write!(f, "{} ", source).unwrap();
		}
		std::fmt::Result::Ok(())
	}
}

impl Reaction {
	/*
	7 A, 1 B => 1 C\n
	*/
	pub fn parse(input: &str) -> Reaction {
		let tokens = input.split("=>").collect::<Vec<_>>();

		Reaction {
			target_chemical: Chemical::parse(tokens[1].trim()),
			source_chemicals: tokens[0]
				.split(',')
				.map(|t| Chemical::parse(t.trim()))
				.collect(),
		}
	}

	pub fn more(&mut self) {
		self.target_chemical.amount *= 2;
		for source in &mut self.source_chemicals {
			source.amount *= 2;
		}
	}
}

type ChemicalStorage = HashMap<String, i32>;

type ReactionStorage = HashMap<String, Reaction>;

fn parse_reactions(input: &str) -> ReactionStorage {
	input
		.split('\n')
		.map(|token| {
			let r = Reaction::parse(token);
			(r.target_chemical.name.clone(), r)
		})
		.collect::<ReactionStorage>()
}

fn storage_add_chemical(chemicals: &mut ChemicalStorage, name: &String, amount: i32) {
	if chemicals.contains_key(name) {
		chemicals.insert(name.to_string(), amount + chemicals[name]);
	} else {
		chemicals.insert(name.to_string(), amount);
	}
}

fn process_reactions(stored_chemicals: &ChemicalStorage,reactions: &ReactionStorage) -> ChemicalStorage {
	let mut result = ChemicalStorage::new();

	let mut reacted = false;

	for stored_chemical in stored_chemicals {
		let mut amount = stored_chemical.1.clone();

		if reacted {
			storage_add_chemical(&mut result, stored_chemical.0, amount);
		} else {
			// If there is a reaction for this target chemical
			if let Some(reaction) = reactions.get(stored_chemical.0) {

				// If there is enough chemical to reverse, then transform and keep the rest
				if amount >= reaction.target_chemical.amount {
					println!("Performing reverse {}", reaction);

					for source_chemical in &reaction.source_chemicals {
						println!("Producing {}", source_chemical);

						storage_add_chemical(&mut result, &source_chemical.name, source_chemical.amount);
					}

					amount -= reaction.target_chemical.amount;
					if amount > 0 {
						println!("Still {} of {} left", amount, reaction.target_chemical.name);

						storage_add_chemical(&mut result, &reaction.target_chemical.name, amount);
					}

					reacted = true;

				} else {
					storage_add_chemical(&mut result, stored_chemical.0, amount);
				}
			} else {
				storage_add_chemical(&mut result, stored_chemical.0, amount);
			}
		}
	}

	result
}

pub fn test() {
	let input = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";

	let reactions = parse_reactions(input);

	assert_eq!(reactions.len(), 6);
	// for reaction in reactions.values() {
	// 	println!("{}", reaction);
	// }

	let mut stored_chemicals = ChemicalStorage::new();
	stored_chemicals.insert("FUEL".to_string(), 1);

	while stored_chemicals.iter().any(|c| c.0 != "ORE") {
		stored_chemicals = process_reactions(&stored_chemicals, &reactions);

		print!("Still ");
		for chemical in &stored_chemicals {
			print!("{}:{} ", chemical.1, chemical.0);
		}
		println!("in storage");
	}
}

pub fn part1() {}

pub fn part2() {}
