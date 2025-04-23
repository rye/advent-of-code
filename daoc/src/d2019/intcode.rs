use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum Opcode {
	Add,
	Mul,
	Input,
	Output,
	JumpIfTrue,
	JumpIfFalse,
	LessThan,
	Equals,
	Halt,
}

impl From<i32> for Opcode {
	fn from(raw: i32) -> Opcode {
		use Opcode::{Add, Equals, Halt, Input, JumpIfFalse, JumpIfTrue, LessThan, Mul, Output};

		// We should not get values >= 100.
		assert_eq!(raw % 100, raw);

		match raw {
			1 => Add,
			2 => Mul,
			3 => Input,
			4 => Output,
			5 => JumpIfTrue,
			6 => JumpIfFalse,
			7 => LessThan,
			8 => Equals,
			99 => Halt,
			_ => panic!("unknown opcode {raw}"),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum ParameterMode {
	Position,
	Immediate,
}

impl From<i32> for ParameterMode {
	fn from(raw: i32) -> ParameterMode {
		match raw {
			0 => Self::Position,
			1 => Self::Immediate,
			_ => unimplemented!(),
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
	opcode: Opcode,
	parameter_modes: (ParameterMode, ParameterMode, ParameterMode),
}

impl From<i32> for Instruction {
	fn from(raw: i32) -> Instruction {
		let opcode = Opcode::from(raw % 100);
		let mode_0: ParameterMode = ((raw / 100) % 10).into();
		let mode_1: ParameterMode = ((raw / 1000) % 10).into();
		let mode_2: ParameterMode = ((raw / 10000) % 10).into();
		let parameter_modes = (mode_0, mode_1, mode_2);

		Instruction {
			opcode,
			parameter_modes,
		}
	}
}

#[test]
fn instruction_example() {
	assert_eq!(
		Instruction {
			opcode: Opcode::Mul,
			parameter_modes: (
				ParameterMode::Position,
				ParameterMode::Immediate,
				ParameterMode::Position
			)
		},
		1002_i32.into()
	);
}

#[derive(Debug)]
pub struct Intcode {
	inner: Vec<i32>,
	head: usize,
	interactive: bool,
	input: VecDeque<i32>,
	output: VecDeque<i32>,
	did_halt: bool,
}

impl Intcode {
	pub fn new(inner: Vec<i32>, head: usize) -> Self {
		let (input, output) = (VecDeque::new(), VecDeque::new());

		Self {
			inner,
			head,
			interactive: true,
			input,
			output,
			did_halt: false,
		}
	}

	pub fn from_data(inner: Vec<i32>) -> Self {
		Self::new(inner, 0_usize)
	}

	pub fn input(&mut self, value: i32) {
		self.input.push_back(value);

		if self.interactive {
			self.interactive = false;
		}
	}

	pub fn output(&mut self) -> Option<i32> {
		self.output.pop_front()
	}

	fn resolve_parameter(&self, value: i32, mode: &ParameterMode) -> i32 {
		match mode {
			ParameterMode::Position => self.inner[usize::try_from(value).expect("what")],
			ParameterMode::Immediate => value,
		}
	}

	pub fn did_halt(&self) -> bool {
		self.did_halt
	}

	fn eval_add(&mut self, parameter_modes: &(ParameterMode, ParameterMode, ParameterMode)) {
		let param_a = self.inner[self.head + 1];
		let param_b = self.inner[self.head + 2];
		let param_outpos = self.inner[self.head + 3];

		// Ensure no illegal output parameter in immediate mode.
		assert!(parameter_modes.2 != ParameterMode::Immediate);

		let a = self.resolve_parameter(param_a, &parameter_modes.0);
		let b = self.resolve_parameter(param_b, &parameter_modes.1);

		self.inner[usize::try_from(param_outpos)
			.expect("failed to convert output position for add instruction")] = a + b;

		self.head += 4;
	}

	fn eval_mul(&mut self, parameter_modes: &(ParameterMode, ParameterMode, ParameterMode)) {
		let param_a = self.inner[self.head + 1];
		let param_b = self.inner[self.head + 2];
		let param_outpos = self.inner[self.head + 3];

		// Ensure no illegal output parameter in immediate mode.
		assert!(parameter_modes.2 != ParameterMode::Immediate);

		let a = self.resolve_parameter(param_a, &parameter_modes.0);
		let b = self.resolve_parameter(param_b, &parameter_modes.1);

		self.inner[usize::try_from(param_outpos)
			.expect("failed to convert output position for mul instruction")] = a * b;

		self.head += 4;
	}

	fn eval_input(&mut self) {
		let location = self.inner[self.head + 1];

		// If the user has supplied us with an input in the queue, use it.
		// Otherwise, prompt for an input.
		if self.interactive {
			use std::io::{stdin, stdout, Write};

			let mut input: String = String::new();
			print!("<= ");
			let _ = stdout().flush();

			stdin().read_line(&mut input).expect("invalid input");
			let input: i32 = input.trim_end().parse::<i32>().expect("invalid input");
			self.inner[usize::try_from(location).unwrap()] = input;
		} else if let Some(input) = self.input.pop_front() {
			self.inner[usize::try_from(location).unwrap()] = input;
		} else {
			panic!("Attempted to take input in non-interactive mode without anything in input buffer");
		}

		self.head += 2;
	}

	fn eval_output(&mut self, parameter_modes: &(ParameterMode, ParameterMode, ParameterMode)) {
		let param = self.inner[self.head + 1];

		let value = self.resolve_parameter(param, &parameter_modes.0);

		if self.interactive {
			println!("=> {value}");
		} else {
			self.output.push_back(value);
		}

		self.head += 2;
	}

	fn eval_jump_if_true(&mut self, parameter_modes: &(ParameterMode, ParameterMode, ParameterMode)) {
		let param_a = self.inner[self.head + 1];
		let param_b = self.inner[self.head + 2];

		let a = self.resolve_parameter(param_a, &parameter_modes.0);
		let b = self.resolve_parameter(param_b, &parameter_modes.1);

		if a != 0 {
			self.head = usize::try_from(b).unwrap();
		} else {
			self.head += 3;
		}
	}

	fn eval_jump_if_false(
		&mut self,
		parameter_modes: &(ParameterMode, ParameterMode, ParameterMode),
	) {
		let param_a = self.inner[self.head + 1];
		let param_b = self.inner[self.head + 2];

		let a = self.resolve_parameter(param_a, &parameter_modes.0);
		let b = self.resolve_parameter(param_b, &parameter_modes.1);

		if a == 0 {
			self.head = usize::try_from(b).unwrap();
		} else {
			self.head += 3;
		}
	}

	fn eval_less_than(&mut self, parameter_modes: &(ParameterMode, ParameterMode, ParameterMode)) {
		let param_a = self.inner[self.head + 1];
		let param_b = self.inner[self.head + 2];
		let param_outpos = self.inner[self.head + 3];

		let a = self.resolve_parameter(param_a, &parameter_modes.0);
		let b = self.resolve_parameter(param_b, &parameter_modes.1);

		self.inner[usize::try_from(param_outpos).unwrap()] = (a < b).into();
		self.head += 4;
	}

	fn eval_equals(&mut self, parameter_modes: &(ParameterMode, ParameterMode, ParameterMode)) {
		let param_a = self.inner[self.head + 1];
		let param_b = self.inner[self.head + 2];
		let param_outpos = self.inner[self.head + 3];

		let a = self.resolve_parameter(param_a, &parameter_modes.0);
		let b = self.resolve_parameter(param_b, &parameter_modes.1);

		self.inner[usize::try_from(param_outpos).unwrap()] = (a == b).into();
		self.head += 4;
	}

	pub fn step(&mut self) -> Option<()> {
		// Fetch
		let instruction: Instruction = Instruction::from(self.inner[self.head]);
		let opcode: Opcode = instruction.opcode;
		let parameter_modes: (ParameterMode, ParameterMode, ParameterMode) =
			instruction.parameter_modes;

		match opcode {
			Opcode::Add => {
				self.eval_add(&parameter_modes);
				Some(())
			}

			Opcode::Mul => {
				self.eval_mul(&parameter_modes);
				Some(())
			}

			Opcode::Input => {
				self.eval_input();
				Some(())
			}

			Opcode::Output => {
				self.eval_output(&parameter_modes);
				Some(())
			}

			Opcode::JumpIfTrue => {
				self.eval_jump_if_true(&parameter_modes);
				Some(())
			}

			Opcode::JumpIfFalse => {
				self.eval_jump_if_false(&parameter_modes);
				Some(())
			}

			Opcode::LessThan => {
				self.eval_less_than(&parameter_modes);
				Some(())
			}

			Opcode::Equals => {
				self.eval_equals(&parameter_modes);
				Some(())
			}

			Opcode::Halt => {
				self.did_halt = true;
				None
			}
		}
	}

	#[must_use]
	pub fn run(mut self) -> Self {
		loop {
			if self.step().is_none() {
				break self;
			}
		}
	}

	#[must_use]
	pub fn run_til_next_output(&mut self) -> &mut Self {
		loop {
			if self.step().is_none() {
				break self;
			}

			if !self.output.is_empty() {
				break self;
			}

			if self.did_halt {
				println!("Did halt... before output was produced!");
				break self;
			}
		}
	}

	pub fn data(&self) -> &Vec<i32> {
		&self.inner
	}
}

impl From<Vec<i32>> for Intcode {
	fn from(program: Vec<i32>) -> Self {
		Self::from_data(program)
	}
}

#[cfg(test)]
mod tests {
	use super::{Instruction, Intcode};

	#[cfg(test)]
	mod instruction {
		use super::super::{Opcode, ParameterMode};
		use super::Instruction;

		#[test]
		fn standard_add_correct() {
			assert_eq!(
				Instruction::from(1),
				Instruction {
					opcode: Opcode::Add,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			);
		}

		#[test]
		fn standard_mul_correct() {
			assert_eq!(
				Instruction::from(2),
				Instruction {
					opcode: Opcode::Mul,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			);
		}

		#[test]
		fn standard_input_correct() {
			assert_eq!(
				Instruction::from(3),
				Instruction {
					opcode: Opcode::Input,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			);
		}

		#[test]
		fn standard_output_correct() {
			assert_eq!(
				Instruction::from(4),
				Instruction {
					opcode: Opcode::Output,
					parameter_modes: (
						ParameterMode::Position,
						ParameterMode::Position,
						ParameterMode::Position
					)
				}
			);
		}
	}

	#[test]
	fn pgm_input_output() {
		let mut program: Intcode = Intcode::from(vec![3, 0, 4, 0, 99]);

		// First, we should be able to set the input.
		program.input(573);
		assert_eq!(program.data(), &vec![3, 0, 4, 0, 99]);

		// After one step, the value we input should be stored in the data, and we
		// shouldn't have any output.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![573, 0, 4, 0, 99]);
		assert_eq!(program.output(), None);

		// Stepping once more, we should now have an output.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![573, 0, 4, 0, 99]);
		assert_eq!(program.output(), Some(573));

		// Stepping once again should halt the program.
		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_add_immediate() {
		let mut program: Intcode = Intcode::from(vec![1101, 100, -1, 4, 0]);

		assert_eq!(program.data(), &vec![1101, 100, -1, 4, 0]);

		// Stepping, opcode 1101 should add the 100 and -1 and store the result, 99,
		// in position 4.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![1101, 100, -1, 4, 99]);

		// Next step should terminate the program.
		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_multiply_immediate() {
		let mut program: Intcode = Intcode::from(vec![1002, 4, 3, 4, 33]);

		assert_eq!(program.data(), &vec![1002, 4, 3, 4, 33]);

		// Stepping, opcode 1002 should multiply the value at address 4
		// by the literal value 3 and store it in position 4, so it effectively
		// multiplies the last position by 3, setting it to the halt instruction.
		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![1002, 4, 3, 4, 99]);

		// Next step should terminate the program.
		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_position_input_8_equal_8() {
		let input = 8;
		let expected_output = (input == 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, input, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_position_input_7_not_equal_8() {
		let input = 7;
		let expected_output = (input == 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, input, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_position_input_8_lt_8() {
		let input = 8;
		let expected_output = (input < 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 9, 7, 9, 10, 9, 4, 9, 99, input, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_position_input_7_lt_8() {
		let input = 7;
		let expected_output = (input < 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 9, 7, 9, 10, 9, 4, 9, 99, input, 8]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, expected_output, 8]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_immediate_input_8_equal_8() {
		let input = 8;
		let expected_output = (input == 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 3, 1108, input, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1108, expected_output, 8, 3, 4, 3, 99]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1108, expected_output, 8, 3, 4, 3, 99]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_immediate_input_7_equal_8() {
		let input = 7;
		let expected_output = (input == 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 3, 1108, input, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1108, expected_output, 8, 3, 4, 3, 99]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1108, expected_output, 8, 3, 4, 3, 99]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_immediate_input_8_lt_8() {
		let input = 8;
		let expected_output = (input < 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 3, 1107, input, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1107, expected_output, 8, 3, 4, 3, 99]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1107, expected_output, 8, 3, 4, 3, 99]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_immediate_input_7_lt_8() {
		let input = 7;
		let expected_output = (input < 8).into();
		let mut program: Intcode = Intcode::from(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
		program.input(input);

		assert_eq!(program.data(), &vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(program.data(), &vec![3, 3, 1107, input, 8, 3, 4, 3, 99]);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1107, expected_output, 8, 3, 4, 3, 99]
		);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1107, expected_output, 8, 3, 4, 3, 99]
		);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_position_input_0_eq_0() {
		let input = 0;
		let expected_output = (input != 0).into();
		let mut program: Intcode = Intcode::from(vec![
			3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
		]);
		program.input(input);

		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
		);
		assert_eq!(program.head, 0_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, input, 0, 1, 9]
		);
		assert_eq!(program.head, 2_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, input, 0, 1, 9]
		);
		assert_eq!(program.head, 9_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, input, 0, 1, 9]
		);
		assert_eq!(program.head, 11_usize);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_position_input_1_neq_0() {
		let input = 1;
		let expected_output = (input != 0).into();
		let mut program: Intcode = Intcode::from(vec![
			3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
		]);
		program.input(input);

		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]
		);
		assert_eq!(program.head, 0_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, input, 0, 1, 9]
		);
		assert_eq!(program.head, 2_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, input, 0, 1, 9]
		);
		assert_eq!(program.head, 5_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, input, 1, 1, 9]
		);
		assert_eq!(program.head, 9_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, input, 1, 1, 9]
		);
		assert_eq!(program.head, 11_usize);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_immediate_0_eq_0() {
		let input = 0;
		let expected_output = (input != 0).into();
		let mut program: Intcode = Intcode::from(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
		program.input(input);

		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
		);
		assert_eq!(program.head, 0_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, input, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
		);
		assert_eq!(program.head, 2_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, input, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
		);
		assert_eq!(program.head, 5_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, input, 9, 1101, 0, 0, 12, 4, 12, 99, 0]
		);
		assert_eq!(program.head, 9_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, input, 9, 1101, 0, 0, 12, 4, 12, 99, 0]
		);
		assert_eq!(program.head, 11_usize);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_immediate_1_neq_0() {
		let input = 1;
		let expected_output = (input != 0).into();
		let mut program: Intcode = Intcode::from(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
		program.input(input);

		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
		);
		assert_eq!(program.head, 0_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, input, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
		);
		assert_eq!(program.head, 2_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, input, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
		);
		assert_eq!(program.head, 9_usize);

		assert_eq!(program.step(), Some(()));
		assert_eq!(
			program.data(),
			&vec![3, 3, 1105, input, 9, 1101, 0, 0, 12, 4, 12, 99, 1]
		);
		assert_eq!(program.head, 11_usize);
		assert_eq!(program.output(), Some(expected_output));

		assert_eq!(program.step(), None);
	}

	#[test]
	fn pgm_12() {
		let mut program: Intcode = Intcode::from(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
		assert_eq!(
			program.data(),
			&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
		);
		program.step();
		assert_eq!(
			program.data(),
			&vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);
		program.step();
		assert_eq!(
			program.data(),
			&vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
		);
	}

	#[test]
	fn pgm_5_a() {
		let program: Intcode = Intcode::from(vec![1, 0, 0, 0, 99]);
		assert_eq!(program.run().data(), &vec![2, 0, 0, 0, 99]);
	}

	#[test]
	fn pgm_5_b() {
		let program: Intcode = Intcode::from(vec![2, 3, 0, 3, 99]);
		assert_eq!(program.run().data(), &vec![2, 3, 0, 6, 99]);
	}

	#[test]
	fn pgm_6() {
		let program: Intcode = Intcode::from(vec![2, 4, 4, 5, 99, 0]);
		assert_eq!(program.run().data(), &vec![2, 4, 4, 5, 99, 9801]);
	}

	#[test]
	fn pgm_9() {
		let program: Intcode = Intcode::from(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
		assert_eq!(program.run().data(), &vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
	}
}
