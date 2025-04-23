use itertools::Itertools;

pub type Intermediate = Vec<char>;
pub type Output = usize;

/// Parses the input data and returns an `Intermediate` type.
///
/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	match input.lines().next() {
		Some(line) => Ok(line.chars().collect()),
		_ => panic!("no input given"),
	}
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_one(chars: &Intermediate) -> Option<Output> {
	Some(
		chars
			.iter()
			.circular_tuple_windows()
			.filter(|(a, b)| a == b)
			.map(|(a, _b)| a)
			.filter_map(|c| c.to_digit(10))
			.sum::<u32>() as usize,
	)
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_two(chars: &Intermediate) -> Option<Output> {
	let mut sum: u32 = 0;
	let halfway: usize = chars.len() / 2;
	for i in 0..halfway {
		if chars[i] == chars[halfway + i] {
			sum += chars[i].to_digit(10).unwrap();
			sum += chars[halfway + i].to_digit(10).unwrap();
		}
	}
	Some(sum as usize)
}

crate::test_example!(part_one_1122, parse, part_one, "1122", Some(3));
crate::test_example!(part_one_1111, parse, part_one, "1111", Some(4));
crate::test_example!(part_one_1234, parse, part_one, "1234", Some(0));
crate::test_example!(part_one_91212129, parse, part_one, "91212129", Some(9));

crate::test_example!(part_two_1212, parse, part_two, "1212", Some(6));
crate::test_example!(part_two_1221, parse, part_two, "1221", Some(0));
crate::test_example!(part_two_123425, parse, part_two, "123425", Some(4));
crate::test_example!(part_two_123123, parse, part_two, "123123", Some(12));
crate::test_example!(part_two_12131415, parse, part_two, "12131415", Some(4));

crate::generate_solver!(solve, =>, self);
