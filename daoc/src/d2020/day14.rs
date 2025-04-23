use std::collections::HashMap;

pub type Intermediate = Program;
pub type Output = u64;

pub fn parse(data: &str) -> Result<Intermediate, ProgramParseError> {
	data.parse()
}

impl Program {
	fn execute_part_one(&self) -> State {
		let mut state = State::new();

		for instr in &self.0 {
			match instr {
				SetMask(mask) => {
					state.mask = Some(mask.clone());
				}
				Write { address, value } => {
					let value = state.mask.clone().unwrap().apply(*value);
					state.memory.insert(*address, value);
				}
			}
		}

		state
	}
}

pub fn part_one(program: &Program) -> Option<Output> {
	Some(program.execute_part_one().memory.into_values().sum())
}

impl Program {
	fn execute_part_two(&self) -> State {
		let mut state = State::new();

		for instr in &self.0 {
			match instr {
				SetMask(mask) => {
					state.mask = Some(mask.clone());
				}
				Write { address, value } => {
					let addresses = state.mask.clone().unwrap().decode_memory_address(*address);

					for address in addresses {
						state.memory.insert(address, *value);
					}
				}
			}
		}

		state
	}
}

pub fn part_two(program: &Program) -> Option<Output> {
	Some(program.execute_part_two().memory.into_values().sum())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mask {
	raw: String,
	ones: u64,
	zeros: u64,
}

impl Mask {
	pub fn apply(&self, value: u64) -> u64 {
		(value & self.zeros) | self.ones
	}

	pub fn decode_memory_address(&self, address: u64) -> Vec<u64> {
		let address = format!("{address:b}").chars().rev().collect::<Vec<_>>();

		self
			.raw
			.chars()
			.rev()
			.enumerate()
			.fold(vec![], |possibilities, (shift, ch)| match ch {
				'0' | '1' => {
					let bit = if ch == '0' {
						if shift < address.len() {
							address[shift]
						} else {
							'0'
						}
					} else {
						'1'
					};

					if possibilities.is_empty() {
						vec![vec![bit]]
					} else {
						possibilities
							.into_iter()
							.map(|possibility| {
								let mut possibility = possibility;
								possibility.push(bit);
								possibility
							})
							.collect()
					}
				}
				'X' => {
					if possibilities.is_empty() {
						vec![vec!['0'], vec!['1']]
					} else {
						possibilities
							.into_iter()
							.flat_map(|possibility| {
								let mut v1 = possibility.clone();
								let mut v2 = possibility;

								v1.push('0');
								v2.push('1');
								vec![v1, v2]
							})
							.collect()
					}
				}
				_ => panic!("Unexpected mask char at {shift}: {ch}."),
			})
			.into_iter()
			.map(|possibility: Vec<char>| {
				let possibility = possibility.into_iter().rev().collect::<String>();
				u64::from_str_radix(possibility.trim_start_matches('0'), 2).unwrap()
			})
			.collect()
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
	SetMask(Mask),
	Write { address: u64, value: u64 },
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InstructionParseError {
	#[error("bitmask should be a u64 in binary: {0}")]
	Bitmask(String),
	#[error("memory address should be a u64 in binary: {0}")]
	Address(String),
	#[error("value should be a u64 in binary: {0}")]
	Value(String),
}

use Instruction::{SetMask, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program(Vec<Instruction>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
	memory: HashMap<u64, u64>,
	mask: Option<Mask>,
}

impl State {
	fn new() -> State {
		State {
			memory: HashMap::new(),
			mask: None,
		}
	}
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ProgramParseError {
	#[error("invalid instruction")]
	InvalidInstruction(#[from] InstructionParseError),
}

impl core::str::FromStr for Program {
	type Err = ProgramParseError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let instructions = str
			.trim_end()
			.split('\n')
			.map(|line| -> Result<Instruction, InstructionParseError> {
				let parts = &line.split(" = ").collect::<Vec<_>>();
				let (lhs, rhs) = (parts[0], parts[1]);

				if lhs == "mask" {
					let ones_mask = rhs.replace('X', "0");
					let ones = u64::from_str_radix(&ones_mask, 2)
						.map_err(|_| InstructionParseError::Bitmask(ones_mask))?;

					let zeros_mask = rhs.replace('X', "1");
					let zeros = u64::from_str_radix(&zeros_mask, 2)
						.map_err(|_| InstructionParseError::Bitmask(zeros_mask))?;

					Ok(SetMask(Mask {
						raw: rhs.to_string(),
						ones,
						zeros,
					}))
				} else {
					let address = lhs
						.trim_start_matches("mem[")
						.trim_end_matches(']')
						.parse::<u64>()
						.map_err(|_| InstructionParseError::Address(lhs.to_string()))?;
					let value = rhs
						.parse::<u64>()
						.map_err(|_| InstructionParseError::Value(rhs.to_string()))?;

					Ok(Write { address, value })
				}
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(Program(instructions))
	}
}

#[cfg(test)]
mod tests {}

daocutil::generate_solver!(solve, =>, self);
