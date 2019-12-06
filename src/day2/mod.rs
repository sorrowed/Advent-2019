fn input() -> &'static str {
	"1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,10,19,23,2,9,23,27,1,6,27,31,2,31,9,35,1,5,35,39,1,10,39,43,1,10,43,47,2,13,47,51,1,10,51,55,2,55,10,59,1,9,59,63,2,6,63,67,1,5,67,71,1,71,5,75,1,5,75,79,2,79,13,83,1,83,5,87,2,6,87,91,1,5,91,95,1,95,9,99,1,99,6,103,1,103,13,107,1,107,5,111,2,111,13,115,1,115,6,119,1,6,119,123,2,123,13,127,1,10,127,131,1,131,2,135,1,135,5,0,99,2,14,0,0"
}

fn read_program(input: &str) -> Vec<usize> {
	input
		.split(",")
		.map(|s| s.parse::<usize>().unwrap())
		.collect()
}

fn execute(mut program: Vec<usize>) -> usize {
	let mut pc: usize = 0;
	let mut opcode = program[pc];
	while opcode != 99 {
		let parm1 = program[pc + 1];
		let parm2 = program[pc + 2];
		let parm3 = program[pc + 3];

		match opcode {
			1 => {
				program[parm3] = program[parm1] + program[parm2];
				pc += 4;
			}

			2 => {
				program[parm3] = program[parm1] * program[parm2];
				pc += 4;
			}

			_ => {}
		}

		opcode = program[pc];
	}
	return program[0];
}

pub fn test() {
	let program = read_program("1,9,10,3,2,3,11,0,99,30,40,50");
	execute(program);
}

pub fn part1() {
	let mut program = read_program(input());
	program[1] = 12;
	program[2] = 2;
	execute(program);
}

pub fn part2() {
	for verb in 0..100 {
		for noun in 0..100 {
			let mut program = read_program(input());
			program[1] = noun;
			program[2] = verb;
			let result = execute(program);
			if result == 19690720 {
				println!(
					"noun: {} verb: {} result: {}",
					noun,
					verb,
					100 * noun + verb
				);
				break;
			}
		}
	}
}