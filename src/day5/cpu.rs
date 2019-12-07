use std::fmt;

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

	pub fn get(&mut self, program: &Vec<i32>) -> Option<i32> {
		match self.mode {
			ParameterMode::POSITION => Some(program[self.value as usize]),
			ParameterMode::IMMEDIATE => Some(self.value),
			_ => None,
		}
	}

	pub fn set(&mut self, program: &mut Vec<i32>, value: i32) {
		match self.mode {
			ParameterMode::POSITION => program[self.value as usize] = value,
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

	pub fn is_quit(&self) -> bool {
		self.opcode as i32 == 99
	}

	pub fn execute(mut self, program: &mut Vec<i32>, pc: usize) -> usize {
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
				&mut self.parameters[0].set(program, 1); // The only input is 1
			}
			Opcode::OUT => {
				println!("DIAGNOSTICS: {}", self.parameters[0].get(program).unwrap());
			}

			_ => {}
		}

		pc + self.size
	}

	pub fn parse(program: &Vec<i32>, pc: usize) -> Instruction {
		let instruction = program[pc];

		let mut result = Instruction::new();
		result.size = 1;

		result.opcode = match instruction % 100 {
			1 => Opcode::ADD,
			2 => Opcode::MUL,
			3 => Opcode::IN,
			4 => Opcode::OUT,
			99 => Opcode::QUIT,
			_ => Opcode::INVALID,
		};

		match result.opcode {
			Opcode::ADD | Opcode::MUL => {
				for rank in 0..3 {
					let p = &mut result.parameters[rank];

					p.parse(instruction, rank, program[pc + result.size]);
					result.size += 1;
				}
			}
			Opcode::IN | Opcode::OUT => {
				let p = &mut result.parameters[0];

				p.parse(instruction, 0, program[pc + result.size]);
				result.size += 1;
			}
			Opcode::QUIT => {
				result.size = std::usize::MAX;
			}
			_ => {}
		}

		result
	}
}
