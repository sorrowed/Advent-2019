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
	let mut io = Moon::new(MoonId::IO, -4, 3, 15);
	let mut europa = Moon::new(MoonId::EUROPA, -11, -10, 13);
	let mut ganymede = Moon::new(MoonId::GANYMEDE, 2, 2, 18);
	let mut callisto = Moon::new(MoonId::CALLISTO, 7, -1, 0);

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
pub fn part2() {}
