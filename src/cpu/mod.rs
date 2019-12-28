use std::fmt;
use std::io::{self, Read, Write};

#[derive(Clone)]
pub struct Program {
	instructions: Vec<i32>,
	pc: usize,
	input: Vec<i32>,
	output: Option<i32>,
	input_ix: usize,
	interactive: bool,
}

impl Program {
	pub fn new(instructions: Vec<i32>) -> Program {
		Program {
			instructions: instructions,
			pc: 0,
			input: vec![],
			output: None,
			input_ix: 0,
			interactive: false,
		}
	}

	pub fn add_input(&mut self, input: i32) {
		self.input.push(input);
	}

	pub fn reset_input(&mut self) {
		self.input_ix = 0;
	}

	pub fn set_input(&mut self, index: usize, input: i32) {
		self.input[index] = input;
	}

	pub fn get_output(&self) -> Option<i32> {
		self.output
	}
}

#[derive(Clone, Copy)]
pub enum ParameterMode {
	POSITION = 0,
	IMMEDIATE = 1,
	INVALID,
}

impl std::fmt::Display for ParameterMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", if (*self as i32) == 0 { "P" } else { "I" })
	}
}

pub struct Parameter {
	mode: ParameterMode,
	value: i32,
}

impl std::fmt::Display for Parameter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "P:[{} {:5}]", self.mode, self.value)
	}
}

impl Parameter {
	pub fn new() -> Parameter {
		Parameter {
			mode: ParameterMode::INVALID,
			value: 0,
		}
	}

	pub fn parse(&mut self, instruction: i32, rank: usize, value: i32) {
		let mut mode = (instruction / 100) % 1000;
		mode /= if rank > 0 { 10 * (rank as i32) } else { 1 };
		mode %= 10;

		self.mode = match mode {
			0 => ParameterMode::POSITION,
			1 => ParameterMode::IMMEDIATE,
			_ => ParameterMode::INVALID,
		};
		self.value = value;
	}

	pub fn get(&mut self, program: &Program) -> Option<i32> {
		match self.mode {
			ParameterMode::POSITION => Some(program.instructions[self.value as usize]),
			ParameterMode::IMMEDIATE => Some(self.value),
			_ => None,
		}
	}

	pub fn set(&mut self, program: &mut Program, value: i32) {
		match self.mode {
			ParameterMode::POSITION => program.instructions[self.value as usize] = value,
			_ => {}
		}
	}
}

#[derive(Clone, Copy)]
pub enum Opcode {
	ADD = 1,
	MUL = 2,
	IN = 3,
	OUT = 4,
	JIT = 5,
	JIZ = 6,
	LT = 7,
	EQ = 8,
	QUIT = 99,
	INVALID,
}
impl std::fmt::Display for Opcode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Opcode::ADD => write!(f, "ADD\t"),
			Opcode::MUL => write!(f, "MUL\t"),
			Opcode::IN => write!(f, "IN\t"),
			Opcode::OUT => write!(f, "OUT\t"),
			Opcode::JIT => write!(f, "JIT\t"),
			Opcode::JIZ => write!(f, "JIZ\t"),
			Opcode::LT => write!(f, "LT\t"),
			Opcode::EQ => write!(f, "EQ\t"),
			Opcode::QUIT => write!(f, "QUIT"),
			_ => write!(f, "ERR!\t"),
		}
	}
}

pub struct Instruction {
	pub opcode: Opcode,
	pub parameters: [Parameter; 3],
	pub size: usize,
}

impl std::fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{} {} {} {}",
			self.opcode, self.parameters[0], self.parameters[1], self.parameters[2]
		)
	}
}

impl Instruction {
	pub fn new() -> Instruction {
		Instruction {
			opcode: Opcode::INVALID,
			parameters: [Parameter::new(), Parameter::new(), Parameter::new()],
			size: 0,
		}
	}

	pub fn is_input(&self) -> bool {
		self.opcode as i32 == 3
	}

	pub fn is_quit(&self) -> bool {
		self.opcode as i32 == 99
	}

