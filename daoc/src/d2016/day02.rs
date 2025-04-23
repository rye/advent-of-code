use anyhow::anyhow;
use phf::phf_map;

pub type Intermediate = Vec<Vec<Instruction>>;
pub type Output = String;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input
		.lines()
		.map(|line| {
			line
				.chars()
				.map(Instruction::from_char)
				.collect::<Option<Vec<Instruction>>>()
				.ok_or(anyhow!("invalid input"))
		})
		.collect::<anyhow::Result<Vec<Vec<_>>>>()
}

#[must_use]
pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	let result = instructions.iter().fold(
		(String::new(), None),
		|(mut code, last_pos): (String, Option<char>), instructions| {
			let last_pos = instructions
				.iter()
				.fold(last_pos, |number, instruction| {
					let number = number.unwrap_or('5');
					let new_number = apply_map_to_number(&PART_ONE_MAP, instruction, number);
					Some(new_number)
				})
				.expect("expected to step at least once");

			code.push(last_pos);
			(code, Some(last_pos))
		},
	);

	Some(result.0)
}

#[must_use]
pub fn part_two(instructions: &Intermediate) -> Option<Output> {
	let result = instructions.iter().fold(
		(String::new(), None),
		|(mut code, last_pos): (String, Option<char>), instructions| {
			let last_pos = instructions
				.iter()
				.fold(last_pos, |number, instruction| {
					let number = number.unwrap_or('5');
					let new_number = apply_map_to_number(&PART_TWO_MAP, instruction, number);
					Some(new_number)
				})
				.expect("expected to step at least once");

			code.push(last_pos);
			(code, Some(last_pos))
		},
	);

	Some(result.0)
}

#[derive(Debug)]
pub enum Instruction {
	Up,
	Down,
	Left,
	Right,
}

impl Instruction {
	fn from_char(char: char) -> Option<Self> {
		Some(match char {
			'U' => Self::Up,
			'D' => Self::Down,
			'L' => Self::Left,
			'R' => Self::Right,
			_ => return None,
		})
	}

	fn to_key(&self, loc: char) -> [u8; 2] {
		#[allow(clippy::cast_possible_truncation)]
		[
			match self {
				Self::Up => 0,
				Self::Down => 1,
				Self::Left => 2,
				Self::Right => 3,
			},
			loc.to_digit(16).expect("expected a digit") as u8,
		]
	}
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("../../inputs/2016/examples/day02.example.in.txt"),
	Some("1985".to_string())
);

static PART_ONE_MAP: phf::Map<[u8; 2], char> = phf_map! {
	// 0 = Up
	[0, 1] => '1',
	[0, 2] => '2',
	[0, 3] => '3',
	[0, 4] => '1',
	[0, 5] => '2',
	[0, 6] => '3',
	[0, 7] => '4',
	[0, 8] => '5',
	[0, 9] => '6',

	// 1 = Down
	[1, 1] => '4',
	[1, 2] => '5',
	[1, 3] => '6',
	[1, 4] => '7',
	[1, 5] => '8',
	[1, 6] => '9',
	[1, 7] => '7',
	[1, 8] => '8',
	[1, 9] => '9',

	// 2 = Left
	[2, 1] => '1',
	[2, 2] => '1',
	[2, 3] => '2',
	[2, 4] => '4',
	[2, 5] => '4',
	[2, 6] => '5',
	[2, 7] => '7',
	[2, 8] => '7',
	[2, 9] => '8',

	// 3 = Right
	[3, 1] => '2',
	[3, 2] => '3',
	[3, 3] => '3',
	[3, 4] => '5',
	[3, 5] => '6',
	[3, 6] => '6',
	[3, 7] => '8',
	[3, 8] => '9',
	[3, 9] => '9',
};

daocutil::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("../../inputs/2016/examples/day02.example.in.txt"),
	Some("5DB3".to_string())
);

static PART_TWO_MAP: phf::Map<[u8; 2], char> = phf_map! {
	// 0 = Up
	[0, 0x1] => '1',
	[0, 0x2] => '2',
	[0, 0x4] => '4',
	[0, 0x5] => '5',
	[0, 0x9] => '9',
	[0, 0x3] => '1',
	[0, 0x6] => '2',
	[0, 0x7] => '3',
	[0, 0x8] => '4',
	[0, 0xa] => '6',
	[0, 0xb] => '7',
	[0, 0xc] => '8',
	[0, 0xd] => 'b',

	// 1 = Down
	[1, 0x5] => '5',
	[1, 0xa] => 'A',
	[1, 0xd] => 'D',
	[1, 0xc] => 'C',
	[1, 0x9] => '9',
	[1, 0xb] => 'D',
	[1, 0x6] => 'A',
	[1, 0x7] => 'B',
	[1, 0x8] => 'C',
	[1, 0x2] => '6',
	[1, 0x3] => '7',
	[1, 0x4] => '8',
	[1, 0x1] => '3',

	// 2 = Left
	[2, 0x1] => '1',
	[2, 0x2] => '2',
	[2, 0x5] => '5',
	[2, 0xa] => 'A',
	[2, 0xd] => 'D',
	[2, 0x6] => '5',
	[2, 0x3] => '2',
	[2, 0x7] => '6',
	[2, 0xb] => 'A',
	[2, 0x4] => '3',
	[2, 0x8] => '7',
	[2, 0xc] => 'B',
	[2, 0x9] => '8',

	// 3 = Right
	[3, 0x1] => '1',
	[3, 0x4] => '4',
	[3, 0x9] => '9',
	[3, 0xc] => 'C',
	[3, 0xd] => 'D',
	[3, 0x8] => '9',
	[3, 0x3] => '4',
	[3, 0x7] => '8',
	[3, 0xb] => 'C',
	[3, 0x2] => '3',
	[3, 0x6] => '7',
	[3, 0xa] => 'B',
	[3, 0x5] => '6',
};

fn apply_map_to_number(
	map: &phf::Map<[u8; 2], char>,
	instruction: &Instruction,
	number: char,
) -> char {
	let key = instruction.to_key(number);
	*map.get(&key).expect("expected a key")
}

daocutil::generate_solver!(solve, =>, self);
