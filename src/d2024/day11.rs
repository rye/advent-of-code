use std::collections::{BTreeMap, HashMap};

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Stone {
	value: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Replacement {
	Single(Stone),
	Double(Stone, Stone),
}

impl Stone {
	fn replacements(self, cache: &mut HashMap<Stone, Replacement>) -> Replacement {
		*cache.entry(self).or_insert_with(|| {
			if self.value == 0 {
				Replacement::Single(Stone { value: 1 })
			} else if (self.value.ilog10() + 1).is_multiple_of(2) {
				let (left, right) = split_digits(self.value);
				Replacement::Double(Stone { value: left }, Stone { value: right })
			} else {
				Replacement::Single(Stone {
					value: self.value * 2024,
				})
			}
		})
	}
}

fn split_digits(num: u64) -> (u64, u64) {
	let radix = 10_u64.pow(num.ilog10().div_ceil(2));
	(num / radix, num % radix)
}

#[test]
fn split_digits_examples() {
	assert_eq!((1, 0), split_digits(10));
	assert_eq!((12, 34), split_digits(1234));
	assert_eq!((123, 456), split_digits(123_456));
}

impl From<Stone> for u64 {
	fn from(value: Stone) -> Self {
		value.value
	}
}

impl From<u64> for Stone {
	fn from(value: u64) -> Self {
		Stone { value }
	}
}

#[derive(Clone, Debug)]
struct RockLine {
	stones: BTreeMap<Stone, usize>,
}

impl RockLine {
	fn blink(&mut self, cache: &mut HashMap<Stone, Replacement>) {
		let mut delta: BTreeMap<Stone, isize> = BTreeMap::new();

		for (stone, count) in &self.stones {
			if *count == 0 {
				continue;
			}

			let replacement = stone.replacements(cache);

			// Every stone that is replaced is decremented by its count:
			let e = delta.entry(*stone).or_default();
			*e = e.checked_sub_unsigned(*count).unwrap();

			match replacement {
				Replacement::Single(new_stone) => {
					let e = delta.entry(new_stone).or_default();
					*e = e.checked_add_unsigned(*count).unwrap();
				}
				Replacement::Double(stone1, stone2) => {
					let e = delta.entry(stone1).or_default();
					*e = e.checked_add_unsigned(*count).unwrap();

					let e = delta.entry(stone2).or_default();
					*e = e.checked_add_unsigned(*count).unwrap();
				}
			}
		}

		for (to_change, delta) in delta {
			if delta == 0 {
				continue;
			}

			let entry = self.stones.entry(to_change).or_default();
			*entry = entry
				.checked_add_signed(delta)
				.expect("stone count overflow");
		}
	}

	fn count(&self) -> usize {
		self.stones.values().sum()
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, arrangement: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let stones = arrangement
			.split_ascii_whitespace()
			.map(|rock| rock.parse::<u64>().expect("failed to parse rock number"))
			.map(Stone::from)
			.fold(BTreeMap::new(), |mut acc, stone| {
				*acc.entry(stone).or_default() += 1;
				acc
			});

		Ok(Box::new(RockLine { stones }))
	}

	fn part_one(&self, rock_line: &dyn core::any::Any) -> Option<String> {
		let rock_line = rock_line.downcast_ref::<RockLine>()?;
		let mut rock_line: RockLine = (*rock_line).clone();

		let mut cache: HashMap<Stone, Replacement> = HashMap::new();

		for _ in 0..25 {
			rock_line.blink(&mut cache);
		}

		Some(rock_line.count().to_string())
	}

	fn part_two(&self, rock_line: &dyn core::any::Any) -> Option<String> {
		let rock_line = rock_line.downcast_ref::<RockLine>()?;
		let mut rock_line: RockLine = (*rock_line).clone();

		let mut cache: HashMap<Stone, Replacement> = HashMap::new();

		for _ in 0..75 {
			rock_line.blink(&mut cache);
		}

		Some(rock_line.count().to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day11.example.in.txt", part_one, literal 55312);
// part_test!(part_two, Solution, None, part_two, None);
