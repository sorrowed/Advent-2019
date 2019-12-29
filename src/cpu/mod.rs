use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read, Write};

#[derive(Clone)]
pub struct Program {
	memory: HashMap<usize, i64>,
	pc: usize,
	input: Vec<i64>,
	output: Option<i64>,
	input_ix: usize,
	pub interactive: bool,
	relative_base: usize,
	state:i32
}

impl Program {
	pub fn new(instructions: HashMap<usize, i64>) -> Program {
		Program {
			memory: instructions,
			pc: 0,
			relative_base: 0,
			input: vec![],
			output: None,
			input_ix: 0,
			interactive: false,
			state : 0,
		}
	}

	pub fn add_input(&mut self, input: i64) {
		self.input.push(input);
	}

	pub fn reset_input(&mut self) {
		self.input.clear();
		self.input_ix = 0;
	}

	pub fn is_waiting(&self) -> bool {
		!self.interactive && self.input_ix >= self.input.len()
	}

	pub fn is_finished(&self) -> bool {
		self.state != 0
	}

	pub fn get_output(&self) -> Option<i64> {
		self.output
	}
}

#[derive(Clone, Copy)]
pub enum ParameterMode {
	POSITION = 0,
	IMMEDIATE = 1,
	RELATIVE = 2,
	INVALID,
}

impl std::fmt::Display for ParameterMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", if (*self as i64) == 0 { "P" } else { "I" })
	}
}

pub struct Parameter {
	mode: ParameterMode,
	value: i64,
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

	fn relative_address(&self, program: &Program) -> usize {
		if self.value < 0 {
			(program.relative_base - self.value.abs() as usize)
		} else {
			(program.relative_base + self.value.abs() as usize)
		}
	}

	pub fn parse(&mut self, instruction: i64, rank: usize, value: i64) {
		let mut mode = (instruction / 100) % 1000;
		mode /= if rank > 0 { 10 * (rank as i64) } else { 1 };
		mode %= 10;

		self.mode = match mode {
			0 => ParameterMode::POSITION,
			1 => ParameterMode::IMMEDIATE,
			2 => ParameterMode::RELATIVE,
			_ => ParameterMode::INVALID,
		};
		self.value = value;
	}

	pub fn get(&mut self, program: &Program) -> Option<i64> {
		match self.mode {
			// If the memory location does not exist, return a default 0 value
			ParameterMode::POSITION => {
				let key = &(self.value as usize);

				let value = if program.memory.contains_key(key) {
					program.memory[key]
				} else {
					0
				};

				Some(value)
			}
			ParameterMode::IMMEDIATE => Some(self.value),
			ParameterMode::RELATIVE => {
				let key = &self.relative_address(program);

				let value = if program.memory.contains_key(key) {
					program.memory[key]
				} else {
					0
				};

				Some(value)
			}
			_ => None,
		}
	}

	pub fn set(&mut self, program: &mut Program, value: i64) {
		match self.mode {
			ParameterMode::POSITION => {
				program.memory.insert(self.value as usize, value);
			}
			ParameterMode::RELATIVE => {
				let index = self.relative_address(program);

				program.memory.insert(index, value);
			}
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
	RB = 9,
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
			Opcode::RB => write!(f, "RB\t"),
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
		self.opcode as i64 == 3
	}

	pub fn is_quit(&self) -> bool {
		self.opcode as i64 == 99
	}

	pub fn execute(mut self, program: &mut Program) {
		let mut new_pc = program.pc + self.size;

		match self.opcode {
			Opcode::ADD => {
				let a = self.parameters[0].get(program);
				let b = self.parameters[1].get(program);
				let c = &mut self.parameters[2];

				match (a, b) {
					(Some(a), Some(b)) => {
						c.set(program, a + b);
					}
					_ => {panic!("ADD requires three operands")}
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
					_ => {panic!("MUL requires three operands")}
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
								.push(input.trim().parse::<i64>().expect("Error in input"));
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

			Opcode::RB => {
				let a = self.parameters[0]
					.get(program)
					.expect("RB requires one operand");
				program.relative_base = if a < 0 {
					program.relative_base - a.abs() as usize
				} else {
					program.relative_base + a.abs() as usize
				};
			}

			_ => {}
		}

		program.pc = new_pc
	}

	pub fn parse(program: &Program) -> Instruction {
		let instruction = program.memory[&program.pc];

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
			9 => Opcode::RB,
			99 => Opcode::QUIT,
			_ => Opcode::INVALID,
		};

		match result.opcode {
			Opcode::ADD | Opcode::MUL | Opcode::LT | Opcode::EQ => {
				for rank in 0..3 {
					let p = &mut result.parameters[rank];

					p.parse(instruction, rank, program.memory[&(program.pc + result.size)]);
					result.size += 1;
				}
			}
			Opcode::IN | Opcode::OUT | Opcode::RB => {
				let p = &mut result.parameters[0];

				p.parse(instruction, 0, program.memory[&(program.pc + result.size)]);
				result.size += 1;
			}
			Opcode::JIT | Opcode::JIZ => {
				for rank in 0..2 {
					let p = &mut result.parameters[rank];

					p.parse(instruction, rank, program.memory[&(program.pc + result.size)]);
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
	while program.state == 0 {
		let instruction = Instruction::parse(&program);
		// If the program is non-interactive and needs input let it pause
		if instruction.is_quit() {
			program.state = 1;
		} else if instruction.is_input() && program.is_waiting() {
			break;
		} else {
			instruction.execute(program);
		}
	}

	program.state == 0
}

pub fn read(input: &str) -> Program {
	let mut index = 0;

	Program::new(
		input
			.split(",")
			.map(|s| {
				(
					{
						index += 1;
						index - 1
					},
					s.parse::<i64>().expect("Invalid instruction in input"),
				)
			})
			.collect(),
	)
}
