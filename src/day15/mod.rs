use std::collections::HashMap;

use crate::common::*;
use crate::cpu::*;


extern crate pathfinding;

#[derive(PartialEq, Clone)]
enum DroidStatus {
    WALL = 0,
    MOVED = 1,
    OXYGEN = 2,
}

#[derive(PartialEq, Clone)]
enum Move {
    NORTH = 1,
    SOUTH = 2,
    WEST = 3,
    EAST = 4,
}

pub fn test() {
    let input = import_lines("src/day15/input.txt");

    let mut position = Vector { x: 0, y: 0, z: 0 };
    let mut world = HashMap::<Vector,Vector>::new();

    loop {
        let mut program = read(&input);

        program.add_input(Move::EAST as i64);
        execute(&mut program);

        print!("{}", program.get_output(0).expect("Expected some output"));

        
    }
}

pub fn part1() {}
pub fn part2() {}
