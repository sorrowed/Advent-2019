use std::slice::Iter;
use std::{thread, time};

use crate::common::*;
use crate::cpu::*;

extern crate pathfinding;
use pathfinding::prelude::{absdiff, astar};

#[derive(PartialEq, Clone)]
enum DroidStatus {
    WALL = 0,
    MOVED = 1,
    OXYGEN = 2,
}

impl From<i64> for DroidStatus {
    fn from(input: i64) -> Self {
        match input {
            0 => DroidStatus::WALL,
            1 => DroidStatus::MOVED,
            2 => DroidStatus::OXYGEN,
            _ => panic!("Invalid DroidStatus input {}", input),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Move {
    NORTH = 1,
    SOUTH = 2,
    WEST = 3,
    EAST = 4,
}

impl Move {
    pub fn iterator() -> Iter<'static, Move> {
        static DIRECTIONS: [Move; 4] = [Move::NORTH, Move::SOUTH, Move::WEST, Move::EAST];
        DIRECTIONS.iter()
    }
}

#[derive(PartialEq, Clone)]
enum Movement {
    START,
    BLOCKED,
    FREE,
    PATH,
    OXYGEN,
}

impl Movement {
    fn is_reachable(&self) -> bool {
        *self == Movement::START || *self == Movement::FREE || *self == Movement::OXYGEN
    }
}

#[derive(PartialEq, Clone)]
struct Position {
    position: Vector,
    status: Movement,
}

type World = Vec<Position>;

/// Tries to move in given direction and returns the droid movement status
fn single_move(program: &mut Program, direction: &Move) -> DroidStatus {
    program.add_input(direction.clone() as i64);
    execute(program);
    let output = program.get_output(0).expect("Expected some output");
    program.flush();
    DroidStatus::from(output)
}

fn reverse_direction(moved: &Move) -> Move {
    match moved {
        Move::NORTH => Move::SOUTH,
        Move::SOUTH => Move::NORTH,
        Move::WEST => Move::EAST,
        Move::EAST => Move::WEST,
    }
}

fn target_position(position: &Vector, movement: &Move) -> Vector {
    let mut result = position.clone();

    match movement {
        Move::NORTH => result.y += 1,
        Move::SOUTH => result.y -= 1,
        Move::WEST => result.x -= 1,
        Move::EAST => result.x += 1,
    }
    result
}

/// Returns all neighbors of a location. Also indicates which move to take to get there and what type of location it is
fn explore_neighbors(program: &mut Program, position: &Vector) -> Vec<(Move, Position)> {
    let mut result = Vec::<(Move, Position)>::new();

    for this_move in Move::iterator() {
        // Try to move
        let status = match single_move(program, this_move) {
            DroidStatus::MOVED => Movement::FREE,
            DroidStatus::OXYGEN => Movement::OXYGEN,
            _ => Movement::BLOCKED,
        };

        if status != Movement::BLOCKED {
            // And move back
            single_move(program, &reverse_direction(this_move));
        }

        // Store move, position and if reachable
        result.push((
            this_move.clone(),
            Position {
                position: target_position(position, this_move),
                status: status,
            },
        ));
    }

    result
}

/// Returns extends (bottom left, top right) of world
fn world_extends(world: &World) -> (Vector, Vector) {
    let x_min = world
        .iter()
        .min_by(|x, y| x.position.x.cmp(&y.position.x))
        .unwrap()
        .position
        .x;
    let x_max = world
        .iter()
        .max_by(|x, y| x.position.x.cmp(&y.position.x))
        .unwrap()
        .position
        .x;
    let y_min = world
        .iter()
        .min_by(|x, y| x.position.y.cmp(&y.position.y))
        .unwrap()
        .position
        .y;
    let y_max = world
        .iter()
        .max_by(|x, y| x.position.y.cmp(&y.position.y))
        .unwrap()
        .position
        .y;

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

fn world_contains<'a>(world: &'a World, location: &Vector) -> Option<&'a Position> {
    world
        .iter()
        .find(|p| p.position.x == location.x && p.position.y == location.y)
}

fn world_contains_mut<'a>(world: &'a mut World, location: &Vector) -> Option<&'a mut Position> {
    world
        .iter_mut()
        .find(|p| p.position.x == location.x && p.position.y == location.y)
}

fn world_print(world: &World) {
    let (bl, tr) = world_extends(world);

    for y in (bl.y..tr.y + 1).rev() {
        for x in bl.x..tr.x + 1 {
            let movement = if x == 0 && y == 0 {
                Movement::START
            } else if let Some(p) = world_contains(world, &Vector { x: x, y: y, z: 0 }) {
                p.status.clone()
            } else {
                Movement::BLOCKED
            };

            print!(
                "{}",
                match movement {
                    Movement::START => '#',
                    Movement::PATH => '.',
                    Movement::FREE => ' ',
                    Movement::OXYGEN => '*',
                    _ => '+',
                }
            );
        }
        println!();
    }
}

