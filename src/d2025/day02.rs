use core::ops::RangeInclusive;

use crate::{PartSolve, Solver, export_solver, part_test};

trait ValidationExt {
	fn is_valid_id(&self) -> bool;
	fn is_valid_id_general(&self) -> bool;
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

	fn is_valid_id_general(&self) -> bool {
		// Start by getting the digits.
		let digits = self
			.to_string()
			.chars()
			.map(|c| c as u8 - b'0')
			.collect::<Vec<u8>>();

		let len = digits.len();

		// Check chunk sizes from len/2 down to 1.
		for chunk_size in (1..=len / 2).rev() {
			// If len is not evenly divisible by chunk_size, skip it.
			if len % chunk_size != 0 {
				continue;
			}

			let num_chunks = len / chunk_size;

			let chunks = (0..num_chunks)
				.map(|i| digits[i * chunk_size..(i + 1) * chunk_size].to_vec())
				.collect::<Vec<Vec<u8>>>();

			// Check if all chunks are the same as the first chunk.
			let first_chunk = &chunks[0];
			let mut all_same = true;
			for chunk in &chunks[1..] {
				if chunk != first_chunk {
					all_same = false;
					break;
				}
			}

			if all_same {
				return false;
			}
		}

		true
	}
}

#[test]
fn range_1122_valid() {
	let range = 11..=22_u64;
	#[allow(clippy::redundant_closure_for_method_calls)]
	let valid_ids = range
		.clone()
		.filter(|id| id.is_valid_id())
		.collect::<Vec<u64>>();
	let invalid_ids = range
		.clone()
		.filter(|id| !id.is_valid_id())
		.collect::<Vec<u64>>();
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
				.map(|range| range.clone().filter(|id| !id.is_valid_id()).sum::<u64>())
				.sum::<u64>()
				.to_string(),
		)
	}

	fn part_two(&self, ranges: &dyn core::any::Any) -> Option<String> {
		let ranges = ranges.downcast_ref::<Vec<RangeInclusive<u64>>>()?;

		Some(
			ranges
				.into_iter()
				.map(|range| {
					range
						.clone()
						.filter(|id| !id.is_valid_id_general())
						.sum::<u64>()
				})
				.sum::<u64>()
				.to_string(),
		)
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day02.example.in.txt", part_one, file "day02.example.out-1.txt");
part_test!(part_two, Solution, file "day02.example.in.txt", part_two, file "day02.example.out-2.txt");
