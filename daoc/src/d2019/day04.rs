pub type Intermediate = (u32, u32);
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let bounds: Vec<u32> = input
		.lines()
		.flat_map(|line| -> Vec<u32> {
			line
				.split('-')
				.map(|num| -> u32 { num.parse::<u32>().expect("malformed bound") })
				.collect()
		})
		.collect();

	assert_eq!(bounds.len(), 2);
	assert!(bounds[0] < bounds[1]);

	let starting_bound = bounds[0];
	let ending_bound = bounds[1];

	Ok((starting_bound, ending_bound))
}

#[must_use]
pub fn part_one((starting_bound, ending_bound): &Intermediate) -> Option<Output> {
	let valid_passwords = (*starting_bound..=*ending_bound)
		.filter(|number| {
			let password = Password(*number);
			password.is_monotonically_increasing() && password.has_at_least_two_adjacent_digits()
		})
		.collect::<Vec<u32>>();

	Some(valid_passwords.len())
}

#[must_use]
pub fn part_two((starting_bound, ending_bound): &Intermediate) -> Option<Output> {
	let valid_passwords = (*starting_bound..=*ending_bound)
		.filter(|number| {
			let password = Password(*number);
			password.is_monotonically_increasing() && password.has_exactly_two_adjacent_digits()
		})
		.collect::<Vec<u32>>();

	Some(valid_passwords.len())
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
struct Password(u32);

#[derive(Debug, Clone)]
struct Digits<'i, I> {
	number: &'i I,
	pow: i32,
}

impl<'i> Password {
	fn digits(&'i self) -> Digits<'i, u32> {
		Digits {
			number: &self.0,
			#[allow(clippy::cast_possible_truncation)]
			pow: f64::from(self.0).log10() as i32,
		}
	}

	fn is_monotonically_increasing(self) -> bool {
		let digits: Vec<u32> = self.digits().collect();
		let sorted: Vec<u32> = {
			let mut tmp = digits.clone();
			tmp.sort_unstable();
			tmp
		};
		sorted == digits
	}

	fn has_at_least_two_adjacent_digits(self) -> bool {
		let mut digits: Vec<u32> = self.digits().collect();
		let starting_len = digits.len();
		digits.dedup();
		let final_len = digits.len();
		starting_len - final_len >= 1
	}

	fn has_exactly_two_adjacent_digits(self) -> bool {
		let digits: Vec<u32> = self.digits().collect();

		let counts: Vec<u32> = (0..=9)
			.map(|cur| -> u32 { digits.iter().map(|digit| u32::from(*digit == cur)).sum() })
			.collect();

		counts.contains(&2)
	}
}

impl<'i> Iterator for Digits<'i, u32> {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		let number: &'i u32 = self.number;
		let power: i32 = self.pow;

		if self.pow >= 0 {
			let current: u32 = (number / 10_u32.pow(u32::try_from(power).unwrap())) % 10_u32;
			self.pow -= 1;
			Some(current)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Password;

	#[test]
	fn digits_0() {
		let password: Password = Password(0_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), None);
	}

	#[test]
	fn digits_1() {
		let password: Password = Password(1_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), Some(1_u32));
		assert_eq!(digits.next(), None);
	}

	#[test]
	fn digits_123() {
		let password: Password = Password(123_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), Some(1_u32));
		assert_eq!(digits.next(), Some(2_u32));
		assert_eq!(digits.next(), Some(3_u32));
		assert_eq!(digits.next(), None);
	}

	#[test]
	fn digits_123456() {
		let password: Password = Password(123_456_u32);
		let mut digits = password.digits();
		assert_eq!(digits.next(), Some(1_u32));
		assert_eq!(digits.next(), Some(2_u32));
		assert_eq!(digits.next(), Some(3_u32));
		assert_eq!(digits.next(), Some(4_u32));
		assert_eq!(digits.next(), Some(5_u32));
		assert_eq!(digits.next(), Some(6_u32));
		assert_eq!(digits.next(), None);
	}
}

daocutil::generate_solver!(solve, =>, self);