/// Build the world by examining valid moves
/// This is a DFS algorithm
fn world_build(program: &mut Program) -> World {
    let mut world = World::new();

    let start = Position {
        position: Vector { x: 0, y: 0, z: 0 },
        status: Movement::START,
    };

    // Add initial position to world
    world.push(start.clone());

    // DFS backtrace stack
    let mut dfs = Vec::<(Move, Position)>::new();
    dfs.push((Move::NORTH, start));

    while dfs.len() > 0 {
        let waypoint = dfs.last().expect("Not good").clone();

        let neighbors = explore_neighbors(program, &waypoint.1.position);

        let mut moved = false;
        for neighbor in neighbors {
            if world_contains(&mut world, &neighbor.1.position) == None {
                // Add it to the world
                world.push(neighbor.1.clone());

                if neighbor.1.status.is_reachable() {
                    // Location is reachable. move to it and explore further
                    single_move(program, &neighbor.0);

                    dfs.push(neighbor);

                    moved = true;
                    break;
                }
            }
        }

        // End of the line, move back to previous position
        if !moved {
            single_move(program, &reverse_direction(&waypoint.0));
            // And remove the dead end
            dfs.pop();
        }
    }
    world
}

/// Cannot move diagonally
fn is_adjacent(a: &Vector, b: &Vector) -> bool {
    ((a.x - b.x).abs() <= 1 && (a.y - b.y).abs() == 0)
        || ((a.y - b.y).abs() <= 1 && (a.x - b.x).abs() == 0)
}

/// Find valid neighbors for given position
fn neighbors<'a>(world: &'a World, position: &Vector) -> Vec<&'a Position> {
    world
        .iter()
        .filter_map(|p| {
            if p.status != Movement::BLOCKED && is_adjacent(&p.position, position) {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}

/// Manhattan distance between positions
fn distance(a: &Vector, b: &Vector) -> u32 {
    (absdiff(a.x, b.x) + absdiff(a.y, b.y)) as u32
}

pub fn test() {}

pub fn part1() {
    let mut world = world_build(&mut read(&import_lines("src/day15/input.txt")));
    let start = world
        .iter()
        .find(|&s| s.status == Movement::START)
        .expect("Badness, no start found")
        .clone();
    let end = world
        .iter()
        .find(|&s| s.status == Movement::OXYGEN)
        .expect("Badness, no end found")
        .clone();

    world_print(&world);
    println!("Oxygen found at {}", end.position);

    let path = astar(
        &start.position,
        |p| {
            neighbors(&world, p)
                .iter()
                .map(|p| (p.position.clone(), 1))
                .collect::<Vec<(Vector, u32)>>()
        },
        |p| distance(p, &end.position),
        |p| *p == end.position,
    )
    .expect("No path found");

    // Mark the path found
    for waypoint in &path.0 {
        if waypoint != &end.position {
            if let Some(p) = world_contains_mut(&mut world, waypoint) {
                p.status = Movement::PATH
            }
        }
        world_print(&world);
        thread::sleep(time::Duration::from_millis(50));
    }

    println!("Number of moves: {}", path.1);
}

pub fn part2() {
    let mut world = world_build(&mut read(&import_lines("src/day15/input.txt")));

    let mut minutes = 0;
    loop {
        // All locations that are not already oxygen
        let not_oxygen = world
            .iter()
            .filter(|p| p.status.is_reachable() && p.status != Movement::OXYGEN)
            .collect::<Vec<&Position>>();

        let mut new_oxygen = Vec::<Vector>::new();

        //  We're done if all positions are filled with oxygen
        if not_oxygen.len() == 0 {
            break;
        } else {
            // See if any of them have a neighbor filled with oxygen
            for p in not_oxygen {
                for neighbor in neighbors(&world, &p.position) {
                    if world
                        .iter()
                        .any(|p| p.position == neighbor.position && p.status == Movement::OXYGEN)
                    {
                        // If so, they are going to be filled with oxygen this minute
                        new_oxygen.push(p.position.clone());

                        // Any neighbor suffices
                        break;
                    }
                }
            }
        }

        // Mark all new oxygen positions
        for position in new_oxygen {
            world
                .iter_mut()
                .find(|p| p.position == position)
                .unwrap()
                .status = Movement::OXYGEN;
        }

        minutes += 1;

        world_print(&world);
        thread::sleep(time::Duration::from_millis(50));
    }

    println!(
        "It took {} minutes for oxygen to fill all reachable locations",
        minutes
    );
}
