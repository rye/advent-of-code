use std::str::FromStr;

pub struct Rule {
	count_range: (usize, usize),
	character: char,
}

impl FromStr for Rule {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, ()> {
		let numbers = s.split(' ').next().unwrap();
		let start = numbers.split('-').next().unwrap();
		let end = numbers.split('-').nth(1).unwrap();

		let a = start.parse::<usize>().unwrap();
		let b = end.parse::<usize>().unwrap();

		let character = s.split(' ').nth(1).unwrap();
		let character = character.chars().next().unwrap();

		Ok(Self {
			count_range: (a, b),
			character,
		})
	}
}

pub fn validate_password(rule: &Rule, password: &str) -> bool {
	let count_range = rule.count_range;
	let character = rule.character;

	let chars: Vec<char> = password.chars().collect();
	let n_matches = &chars
		.iter()
		.filter(|c| *c == &character)
		.collect::<Vec<&char>>()
		.len();

	count_range.0 <= *n_matches && *n_matches <= count_range.1
}

pub fn validate_password_two(rule: &Rule, password: &str) -> bool {
	let count_range = rule.count_range;
	let character = rule.character;

	let chars: Vec<char> = password.chars().collect();

	(chars[count_range.0 - 1] == character) ^ (chars[count_range.1 - 1] == character)
}

pub type Intermediate = Vec<(Rule, String)>;
pub type Output = usize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		data
			.lines()
			.map(|s| {
				let rule = s.split(": ").next().unwrap().parse::<Rule>().unwrap();
				let password = s.split(": ").nth(1).unwrap().to_string();

				(rule, password)
			})
			.collect(),
	)
}

pub fn part_one(rules: &Intermediate) -> Option<Output> {
	Some(
		rules
			.iter()
			.filter(|(rule, password)| validate_password(rule, password))
			.count(),
	)
}

pub fn part_two(rules: &Intermediate) -> Option<Output> {
	Some(
		rules
			.iter()
			.filter(|(rule, password)| validate_password_two(rule, password))
			.count(),
	)
}

#[cfg(test)]
mod tests;

daocutil::generate_solver!(solve, =>, self);
