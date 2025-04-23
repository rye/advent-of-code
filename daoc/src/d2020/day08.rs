use core::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
	Acc(isize),
	Jmp(isize),
	Nop(isize),
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, ()> {
		let parameter: isize = s.split(' ').nth(1).unwrap().parse::<isize>().unwrap();

		match &s[0..3] {
			"nop" => Ok(Instruction::Nop(parameter)),
			"acc" => Ok(Instruction::Acc(parameter)),
			"jmp" => Ok(Instruction::Jmp(parameter)),
			_ => todo!(),
		}
	}
}

#[derive(Debug)]
pub enum ExecutionResult<T> {
	Normal(T),
	Looped(T),
}

impl<T> ExecutionResult<T> {
	pub fn unwrap(self) -> T {
		match self {
			Self::Normal(inner) | Self::Looped(inner) => inner,
		}
	}
}

pub fn execute_program(program: &VecDeque<Instruction>) -> ExecutionResult<isize> {
	let mut visits: VecDeque<usize> = VecDeque::new();
	visits.resize(program.len(), 0_usize);

	let mut accumulator = 0_isize;
	let mut head = 0_usize;

	loop {
		if head == program.len() {
			break ExecutionResult::Normal(accumulator);
		} else if visits[head] != 0_usize {
			break ExecutionResult::Looped(accumulator);
		}

		visits[head] += 1_usize;

		let instruction = &program[head];

		match instruction {
			Instruction::Acc(x) => {
				accumulator += x;
				head += 1;
			}
			Instruction::Jmp(ofs) => {
				head =
					usize::try_from((isize::try_from(head).unwrap()).checked_add(*ofs).unwrap()).unwrap();
			}
			Instruction::Nop(_) => {
				head += 1;
			}
		}
	}
}

pub type Intermediate = VecDeque<Instruction>;
pub type Output = isize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		data
			.lines()
			.map(|ln| ln.parse().expect("failed to parse instruction"))
			.collect(),
	)
}

pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	Some(execute_program(instructions).unwrap())
}

pub fn part_two(instructions: &Intermediate) -> Option<Output> {
	let mut accumulator = None;

	for position in 0..instructions.len() {
		if let Instruction::Acc(_x) = instructions[position] {
			continue;
		}

		let mut program = instructions.clone();

		match program[position] {
			Instruction::Jmp(ofs) => program[position] = Instruction::Nop(ofs),
			Instruction::Nop(par) => program[position] = Instruction::Jmp(par),
			Instruction::Acc(_) => unreachable!(),
		}

		let result = execute_program(&program);

		if let ExecutionResult::Normal(acc) = result {
			accumulator = Some(acc);
			break;
		}
	}

	accumulator
}

daocutil::generate_solver!(solve, =>, self);
