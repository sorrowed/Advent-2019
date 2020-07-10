use std::collections::HashMap;

use crate::common::*;
use crate::cpu::*;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    UP = 0,
    LEFT = 1,
    DOWN = 2,
    RIGHT = 3,
}

#[derive(PartialEq, Copy, Clone)]
enum Color {
    BLACK = 0,
    WHITE = 1,
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

struct Robot {
    location: Vector,
    direction: Direction,
    world: HashMap<Vector, Color>,
}

impl Robot {
    pub fn new() -> Robot {
        Robot {
            location: Vector { x: 0, y: 0, z: 0 },
            direction: Direction::UP,
            world: HashMap::<_, _>::new(),
        }
    }

    pub fn paint(&mut self, color: Color) {
        self.world.insert(self.location.clone(), color);
    }

    fn turn(&mut self, dir: Direction) {
        match dir {
            Direction::LEFT => {
                self.direction = [
                    Direction::LEFT,
                    Direction::DOWN,
                    Direction::RIGHT,
                    Direction::UP,
                ][self.direction as usize];
            }
            Direction::RIGHT => {
                self.direction = [
                    Direction::RIGHT,
                    Direction::UP,
                    Direction::LEFT,
                    Direction::DOWN,
                ][self.direction as usize];
            }
            _ => panic!("Robot Can only turn left or right"),
        }
    }

    fn forward(&mut self) {
        match self.direction {
            Direction::UP => {
                self.location.y -= 1;
            }
            Direction::LEFT => {
                self.location.x -= 1;
            }
            Direction::DOWN => {
                self.location.y += 1;
            }
            Direction::RIGHT => {
                self.location.x += 1;
            }
        }
    }

    pub fn color(&self, location: &Vector) -> Color {
        if self.world.contains_key(location) {
            self.world[location]
        } else {
            Color::BLACK
        }
    }

    pub fn current(&mut self) -> Color {
        self.color(&self.location)
    }

    /*
        returns the top left and bottom right corner of the map so far
    */
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

    pub fn next(&mut self, color: Color, dir: Direction) {
        self.paint(color);
        self.turn(dir);
        self.forward();
    }

    pub fn painted(&self) -> usize {
        self.world.len()
    }
}

fn draw_trail(robot: &Robot) {
    let extends = robot.extends();
    for y in extends.0.y - 1..=extends.1.y + 1 {
        for x in extends.0.x - 1..=extends.1.x + 1 {
            print!(
                "{}",
                if robot.color(&Vector { x: x, y: y, z: 0 }) == Color::WHITE {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!("");
    }
}

pub fn test() {
    let mut robot = Robot::new();

    robot.next(Color::WHITE, Direction::LEFT);
    robot.next(Color::BLACK, Direction::LEFT);
    robot.next(Color::WHITE, Direction::LEFT);
    robot.next(Color::WHITE, Direction::LEFT);
    robot.next(Color::BLACK, Direction::RIGHT);
    robot.next(Color::WHITE, Direction::LEFT);
    robot.next(Color::WHITE, Direction::LEFT);

    draw_trail(&robot);

    println!("Painted {} panels on the test hull", robot.painted());
}

fn run_robot_program(robot: &mut Robot, program: &mut Program) {
    while !program.is_finished() {
        if program.is_waiting() {
            program.flush();
            program.add_input(if robot.current() == Color::BLACK {
                0
            } else {
                1
            });
        }
        execute(program);

        robot.next(
            if program.get_output(0).expect("No output") == 0 {
                Color::BLACK
            } else {
                Color::WHITE
            },
            if program.get_output(1).expect("No output") == 0 {
                Direction::LEFT
            } else {
                Direction::RIGHT
            },
        );
    }
}

pub fn part1() {
    let mut program = read(&import_lines("src/day11/input.txt"));

    let mut robot = Robot::new();

    run_robot_program(&mut robot, &mut program);

    draw_trail(&robot);

    println!("Painted {} panels on the hull", robot.painted());
}

pub fn part2() {
    let mut program = read(&import_lines("src/day11/input.txt"));

    let mut robot = Robot::new();
    robot.paint(Color::WHITE);

    run_robot_program(&mut robot, &mut program);

    draw_trail(&robot);

    println!("Painted {} panels on the hull", robot.painted());
}
