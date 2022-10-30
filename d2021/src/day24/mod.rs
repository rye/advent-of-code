#![allow(dead_code, unused)]

use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Value {
	W,
	X,
	Y,
	Z,
	Number(i64),
}

impl core::str::FromStr for Value {
	type Err = core::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().nth(0) {
			Some('w') => Ok(Value::W),
			Some('x') => Ok(Value::X),
			Some('y') => Ok(Value::Y),
			Some('z') => Ok(Value::Z),
			_ => Ok(Value::Number(s.parse().expect("failed to parse as i64"))),
		}
	}
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Instruction {
	Inp(Value),
	Add(Value, Value),
	Mul(Value, Value),
	Div(Value, Value),
	Mod(Value, Value),
	Eql(Value, Value),
}

impl core::str::FromStr for Instruction {
	type Err = core::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(' ');

		match (split.next(), split.next(), split.next()) {
			(Some("inp"), Some(var), None) => var.parse().map(Instruction::Inp),
			(Some("add"), Some(a), Some(b)) => match (a.parse(), b.parse()) {
				(Ok(a), Ok(b)) => Ok(Instruction::Add(a, b)),
				_ => unreachable!(),
			},
			(Some("mul"), Some(a), Some(b)) => match (a.parse(), b.parse()) {
				(Ok(a), Ok(b)) => Ok(Instruction::Mul(a, b)),
				_ => unreachable!(),
			},
			(Some("div"), Some(a), Some(b)) => match (a.parse(), b.parse()) {
				(Ok(a), Ok(b)) => Ok(Instruction::Div(a, b)),
				_ => unreachable!(),
			},
			(Some("mod"), Some(a), Some(b)) => match (a.parse(), b.parse()) {
				(Ok(a), Ok(b)) => Ok(Instruction::Mod(a, b)),
				_ => unreachable!(),
			},
			(Some("eql"), Some(a), Some(b)) => match (a.parse(), b.parse()) {
				(Ok(a), Ok(b)) => Ok(Instruction::Eql(a, b)),
				_ => unreachable!(),
			},
			_ => unreachable!(),
		}
	}
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ALU {
	w: i64,
	x: i64,
	y: i64,
	z: i64,
}

// Want to be able to determine which (Instruction, affected register, affected register) => ( pairs are

// struct InstructionCache {
// 	states: HashMap<ALU, HashMap<Instruction, ALU>>,
// }

// impl InstructionCache {
// 	fn cached_eval(&mut self, alu: &mut ALU, instruction: &Instruction) {
// 		match self.states.entry((*alu)) {
// 			Entry::Occupied(mut e) => {
// 				// Already have seen this ALU state...
// 				if let Entry::Occupied(mut result) = e.get_mut().entry(*instruction) {
// 					// ... AND with this Instruction
// 					let cached_result = result.get();
// 					alu.w = cached_result.w;
// 					alu.x = cached_result.x;
// 					alu.y = cached_result.y;
// 					alu.z = cached_result.z;
// 				} else {
// 					// ... BUT NOT with this Instruction.
// 				}
// 			}
// 			Entry::Vacant(mut e) => {
// 				// Have never seen this ALU state.
// 				e.insert(HashMap::new());

// 			}
// 		}
// 	}
// }

#[derive(Clone, Debug)]
pub struct Program {
	instructions: Vec<Instruction>,
}

type Intermediate = (ALU, Program);

fn generate_valid_model_numbers() -> impl Iterator<Item = Vec<u8>> {
	(0..14).map(|_| (1..=9)).multi_cartesian_product()
}

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let instructions = input
		.lines()
		.map(str::parse)
		.filter_map(Result::ok)
		.collect();

	let program = Program { instructions };

	let alu = ALU {
		w: 0,
		x: 0,
		y: 0,
		z: 0,
	};

	Ok((alu, program))
}

