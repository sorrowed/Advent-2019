use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::common::*;
use crate::cpu::*;

use std::{thread, time};

type MapType = HashMap<Vector, Tile>;

#[derive(PartialEq)]
enum TileType {
	EMPTY = 0,
	WALL = 1,
	BLOCK = 2,
	HPADDLE = 3,
	BALL = 4,
}

impl TileType {
	pub fn new(tile_type: i32) -> Option<TileType> {
		match tile_type {
			0 => Some(TileType::EMPTY),
			1 => Some(TileType::WALL),
			2 => Some(TileType::BLOCK),
			3 => Some(TileType::HPADDLE),
			4 => Some(TileType::BALL),
			_ => None,
		}
	}
}

impl std::fmt::Display for TileType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				TileType::EMPTY => " ",
				TileType::WALL => "#",
				TileType::BLOCK => "*",
				TileType::HPADDLE => "_",
				TileType::BALL => "o",
			}
		)
	}
}

struct Tile {
	location: Vector,
	tile_type: TileType,
}

impl std::fmt::Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} -> {} ", self.location, self.tile_type)
	}
}

impl Tile {
	pub fn new(x: i32, y: i32, tile_type: TileType) -> Tile {
		Tile {
			location: Vector { x: x, y: y, z: 0 },
			tile_type: tile_type,
		}
	}

	pub fn is_score_chunk(chunk: &[CpuWidth]) -> bool {
		chunk[0] == -1
	}

	pub fn read(input: &Vec<CpuWidth>) -> HashMap<Vector, Tile> {
		input
			.chunks(3)
			.filter(|chunk| !Tile::is_score_chunk(chunk))
			.map(|chunk| {
				(
					Vector::new(chunk[0] as i32, chunk[1] as i32, 0),
					Tile::new(
						chunk[0] as i32,
						chunk[1] as i32,
						TileType::new(chunk[2] as i32).expect("Error parsing tile type"),
					),
				)
			})
			.collect()
	}
}

struct Map {
	world: MapType,
	score: CpuWidth,

	paddle: Option<Vector>,
	ball: Option<Vector>,
}

impl Map {
	pub fn new() -> Map {
		Map {
			world: MapType::new(),
			score: 0,
			paddle: None,
			ball: None,
		}
	}

	pub fn extends(&self) -> (Vector, Vector) {
		let x_min = self.world.keys().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
		let x_max = self.world.keys().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
		let y_min = self.world.keys().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
		let y_max = self.world.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

		(
			Vector {
				x: x_min,
				y: y_min,
				z: 0,
			},
			Vector {
				x: x_max,
				y: y_max,
				z: 0,
			},
		)
	}

	fn extract_score(&mut self, input: &Vec<CpuWidth>) {
		if let Some(score_chunk) = input.chunks(3).find(|chunk| Tile::is_score_chunk(chunk)) {
			self.score = score_chunk[2];
		}
	}

	pub fn update(&mut self, input: &Vec<CpuWidth>) {
		self.extract_score(input);

		let tiles = Tile::read(input);

		for tile in tiles {
			if tile.1.tile_type == TileType::BALL {
				self.ball = Some(tile.0.clone());
			} else if tile.1.tile_type == TileType::HPADDLE {
				self.paddle = Some(tile.0.clone());
			}

			self.world.insert(tile.0, tile.1);
		}
	}
}

impl std::fmt::Display for Map {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let extends = self.extends();
		for y in extends.0.y..=extends.1.y {
			for x in extends.0.x..=extends.1.x {
				write!(f, "{}", self.world[&Vector::new(x, y, 0)].tile_type).expect("IO error");
			}
			writeln!(f).expect("IO error");
		}
		std::fmt::Result::Ok(())
	}
}

pub fn test() {}

pub fn part1() {
	let input = import_lines("src/day13/input.txt");

	let mut program = read(&input);
	execute(&mut program);

	let mut map = Map::new();
	map.world = Tile::read(&program.output);

	println!(
		"Number of block tiles on the screen is {}",
		map
			.world
			.values()
			.filter(|&tile| tile.tile_type == TileType::BLOCK)
			.count()
	);
}

pub fn part2() {
	let input = import_lines("src/day13/input.txt");

	let mut program = read(&input);
	program.set(0, 2); // Free play

	let mut map = Map::new();

	print!("\x1B[2J");

	// Now play until finished
	while !program.is_finished() {
		execute(&mut program);

		// Update the map with output of this program cycle and then clear the program input and output
		map.update(&program.output);
		program.flush();

		// Print the map, because its fun
		println!("\x1B[1;1H{}", map);

		if program.is_waiting() {
			// Make sure bat moves towards ball by providing joystick input (nput is cleared at this point)
			let ball = map.ball.as_ref().expect("Ball not found on map");
			let paddle = map.paddle.as_ref().expect("Paddle not found on map");

			if ball.x < paddle.x {
				program.add_input(-1);
			} else if ball.x > paddle.x {
				program.add_input(1);
			} else {
				program.add_input(0);
			}
		}

		std::thread::sleep(time::Duration::from_millis(10));
	}

	// Update final state of map and score
	map.update(&program.output);
	println!("\x1B[1;1H{}", map);

	println!("Final score is {}", map.score);
}
