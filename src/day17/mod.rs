use crate::common::*;
use crate::cpu::*;
use std::collections::HashMap;

type TileType = String;
type MapType = HashMap<Vector, TileType>;

enum Direction {
    UNKNOWN,
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

impl Direction {
    pub fn new(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::NORTH),
            '<' => Some(Direction::WEST),
            '>' => Some(Direction::EAST),
            'v' => Some(Direction::SOUTH),
            _ => None,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match *self {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            _ => panic!(),
        }
    }

    pub fn turn_right(&self) -> Direction {
        match *self {
            Direction::NORTH => Direction::EAST,
            Direction::WEST => Direction::NORTH,
            Direction::EAST => Direction::NORTH,
            Direction::SOUTH => Direction::WEST,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Direction::NORTH => Some("North"),
            Direction::WEST => Some("West"),
            Direction::EAST => Some("East"),
            Direction::SOUTH => Some("South"),
            _ => None,
        };

        writeln!(f, "{}", c.expect("Not a valid direction")).expect("IO error");

        std::fmt::Result::Ok(())
    }
}

struct Map {
    location: Vector,
    heading: Direction,
    tiles: MapType,
}

impl Map {
    fn new() -> Map {
        Map {
            location: Vector::new(0, 0, 0),
            heading: Direction::UNKNOWN,
            tiles: MapType::new(),
        }
    }

    fn update(&mut self, input: &Vec<CpuWidth>) {
        let mut current = Vector::new(0, 0, 0);

        for token in input.iter() {
            match token {
                10 => {
                    current.y += 1;
                    current.x = 0;
                }

                _ => {
                    let c = (*token as u8) as char;

                    if let Some(heading) = Direction::new(c) {
                        self.heading = heading;
                        self.location = current.clone();
                    }

                    self.tiles.insert(current.clone(), c.to_string());
                    current.x += 1;
                }
            }
        }
    }

    pub fn extends(&self) -> (Vector, Vector) {
        let x_min = self.tiles.keys().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let x_max = self.tiles.keys().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
        let y_min = self.tiles.keys().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        let y_max = self.tiles.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

        (Vector::new(x_min, y_min, 0), Vector::new(x_max, y_max, 0))
    }

    pub fn intersections(&self) -> Vec<Vector> {
        let mut result = vec![];

        let extends = self.extends();
        let m = Some("#".to_string());

        for y in extends.0.y + 1..extends.1.y {
            for x in extends.0.x + 1..extends.1.x {
                if self.tiles.get(&Vector { x: x, y: y, z: 0 }) == m.as_ref()
                    && self.tiles.get(&Vector {
                        x: x,
                        y: y - 1,
                        z: 0,
                    }) == m.as_ref()
                    && self.tiles.get(&Vector {
                        x: x - 1,
                        y: y,
                        z: 0,
                    }) == m.as_ref()
                    && self.tiles.get(&Vector {
                        x: x,
                        y: y + 1,
                        z: 0,
                    }) == m.as_ref()
                    && self.tiles.get(&Vector {
                        x: x + 1,
                        y: y,
                        z: 0,
                    }) == m.as_ref()
                {
                    result.push(Vector::new(x, y, 0));
                }
            }
        }
        result
    }

    pub fn move_robot(&mut self, m: char) {
        match m {
            'L' => self.heading = self.heading.turn_left(),
            'R' => self.heading = self.heading.turn_right(),
            '1'..='9' => {
                let reps = (m as u8) - ('0' as u8);

                for _ in 0..reps {
                    match self.heading {
                        Direction::NORTH => {
                            self.location.y -= 1;
                        }
                        Direction::WEST => {
                            self.location.x -= 1;
                        }
                        Direction::SOUTH => {
                            self.location.y += 1;
                        }
                        Direction::EAST => {
                            self.location.x += 1;
                        }
                        _ => panic!("Cannot move when heading is unknown"),
                    }
                }
            }
            _ => panic!("Invalid move"),
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extends = self.extends();
        for y in extends.0.y..=extends.1.y {
            for x in extends.0.x..=extends.1.x {
                write!(f, "{}", self.tiles[&Vector::new(x, y, 0)]).expect("IO error");
            }
            writeln!(f).expect("IO error");
        }
        std::fmt::Result::Ok(())
    }
}

#[test]
pub fn test() {
    let input = import_lines("src/day17/test.txt");
    let mut map = Map::new();

    map.update(&input.chars().map(|c| c as i64).collect::<Vec<i64>>());
    let intersections = map.intersections();

    for v in &intersections {
        println!("{} {}", v, v.x * v.y);
    }
    print!("{} {} {}", map, map.start, map.heading);
    assert_eq!(map.start, Vector::new(10, 6, 0));
    assert_eq!(intersections.iter().fold(0, |acc, c| acc + c.x * c.y), 76);
}

pub fn part1() {
    let input = import_lines("src/day17/input.txt");
    let mut program = read(&input);
    while !program.is_finished() {
        execute(&mut program);
    }

    let mut map = Map::new();
    map.update(&program.output);

    let intersections = map.intersections();
    println!(
        "Sum of the alignment parameters {}",
        intersections.iter().fold(0, |acc, c| acc + c.x * c.y)
    );

    print!("{}", map);
}

pub fn part2() {}
