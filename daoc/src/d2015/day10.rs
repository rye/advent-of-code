pub type Intermediate = LookAndSay;
pub type Output = usize;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct LookAndSay(String);

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.next()
			.map(str::to_string)
			.map(LookAndSay)
			.unwrap(),
	)
}

impl LookAndSay {
	fn say(&self) -> Self {
		let source: &str = &self.0;
		let mut output: String = String::new();
		let mut idx = 0_usize;

		loop {
			use core::fmt::Write;
			let cur_digit = source[idx..=idx].chars().next().unwrap();
			let offset = source[idx..].find(|c| c != cur_digit);

			let slice = if let Some(offset) = offset {
				&source[idx..(idx + offset)]
			} else {
				&source[idx..]
			};

			write!(output, "{}", slice.len()).unwrap();

			output.push(cur_digit);

			if let Some(offset) = offset {
				idx += offset;
			} else {
				break;
			}
		}

		Self(output)
	}
}

#[must_use]
pub fn part_one(seed_command: &Intermediate) -> Option<Output> {
	let mut current_las: LookAndSay = seed_command.clone();

	for _ in 0..40 {
		current_las = current_las.say();
	}

	Some(current_las.0.len())
}

#[must_use]
pub fn part_two(seed_command: &Intermediate) -> Option<Output> {
	let mut current_las: LookAndSay = seed_command.clone();

	for _ in 0..50 {
		current_las = current_las.say();
	}

	Some(current_las.0.len())
}

#[cfg(test)]
mod look_and_say {
	use super::LookAndSay;

	#[test]
	fn say_1() {
		let las = LookAndSay("1".into());
		assert_eq!(las.say(), LookAndSay("11".into()));
	}

	#[test]
	fn say_11() {
		let las = LookAndSay("11".into());
		assert_eq!(las.say(), LookAndSay("21".into()));
	}

	#[test]
	fn say_21() {
		let las = LookAndSay("21".into());
		assert_eq!(las.say(), LookAndSay("1211".into()));
	}

	#[test]
	fn say_1211() {
		let las = LookAndSay("1211".into());
		assert_eq!(las.say(), LookAndSay("111221".into()));
	}

	#[test]
	fn say_111221() {
		let las = LookAndSay("111221".into());
		assert_eq!(las.say(), LookAndSay("312211".into()));
	}
}

daocutil::generate_solver!(solve, =>, self);
