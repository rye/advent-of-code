use core::ops::RangeInclusive;

use crate::{PartSolve, Solver, export_solver, part_test};

trait ValidationExt {
	fn is_valid_id(&self) -> bool;
}

impl ValidationExt for u64 {
	fn is_valid_id(&self) -> bool {
		let digits = self
			.to_string()
			.chars()
			.map(|c| c as u8 - b'0')
			.collect::<Vec<u8>>();

		// If an odd number of digits, it can't be a pattern repeating, so it's valid.
		if digits.len() % 2 != 0 {
			return true;
		}

		let half_len = digits.len() / 2;
		let mut invalid = true;

		for i in 0..half_len {
			if digits[i] != digits[i + half_len] {
				invalid = false;
				break;
			}
		}

		!invalid
	}
}

enum ValidityFilter<I>
where
	I: Iterator,
{
	None(I),
	ValidOnly(I),
	InvalidOnly(I),
	All(I),
}

impl<I> Iterator for ValidityFilter<I>
where
	I: Iterator<Item = u64>,
{
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		match self {
			ValidityFilter::None(iter) => iter.next(),
			ValidityFilter::ValidOnly(iter) => iter.next(),
			ValidityFilter::InvalidOnly(iter) => iter.next(),
			ValidityFilter::All(iter) => iter.next(),
		}
	}
}

trait ValidityFilterExt: Iterator {
	fn only_valid(self) -> ValidityFilter<impl Iterator<Item = u64>>;
	fn only_invalid(self) -> ValidityFilter<impl Iterator<Item = u64>>;
}

impl<I> ValidityFilterExt for I
where
	I: Iterator<Item = u64>,
{
	fn only_valid(self) -> ValidityFilter<impl Iterator<Item = u64>>
	where
		Self: Sized,
	{
		ValidityFilter::ValidOnly(self.filter(|id| id.is_valid_id()))
	}

	fn only_invalid(self) -> ValidityFilter<impl Iterator<Item = u64>>
	where
		Self: Sized,
	{
		ValidityFilter::ValidOnly(self.filter(|id| !id.is_valid_id()))
	}
}

#[test]
fn range_1122_valid() {
	let range = 11..=22_u64;
	let valid_ids = range.clone().only_valid().collect::<Vec<u64>>();
	let invalid_ids = range.clone().only_invalid().collect::<Vec<u64>>();
	assert_eq!(2, invalid_ids.len());
	assert_eq!(10, valid_ids.len());
	assert_eq!(vec![11, 22], invalid_ids);
	assert_eq!(vec![12, 13, 14, 15, 16, 17, 18, 19, 20, 21], valid_ids);
}

#[derive(Default)]
struct Solution;

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let ranges = input
			.split(',')
			.map(str::trim)
			.map(|s| {
				let (start, end) = s.split_once('-').expect("Range parse err");
				let start: u64 = start.parse().expect("u64 parse err");
				let end: u64 = end.parse().expect("u64 parse err");
				start..=end
			})
			.collect::<Vec<RangeInclusive<u64>>>();

		Ok(Box::new(ranges))
	}

	fn part_one(&self, ranges: &dyn core::any::Any) -> Option<String> {
		let ranges = ranges.downcast_ref::<Vec<RangeInclusive<u64>>>()?;

		Some(
			ranges
				.into_iter()
				.map(|range| range.clone().only_invalid().sum::<u64>())
				.sum::<u64>()
				.to_string(),
		)
	}

	fn part_two(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
		None
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day02.example.in.txt", part_one, file "day02.example.out-1.txt");
part_test!(part_two, Solution, file "day02.example.in.txt", part_two, None);
