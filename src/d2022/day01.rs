use {
	core::{num::ParseIntError, str::FromStr},
	std::collections::BTreeSet,
};

#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Elf {
	snacks: Vec<u32>,
}

impl Elf {
	fn calorie_total(&self) -> u32 {
		self.snacks.iter().sum()
	}
}

impl FromStr for Elf {
	type Err = ParseIntError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let snacks: Vec<u32> = str
			.lines()
			.map(str::parse)
			.collect::<Result<Vec<u32>, _>>()?;

		Ok(Self { snacks })
	}
}

pub type Intermediate = Vec<Elf>;
pub type Output = u32;

/// Parse the input to the [`Intermediate`] type.
///
/// # Errors
///
/// Will return `Err` if parsing any of the lines to a `u32` should fail.
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	Ok(
		str
			.split("\n\n")
			.map(str::parse)
			.collect::<Result<Vec<Elf>, ParseIntError>>()?,
	)
}

pub fn part_one(elves: &Intermediate) -> Option<Output> {
	let elf_carrying_totals: BTreeSet<u32> = elves.iter().map(Elf::calorie_total).collect();
	elf_carrying_totals.last().copied()
}

pub fn part_two(elves: &Intermediate) -> Option<Output> {
	let elf_carrying_totals: BTreeSet<u32> = elves.iter().map(Elf::calorie_total).collect();
	Some(elf_carrying_totals.iter().rev().take(3).sum())
}

#[cfg(test)]
mod elf {
	use super::Elf;

	#[cfg(test)]
	mod calorie_total {
		use super::Elf;

		#[test]
		fn single_snack() {
			let snacks = vec![1257];
			assert_eq!(Elf { snacks }.calorie_total(), 1257);
		}

		#[test]
		fn multiple_snacks() {
			let snacks = vec![1257, 89713];
			assert_eq!(Elf { snacks }.calorie_total(), 90970);
		}
	}

	#[cfg(test)]
	mod from_str {
		use super::Elf;

		#[test]
		fn single_line() {
			let line = "1257";
			assert_eq!(line.parse(), Ok(Elf { snacks: vec![1257] }));
		}

		#[test]
		fn multi_line() {
			let line = "1257\n8817\n90361655";
			assert_eq!(
				line.parse(),
				Ok(Elf {
					snacks: vec![1257, 8817, 90_361_655]
				})
			);
		}

		#[test]
		fn malformed_line_err() {
			let line = "1257\n8817\nnot a number";
			assert!(line.parse::<Elf>().is_err());
		}
	}
}

crate::generate_solver!(solve, =>, self);
