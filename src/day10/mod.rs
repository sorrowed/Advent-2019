use std::fmt;

use crate::common::*;

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

struct Asteroid {
    pub location: Vector,
}

impl Asteroid {
    pub fn new(location: Vector) -> Asteroid {
        Asteroid { location: location }
    }
}

impl std::cmp::PartialEq for Asteroid {
    fn eq(&self, other: &Asteroid) -> bool {
        return self.location == other.location;
    }
}

fn parse_asteroids(input: &Vec<String>) -> Vec<Asteroid> {
    let mut asteroids = Vec::<Asteroid>::new();

    for y in 0..input.len() {
        let line = &input[y].as_bytes();
        for x in 0..line.len() {
            if line[x] == '#' as u8 {
                asteroids.push(Asteroid::new(Vector {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                }));
            }
        }
    }
    asteroids
}

/*
	Generate paths from source to target asteroids, if the target is in view (not blocked by a closer byt asteroid)
*/
fn generate_paths(source: &Asteroid, targets: &Vec<Asteroid>) -> Vec<Path> {
    // Create a collection of Paths for each asteroid
    let mut paths = Vec::<Path>::new();

    for target in targets {
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
    paths
}

/*
	This creates a vector of tuples which contains for each location a collection of Paths to other asteroids
	Asteroids that are blocked by other asteroids (same angle, closer by) are not included in the latter collection
*/
fn process_asteroids(asteroids: &Vec<Asteroid>) -> Vec<(Vector, Vec<Path>)> {
    let mut result = Vec::<(Vector, Vec<Path>)>::new();

    for source in asteroids {
        let paths = generate_paths(source, asteroids);
        result.push((source.location.clone(), paths));
    }

    result
}

pub fn test() {
    let source = vec_of_strings![".#..#", ".....", "#####", "....#", "...##"];

    let targetable = process_asteroids(&parse_asteroids(&source));

    for v in targetable {
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
        "Best location {} with {} targetable asteroids",
        best_location.0,
        best_location.1.len()
    );
}

pub fn part2() {
    // let input = vec_of_strings![
    // 	".#....#####...#..",
    // 	"##...##.#####..##",
    // 	"##...#...#.#####.",
    // 	"..#.....X...###..",
    // 	"..#.#.....#....##"
    // ];
    // let laser = Asteroid {
    // 	location: Coordinate { x: 8, y: 3 },
    // };

    // let input = vec_of_strings![
    // 	".#..##.###...#######",
    // 	"##.############..##.",
    // 	".#.######.########.#",
    // 	".###.#######.####.#.",
    // 	"#####.##.#.##.###.##",
    // 	"..#####..#.#########",
    // 	"####################",
    // 	"#.####....###.#.#.##",
    // 	"##.#################",
    // 	"#####.##.###..####..",
    // 	"..######..##.#######",
    // 	"####.##.####...##..#",
    // 	".#####..#.######.###",
    // 	"##...#.##########...",
    // 	"#.##########.#######",
    // 	".####.#.###.###.#.##",
    // 	"....##.##.###..#####",
    // 	".#.#.###########.###",
    // 	"#.#.#.#####.####.###",
    // 	"###.##.####.##.#..##"
    // ];
    // let laser = Asteroid {
    // 	location: Coordinate { x: 11, y: 13 },
    // };

    // Place the laser at location obtained from part 1
    let mut galaxy = parse_asteroids(&input());
    let laser = Asteroid {
        location: Vector { x: 26, y: 36, z: 0 },
    };

    let mut destroyed = 0;

    while galaxy.len() > 0 && destroyed < 200 {
        // Generate paths to targettable asteroids
        let mut paths = generate_paths(&laser, &galaxy);

        // Sort paths by increasing angle
        paths.sort_by(|a, b| {
            a.angle
                .partial_cmp(&b.angle)
                .expect("The world ceased to exist while sorting paths")
        });

        for p in paths {
            // Shoot them ...
            destroyed += 1;

            // and remove from galaxy
            galaxy.remove(
                galaxy
                    .iter()
                    .position(|a| a.location == p.target)
                    .expect("Eeks, asteroid not found in galaxy"),
            );

            // Enough with the shooting already
            if destroyed == 200 {
                println!(
                    "Destroyed asteroid {} was {} which results in {}",
                    destroyed,
                    p,
                    p.target.x * 100 + p.target.y
                );
                break;
            }
        }
    }
}
