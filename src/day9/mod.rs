use crate::common::import_lines;
use crate::cpu::*;

pub fn test() {
	let input = "1102,34915192,34915192,7,4,7,99,0";
	//let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
	//let input = "104,1125899906842624,99";

	let mut program = read(&input);
	execute(&mut program);
}

pub fn part1() {
	let input = import_lines("src/day9/input.txt");

	let mut program = read(&input);
	program.add_input(1);
	execute(&mut program);
	println!("BOOST test mode output: {}", program.get_output().expect("Error, BOOST program TEST mode produced no output"))
}

pub fn part2() {
	let input = import_lines("src/day9/input.txt");

	let mut program = read(&input);
	program.add_input(2);
	execute(&mut program);
	println!("BOOST sensor mode output: {}", program.get_output().expect("Error, BOOST program SENSOR mode produced no output"))
}
