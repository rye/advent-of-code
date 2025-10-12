pub type Intermediate<'i> = Vec<&'i str>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate<'_>> {
	Ok(input.lines().collect())
}

const NUMBER_SUBSTITUTIONS: [(&str, &str); 9] = [
	("one", "1"),
	("two", "2"),
	("three", "3"),
	("four", "4"),
	("five", "5"),
	("six", "6"),
	("seven", "7"),
	("eight", "8"),
	("nine", "9"),
];

fn get_calibration_value(str: &str, consider_spelling: bool) -> u32 {
	let mut input = str.to_string();

	if consider_spelling {
		let mut temp = String::new();
		let mut idx = 0;
		// do something
		while idx < input.len() {
			if let Some((_word, substitution)) = NUMBER_SUBSTITUTIONS
				.iter()
				.find(|(word, _substitution)| input[idx..].starts_with(word))
			{
				temp.push_str(substitution);
				idx += substitution.len();
			} else {
				temp.push_str(&input[idx..=idx]);
				idx += 1;
			}
		}

		input = temp;
	}

	format!(
		"{}{}",
		input.chars().find(char::is_ascii_digit).unwrap(),
		input.chars().rfind(char::is_ascii_digit).unwrap()
	)
	.parse()
	.unwrap()
}

#[must_use]
pub fn part_one(terminal_digits: &Intermediate) -> Option<Output> {
	Some(
		terminal_digits
			.iter()
			.map(|str| get_calibration_value(str, false))
			.sum(),
	)
}

#[test]
fn get_calibration_value_initial() {
	assert_eq!(12, get_calibration_value("1abc2", false));
	assert_eq!(38, get_calibration_value("pqr3stu8vwx", false));
	assert_eq!(15, get_calibration_value("a1b2c3d4e5f", false));
	assert_eq!(77, get_calibration_value("treb7uchet", false));
}

crate::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day01-part1.example.in.txt"),
	Some(142)
);

#[must_use]
pub fn part_two(terminal_digits: &Intermediate) -> Option<Output> {
	Some(
		terminal_digits
			.iter()
			.map(|str| get_calibration_value(str, true))
			.sum(),
	)
}

#[test]
fn get_calibration_value_fancy() {
	assert_eq!(29, get_calibration_value("two1nine", true));
	assert_eq!(83, get_calibration_value("eightwothree", true));
	assert_eq!(13, get_calibration_value("abcone2threexyz", true));
	assert_eq!(24, get_calibration_value("xtwone3four", true));
	assert_eq!(42, get_calibration_value("4nineeightseven2", true));
	assert_eq!(14, get_calibration_value("zoneight234", true));
	assert_eq!(76, get_calibration_value("7pqrstsixteen", true));
}

crate::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("examples/day01-part2.example.in.txt"),
	Some(281)
);

crate::generate_solver!(solve, =>, self);
