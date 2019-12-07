mod cpu;

use cpu::Instruction;

fn read_program(input: &str) -> Vec<i32> {
	input
		.split(",")
		.map(|s| s.parse::<i32>().unwrap())
		.collect()
}

pub fn test() {
    let source = "1002,4,3,4,33";

    let program = read_program(source);

    let mut pc :usize = 0;
    let (instruction,pc) = Instruction::parse(&program, pc);

    println!("{}", instruction);

}

pub fn part1() {
}

pub fn part2() {
}