	pub fn execute(mut self, program: &mut Program, pc: usize) -> usize {
		let mut new_pc = pc + self.size;

		match self.opcode {
			Opcode::ADD => {
				let a = self.parameters[0].get(program);
				let b = self.parameters[1].get(program);
				let c = &mut self.parameters[2];

				match (a, b) {
					(Some(a), Some(b)) => {
						c.set(program, a + b);
					}
					_ => {}
				}
			}
			Opcode::MUL => {
				let a = self.parameters[0].get(program);
				let b = self.parameters[1].get(program);
				let c = &mut self.parameters[2];

				match (a, b) {
					(Some(a), Some(b)) => {
						c.set(program, a * b);
					}
					_ => {}
				}
			}
			Opcode::IN => {
				if program.input.len() <= program.input_ix {
					io::stdout().write_all(b"INPUT?\n").expect("ERR!");

					let mut input = String::new();
					match io::stdin().read_line(&mut input) {
						Ok(_n) => {
							program
								.input
								.push(input.trim().parse::<i32>().expect("Error in input"));
						}
						Err(error) => println!("error: {}", error),
					}
				}

				&mut self.parameters[0].set(program, program.input[program.input_ix]);
				program.input_ix += 1;
			}

			Opcode::OUT => {
				program.output = self.parameters[0].get(program);

				if program.interactive {
					writeln!(
						io::stdout(),
						"OUT {}",
						program.output.expect("Error, no output set")
					)
					.expect("ERR!");
				}
			}

			Opcode::JIT => {
				let a = self.parameters[0].get(program);
				let b = self.parameters[1].get(program);
				if a != Some(0) {
					new_pc = b.expect("invalid operand b in JIT") as usize;
				}
			}

			Opcode::JIZ => {
				let a = self.parameters[0].get(program);
				let b = self.parameters[1].get(program);
				if a == Some(0) {
					new_pc = b.expect("invalid operand b in JIZ") as usize;
				}
			}

			Opcode::LT => {
				let a = self.parameters[0].get(program);
				let b = self.parameters[1].get(program);
				if Some(a) < Some(b) {
					&mut self.parameters[2].set(program, 1);
				} else {
					&mut self.parameters[2].set(program, 0);
				}
			}

			Opcode::EQ => {
				let a = self.parameters[0].get(program);
				let b = self.parameters[1].get(program);
				if Some(a) == Some(b) {
					&mut self.parameters[2].set(program, 1);
				} else {
					&mut self.parameters[2].set(program, 0);
				}
			}

			_ => {}
		}

		new_pc
	}

	pub fn parse(program: &Program, pc: usize) -> Instruction {
		let instruction = program.instructions[pc];

		let mut result = Instruction::new();
		result.size = 1;

		result.opcode = match instruction % 100 {
			1 => Opcode::ADD,
			2 => Opcode::MUL,
			3 => Opcode::IN,
			4 => Opcode::OUT,
			5 => Opcode::JIT,
			6 => Opcode::JIZ,
			7 => Opcode::LT,
			8 => Opcode::EQ,
			99 => Opcode::QUIT,
			_ => Opcode::INVALID,
		};

		match result.opcode {
			Opcode::ADD | Opcode::MUL | Opcode::LT | Opcode::EQ => {
				for rank in 0..3 {
					let p = &mut result.parameters[rank];

					p.parse(instruction, rank, program.instructions[pc + result.size]);
					result.size += 1;
				}
			}
			Opcode::IN | Opcode::OUT => {
				let p = &mut result.parameters[0];

				p.parse(instruction, 0, program.instructions[pc + result.size]);
				result.size += 1;
			}
			Opcode::JIT | Opcode::JIZ => {
				for rank in 0..2 {
					let p = &mut result.parameters[rank];

					p.parse(instruction, rank, program.instructions[pc + result.size]);
					result.size += 1;
				}
			}
			Opcode::QUIT => {
				result.size = std::usize::MAX;
			}
			_ => {}
		}

		result
	}
}

pub fn execute(program: &mut Program) -> bool {
	let mut quit = false;
	let mut pause = false;

	while !quit && !pause {
		let instruction = Instruction::parse(&program, program.pc);
		// If the program is non-interactive and needs input let it pause
		if instruction.is_input() && !program.interactive && program.input_ix >= program.input.len() {
			pause = true;
		} else if instruction.is_quit() {
			quit = true;
		} else {
			program.pc = instruction.execute(program, program.pc);
		}
	}

	!quit
}

pub fn read(input: &str) -> Program {
	Program::new(
		input
			.split(",")
			.map(|s| s.parse::<i32>().expect("Invalid instruction in input"))
			.collect(),
	)
}
