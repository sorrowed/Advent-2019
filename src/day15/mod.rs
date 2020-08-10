use std::collections::HashMap;
use std::slice::Iter;

use crate::common::*;
use crate::cpu::*;

extern crate pathfinding;

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

type World = HashMap<Vector, bool>;

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

fn target_position(position: &Vector, moved: &Move) -> Vector {
    match moved {
        Move::NORTH => {
            return Vector {
                x: position.x,
                y: position.y + 1,
                z: position.z,
            }
        }

        Move::SOUTH => {
            return Vector {
                x: position.x,
                y: position.y - 1,
                z: position.z,
            }
        }

        Move::WEST => {
            return Vector {
                x: position.x - 1,
                y: position.y,
                z: position.z,
            }
        }

        Move::EAST => {
            return Vector {
                x: position.x + 1,
                y: position.y,
                z: position.z,
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct Waypoint {
    moved: Move,
    position: Vector,
    reachable: bool,
}

fn explore_position(program: &mut Program, position: &Vector) -> Vec<Waypoint> {
    let mut result = Vec::<Waypoint>::new();

    for this_move in Move::iterator() {
        // Try to move
        let status = single_move(program, this_move);

        let reachable = status == DroidStatus::MOVED || status == DroidStatus::OXYGEN;

        // Store move, position and if reachable
        result.push(Waypoint {
            moved: this_move.clone(),
            position: target_position(position, this_move),
            reachable: reachable,
        });

        if reachable {
            // And move back
            single_move(program, &reverse_direction(this_move));
        }

        //FIXME: Remember OXYGEN position
        if status == DroidStatus::OXYGEN {
            println!("Oxygen foud at {}", target_position(position, this_move));
        }
    }

    result
}

pub fn test() {
    let input = import_lines("src/day15/input.txt");
    let mut program = read(&input);

    let mut world = World::new();

    let mut positions = Vec::<Waypoint>::new();

    // Add initial position
    let start = Waypoint {
        moved: Move::NORTH,
        position: Vector { x: 0, y: 0, z: 0 },
        reachable: true,
    };

    world.insert(start.position.clone(), start.reachable.clone());
    positions.push(start);

    loop {
        if positions.len() > 0 {
            let waypoint = positions.last().expect("Not good").clone();
            println!("I'm now at {}\n", waypoint.position);

            let neighbors = explore_position(&mut program, &waypoint.position);

            let mut moved = false;
            for neighbor in neighbors {
                if !world.contains_key(&neighbor.position) {
                    // Add it to the world
                    world.insert(neighbor.position.clone(), neighbor.reachable.clone());
                    if neighbor.reachable {
                        // Location is reachable. move to it and explore further
                        single_move(&mut program, &neighbor.moved);

                        println!("Adding to locations to explore {}\n", neighbor.position);

                        positions.push(neighbor);

                        moved = true;
                        break;
                    }
                }
            }

            if !moved {
                // End of the line, move back to previous position
                single_move(&mut program, &reverse_direction(&waypoint.moved));
                // And remove the dead end
                positions.pop();
            }
        } else {
            break;
        }
    }
}

pub fn part1() {}
pub fn part2() {}
