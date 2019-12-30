use std::fmt;

macro_rules! vec_of_strings {
	($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn input() -> Vec<String> {
	vec_of_strings![
		"..............#.#...............#....#....",
		"#.##.......#....#.#..##........#...#......",
		"..#.....#....#..#.#....#.....#.#.##..#..#.",
		"...........##...#...##....#.#.#....#.##..#",
		"....##....#...........#..#....#......#.###",
		".#...#......#.#.#.#...#....#.##.##......##",
		"#.##....#.....#.....#...####........###...",
		".####....#.......#...##..#..#......#...#..",
		"...............#...........#..#.#.#.......",
		"........#.........##...#..........#..##...",
		"...#..................#....#....##..#.....",
		".............#..#.#.........#........#.##.",
		"...#.#....................##..##..........",
		".....#.#...##..............#...........#..",
		"......#..###.#........#.....#.##.#......#.",
		"#......#.#.....#...........##.#.....#..#.#",
		".#.............#..#.....##.....###..#..#..",
		".#...#.....#.....##.#......##....##....#..",
		".........#.#..##............#..#...#......",
		"..#..##...#.#..#....#..#.#.......#.##.....",
		"#.......#.#....#.#..##.#...#.......#..###.",
		".#..........#...##.#....#...#.#.........#.",
		"..#.#.......##..#.##..#.......#.###.......",
		"...#....###...#......#..#.....####........",
		".............#.#..........#....#......#...",
		"#................#..................#.###.",
		"..###.........##...##..##.................",
		".#.........#.#####..#...##....#...##......",
		"........#.#...#......#.................##.",
		".##.....#..##.##.#....#....#......#.#....#",
		".....#...........#.............#.....#....",
		"........#.##.#...#.###.###....#.#......#..",
		"..#...#.......###..#...#.##.....###.....#.",
		"....#.....#..#.....#...#......###...###...",
		"#..##.###...##.....#.....#....#...###..#..",
		"........######.#...............#...#.#...#",
		"...#.....####.##.....##...##..............",
		"###..#......#...............#......#...#..",
		"#..#...#.#........#.#.#...#..#....#.#.####",
		"#..#...#..........##.#.....##........#.#..",
		"........#....#..###..##....#.#.......##..#",
		".................##............#.......#.."
	]
}

#[derive(Clone)]
struct Coordinate {
	pub x: i32,
	pub y: i32,
}

impl std::fmt::Display for Coordinate {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "({},{})", self.x, self.y)
	}
}

impl std::cmp::PartialEq for Coordinate {
	fn eq(&self, other: &Coordinate) -> bool {
		return self.x == other.x && self.y == other.y;
	}
}

impl Coordinate {
	pub fn path(&self, other: &Coordinate) -> Path {
		let x = (other.x - self.x) as f32;
		let y = (other.y - self.y) as f32;

		Path {
			angle: x.atan2(y),
			distance: x.hypot(y),
		}
	}
}

#[derive(Clone)]
struct Asteroid {
	pub location: Coordinate,
}

impl std::cmp::PartialEq for Asteroid {
	fn eq(&self, other: &Asteroid) -> bool {
		return self.location == other.location;
	}
}

struct Path {
	pub angle: f32,
	pub distance: f32,
}

// Path is considered equal when they have the same angle
impl std::cmp::PartialEq for Path {
	fn eq(&self, other: &Path) -> bool {
		return self.angle == other.angle;
	}
}

// Path is compared based on distance
impl PartialOrd for Path {
	fn partial_cmp(&self, other: &Path) -> Option<std::cmp::Ordering> {
		self.distance.partial_cmp(&other.distance)
	}
}

fn parse_asteroids(input: &Vec<String>) -> Vec<Asteroid> {
	let mut asteroids = Vec::<Asteroid>::new();

	for y in 0..input.len() {
		let line = &input[y].as_bytes();
		for x in 0..line.len() {
			if line[x] == '#' as u8 {
				asteroids.push(Asteroid {
					location: Coordinate {
						x: x as i32,
						y: y as i32,
					},
				});
			}
		}
	}
	asteroids
}

/*
	This creates a vector of tuples which contains for each location a collection of Paths to other asteroids
	Asteroids that are blocked by other asteroids (same angle, closer by) are not included in the latter collection
*/
fn process_asteroids(asteroids: &Vec<Asteroid>) -> Vec<(Coordinate, Vec<Path>)> {
	let mut result = Vec::<(Coordinate, Vec<Path>)>::new();

	for source in asteroids {
		// Create a collection of Paths for each asteroid
		let mut paths = Vec::<Path>::new();

		for target in asteroids {
			if source != target {
				// Calculate Path to target asteroid
				let path = source.location.path(&target.location);

				// If there is an asteroid in the same path as the target asteroid
				if let Some(index) = paths.iter().position(|v| v == &path) {
					let d = &paths[index];

					// Remove it if it is further away and add this one, else do not include this one
					if d > &path {
						paths.remove(index);
						paths.push(path);
					}
				} else {
					// No target asteroid in same path/angle, add this one
					paths.push(path);
				}
			}
		}

		result.push((source.location.clone(), paths));
	}

	result
}

pub fn test() {
	let source = vec_of_strings![".#..#", ".....", "#####", "....#", "...##"];

	let asteroids_in_view = process_asteroids(&parse_asteroids(&source));

	for v in asteroids_in_view {
		println!("{} {}", v.0, v.1.len());
	}
}

pub fn part1() {
	let mut asteroids_in_view = process_asteroids(&parse_asteroids(&input()));

	// Sort by number of asteroids in view, ascending
	asteroids_in_view.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

	let best_location = asteroids_in_view
		.last()
		.expect("No asteroids found, bollox!");

	println!(
		"Best location {} with {} asteroids in view",
		best_location.0,
		best_location.1.len()
	);
}

pub fn part2() {}
