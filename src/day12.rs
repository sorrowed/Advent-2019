use crate::common::Vector;

#[derive(PartialEq, Clone)]
enum MoonId {
	IO,
	EUROPA,
	GANYMEDE,
	CALLISTO,
}

#[derive(Clone)]
struct Moon {
	pub id: MoonId,
	pub location: Vector,
	pub velocity: Vector,
}

impl Moon {
	pub fn new(id: MoonId, x: i32, y: i32, z: i32) -> Moon {
		Moon {
			id: id,
			location: Vector { x: x, y: y, z: z },
			velocity: Vector { x: 0, y: 0, z: 0 },
		}
	}

	pub fn energy(&self) -> i32 {
		self.location.magnitude() * self.velocity.magnitude()
	}

	pub fn gravity(this: &mut Moon, that: &mut Moon) {
		gravity_once(
			&mut this.velocity.x,
			&mut that.velocity.x,
			this.location.x,
			that.location.x,
		);
		gravity_once(
			&mut this.velocity.y,
			&mut that.velocity.y,
			this.location.y,
			that.location.y,
		);
		gravity_once(
			&mut this.velocity.z,
			&mut that.velocity.z,
			this.location.z,
			that.location.z,
		);
	}

	pub fn step(&mut self) {
		self.location.x += self.velocity.x;
		self.location.y += self.velocity.y;
		self.location.z += self.velocity.z;
	}
}

fn gravity_once(velocity_a: &mut i32, velocity_b: &mut i32, pos_a: i32, pos_b: i32) {
	if pos_a > pos_b {
		*velocity_a -= 1;
		*velocity_b += 1;
	} else if pos_b > pos_a {
		*velocity_a += 1;
		*velocity_b -= 1;
	}
}

fn simulate(
	io: &mut Moon,
	europa: &mut Moon,
	ganymede: &mut Moon,
	callisto: &mut Moon,
	steps: i32,
) {
	for _ in 0..steps {
		Moon::gravity(io, europa);
		Moon::gravity(io, ganymede);
		Moon::gravity(io, callisto);
		Moon::gravity(europa, ganymede);
		Moon::gravity(europa, callisto);
		Moon::gravity(ganymede, callisto);

		io.step();
		europa.step();
		ganymede.step();
		callisto.step();
	}
}

/*
	Find greatest common divisor of a and busing Euclid's algorithm
*/
fn gcd(a: i64, b: i64) -> i64 {
	let (mut x, mut y) = if a > b { (a, b) } else { (b, a) };

	let mut r = x % y;
	while r != 0 {
		x = y;
		y = r;
		r = x % y;
	}

	y
}

/*
	Find least common multiple of a and b
*/
fn lcm(a: i64, b: i64) -> i64 {
	(a * b) / gcd(a, b)
}

fn moons() -> (Moon, Moon, Moon, Moon) {
	(
		Moon::new(MoonId::IO, -4, 3, 15),
		Moon::new(MoonId::EUROPA, -11, -10, 13),
		Moon::new(MoonId::GANYMEDE, 2, 2, 18),
		Moon::new(MoonId::CALLISTO, 7, -1, 0),
	)
}


pub fn test() {
	let mut io = Moon::new(MoonId::IO, -1, 0, 2);
	let mut europa = Moon::new(MoonId::EUROPA, 2, -10, -7);
	let mut ganymede = Moon::new(MoonId::GANYMEDE, 4, -8, 8);
	let mut callisto = Moon::new(MoonId::CALLISTO, 3, 5, -1);

	simulate(&mut io, &mut europa, &mut ganymede, &mut callisto, 10);

	println!("Energy for Io: {}", io.energy());
	println!("Energy for Europa: {}", europa.energy());
	println!("Energy for Ganymede: {}", ganymede.energy());
	println!("Energy for Callisto: {}", callisto.energy());
	println!(
		"Total energy for all moons: {}",
		io.energy() + europa.energy() + ganymede.energy() + callisto.energy()
	);
}

pub fn part1() {
	let (mut io, mut europa, mut ganymede, mut callisto) = moons();

	simulate(&mut io, &mut europa, &mut ganymede, &mut callisto, 1000);

	println!("Energy for Io: {}", io.energy());
	println!("Energy for Europa: {}", europa.energy());
	println!("Energy for Ganymede: {}", ganymede.energy());
	println!("Energy for Callisto: {}", callisto.energy());
	println!(
		"Total energy for all moons: {}",
		io.energy() + europa.energy() + ganymede.energy() + callisto.energy()
	);
}

pub fn part2() {
	// let initial_state = (
	// 	Moon::new(MoonId::IO, -1, 0, 2),
	// 	Moon::new(MoonId::EUROPA, 2, -10, -7),
	// 	Moon::new(MoonId::GANYMEDE, 4, -8, 8),
	// 	Moon::new(MoonId::CALLISTO, 3, 5, -1),
	// );

	// let initial_state = (
	// 	Moon::new(MoonId::IO, -8, -10, 0),
	// 	Moon::new(MoonId::EUROPA, 5, 5, 10),
	// 	Moon::new(MoonId::GANYMEDE, 2, -7, 3),
	// 	Moon::new(MoonId::CALLISTO, 9, -8, -3),
	// );

	let initial_state = moons();

	let x_initial = vec![
		(initial_state.0.location.x, initial_state.0.velocity.x),
		(initial_state.1.location.x, initial_state.1.velocity.x),
		(initial_state.2.location.x, initial_state.2.velocity.x),
		(initial_state.3.location.x, initial_state.3.velocity.x),
	];

	let y_initial = vec![
		(initial_state.0.location.y, initial_state.0.velocity.y),
		(initial_state.1.location.y, initial_state.1.velocity.y),
		(initial_state.2.location.y, initial_state.2.velocity.y),
		(initial_state.3.location.y, initial_state.3.velocity.y),
	];

	let z_initial = vec![
		(initial_state.0.location.z, initial_state.0.velocity.z),
		(initial_state.1.location.z, initial_state.1.velocity.z),
		(initial_state.2.location.z, initial_state.2.velocity.z),
		(initial_state.3.location.z, initial_state.3.velocity.z),
	];

	let mut moons = initial_state.clone();
	let mut periods = Vector::new(0, 0, 0);
	let mut generation = 0;
	while periods.x == 0 || periods.y == 0 || periods.z == 0 {
		simulate(&mut moons.0, &mut moons.1, &mut moons.2, &mut moons.3, 1);
		generation += 1;

		if periods.x == 0 {
			let x_components = [&moons.0, &moons.1, &moons.2, &moons.3]
				.iter()
				.map(|m| (m.location.x, m.velocity.x))
				.collect::<Vec<(i32, i32)>>();

			if x_components == x_initial {
				periods.x = generation;
			}
		}
		if periods.y == 0 {
			let y_components = [&moons.0, &moons.1, &moons.2, &moons.3]
				.iter()
				.map(|m| (m.location.y, m.velocity.y))
				.collect::<Vec<(i32, i32)>>();

			if y_components == y_initial {
				periods.y = generation;
			}
		}
		if periods.z == 0 {
			let z_components = [&moons.0, &moons.1, &moons.2, &moons.3]
				.iter()
				.map(|m| (m.location.z, m.velocity.z))
				.collect::<Vec<(i32, i32)>>();

			if z_components == z_initial {
				periods.z = generation;
			}
		}
	}

	println!(
		"Repeating universe history after {} generations",
		lcm(lcm(periods.x.into(), periods.y.into()), periods.z.into())
	);
}