impl ALU {
	fn get_value(&self, value: &Value) -> i64 {
		match value {
			Value::Number(num) => *num,
			Value::W => self.w,
			Value::X => self.x,
			Value::Y => self.y,
			Value::Z => self.z,
		}
	}

	fn deref_destination(&mut self, destination: &Value) -> &mut i64 {
		match destination {
			Value::W => &mut self.w,
			Value::X => &mut self.x,
			Value::Y => &mut self.y,
			Value::Z => &mut self.z,
			_ => panic!("attempted to dereference a number as a destination"),
		}
	}

	fn inp(&mut self, destination: &Value, digits: &Vec<u8>, index: &mut usize) {
		let digit = digits[*index];
		*index += 1;
		*self.deref_destination(destination) = i64::from(digit);
	}

	fn add(&mut self, a: &Value, b: &Value) {
		let a_value = self.get_value(a);
		let b_value = self.get_value(b);
		*self.deref_destination(a) = a_value + b_value;
	}

	fn mul(&mut self, a: &Value, b: &Value) {
		let a_value = self.get_value(a);
		let b_value = self.get_value(b);
		*self.deref_destination(a) = a_value * b_value;
	}

	fn div(&mut self, a: &Value, b: &Value) {
		let a_value = self.get_value(a);
		let b_value = self.get_value(b);
		*self.deref_destination(a) = a_value / b_value;
	}

	fn modu(&mut self, a: &Value, b: &Value) {
		let a_value = self.get_value(a);
		let b_value = self.get_value(b);
		*self.deref_destination(a) = a_value % b_value;
	}

	fn eql(&mut self, a: &Value, b: &Value) {
		let a_value = self.get_value(a);
		let b_value = self.get_value(b);
		*self.deref_destination(a) = if a_value == b_value { 1 } else { 0 };
	}

	fn eval(&mut self, digits: &Vec<u8>, instructions: &[Instruction]) -> (i64, i64, i64, i64) {
		let mut idx = 0;

		for instruction in instructions {
			match instruction {
				Instruction::Inp(dest) => self.inp(&dest, digits, &mut idx),
				Instruction::Add(a, b) => self.add(a, b),
				Instruction::Mul(a, b) => self.mul(a, b),
				Instruction::Div(a, b) => self.div(a, b),
				Instruction::Mod(a, b) => self.modu(a, b),
				Instruction::Eql(a, b) => self.eql(a, b),
			}
		}

		(self.w, self.x, self.y, self.z)
	}
}

type Solution = i64;

fn model_number_to_i64(digits: &[u8]) -> i64 {
	let length = digits.len();

	digits.iter().enumerate().fold(0_i64, |sum, (idx, digit)| {
		let power: u32 = length as u32 - 1 - idx as u32;
		sum + i64::from(*digit) * 10_i64.pow(power)
	})
}

#[test]
fn model_number_convert_12345678954321() {
	assert_eq!(
		model_number_to_i64(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 5, 4, 3, 2, 1]),
		12345678954321_i64
	);
}

pub fn part_one((alu, program): &Intermediate) -> Option<Solution> {
	let mut alu: ALU = alu.clone();

	let mut counter: usize = 0;

	const COUNTER_MAX_EST: usize = 9_usize.pow(14);

	let earliest_valid = generate_valid_model_numbers()
		.inspect(|_n| {
			counter += 1;
			if counter % 1_000_000 == 0 {
				println!(
					"Processed {} of {} ({}%)",
					counter,
					COUNTER_MAX_EST,
					(counter as f64 / COUNTER_MAX_EST as f64) * 100.0
				);
			}
		})
		.map(|model_number| {
			(
				model_number.clone(),
				alu.eval(&model_number, &program.instructions),
			)
		})
		.find_map(|(model_number, result)| {
			if result.3 == 1 {
				Some(model_number)
			} else {
				None
			}
		});

	earliest_valid.map(|earliest_valid| model_number_to_i64(&earliest_valid))
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
