use std::fmt;

enum ParameterMode {
    POSITION = 0,
    IMMEDIATE = 1,
    INVALID,
}

struct Parameter {
    mode: ParameterMode,
    value: i32,
}

impl Parameter {
    pub fn new() -> Parameter {
        Parameter {
            mode: ParameterMode::INVALID,
            value: 0,
        }
    }

    pub fn parse(mut self, instruction: i32, value: i32) {
        self.mode = match (instruction / 100) % 10 {
            0 => ParameterMode::POSITION,
            1 => ParameterMode::IMMEDIATE,
            _ => ParameterMode::INVALID,
        };
        self.value = value;
    }
}

enum Opcode {
    ADD = 1,
    MUL = 2,
    QUIT = 99,
    INVALID,
}
impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Instruction {
    opcode: Opcode,
    parameters: [Parameter; 3],
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OP: {}", self.opcode)
    }
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction {
            opcode: Opcode::INVALID,
            parameters: [Parameter::new(),Parameter::new(),Parameter::new()],
        }
    }

    pub fn parse(program: &Vec<i32>, mut pc: usize) -> (Instruction, usize) {
        let instruction = program[pc];
        pc += 1;

        let mut result = Instruction::new();

        result.opcode = match instruction % 100 {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            99 => Opcode::QUIT,
            _ => Opcode::INVALID,
        };

        match result.opcode {
            Opcode::ADD | Opcode::MUL => {
                for ix in 0..3 {

                    let mut p = result.parameters[ix];
                    p.parse(instruction, program[pc]);
                    pc += 1;
                }
            }
            Opcode::QUIT => {}
            _ => {}
        }

        (result, pc)
    }
}
