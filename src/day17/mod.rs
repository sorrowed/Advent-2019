use crate::common::*;
use crate::cpu::*;
use std::collections::HashMap;
use std::{thread, time};

type TileType = String;
type MapType = HashMap<Vector, TileType>;

#[derive(Clone)]
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
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
            _ => panic!(),
        }
    }

    pub fn turn_right(&self) -> Direction {
        match *self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Direction::NORTH => Some('^'),
            Direction::WEST => Some('<'),
            Direction::EAST => Some('>'),
            Direction::SOUTH => Some('v'),
            _ => None,
        };

        write!(f, "{}", c.expect("Not a valid direction")).expect("IO error");

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

    fn forward(location: &Vector, heading: &Direction) -> Vector {
        let mut result = location.clone();

        match heading {
            Direction::NORTH => {
                result.y -= 1;
            }
            Direction::WEST => {
                result.x -= 1;
            }
            Direction::SOUTH => {
                result.y += 1;
            }
            Direction::EAST => {
                result.x += 1;
            }
            _ => panic!("Cannot move when heading is unknown"),
        }

        result
    }

    pub fn find_route(&mut self) -> Vec<String> {
        let mut route = vec![];
        let mut turn: Option<String> = None;

        let mut turns = 0;
        let mut forwards = 0;

        while turns < 3 {
            let l = Self::forward(&self.location, &self.heading);

            let valid_move = if let Some(p) = self.tiles.get(&l) {
                p == "#"
            } else {
                false
            };

            if valid_move {
                *self.tiles.get_mut(&self.location).unwrap() = "#".to_string();

                self.location = Self::forward(&self.location, &self.heading);
                if let Some(t) = turn {
                    if forwards > 0 {
                        route.push(forwards.to_string());
                        forwards = 0;
                    }

                    route.push(t);
                }
                turn = None;
                turns = 0;

                forwards += 1;

                //println!("\x1B[1;1H{}", self);
                //std::thread::sleep(time::Duration::from_millis(10));
            } else {
                if turns == 0 {
                    self.heading = self.heading.turn_right();
                    turn = Some("R".to_string());
                } else if turns == 1 {
                    self.heading = self.heading.turn_left().turn_left();
                    turn = Some("L".to_string());
                } else {
                    // Must be the end of the scaffolding
                    if forwards > 0 {
                        route.push(forwards.to_string());
                    }
                    break;
                }
                turns += 1;
            }
        }

        route
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extends = self.extends();
        for y in extends.0.y..=extends.1.y {
            for x in extends.0.x..=extends.1.x {
                if self.location.x == x && self.location.y == y {
                    write!(f, "{}", self.heading).expect("IO error");
                } else {
                    write!(f, "{}", self.tiles[&Vector::new(x, y, 0)]).expect("IO error");
                }
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
    print!("{} {} {}", map, map.location, map.heading);
    assert_eq!(map.location, Vector::new(10, 6, 0));
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
}

pub fn part2() {
    let input = import_lines("src/day17/input.txt");
    let mut program = read(&input);
    while !program.is_finished() {
        execute(&mut program);
    }

    let mut map = Map::new();
    map.update(&program.output);

    let route = map.find_route();
    for c in route {
        print!("{}", c)
    }
    println!("");

    let mut program = read(&input);
    program.flush();

    program.set(0, 2);

    // Printing the latter shows the following sub-paths in order A,A,B,A,B,C,B
    let moves = "A,B,A,B,C,A,B,C,A,C\nR,6,L,10,R,8\nR,8,R,12,L,8,L,8\nL,10,R,6,R,6,L,8\nn\n";
    for c in moves.chars(){
        program.add_input((c as u8)as i64);
    }

    while !program.is_finished() {
        execute(&mut program);
    }

    println!("Dust collected : {}", program.get_output(program.output.len()-1).unwrap())
}
