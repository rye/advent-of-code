use std::collections::BTreeMap;

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let lines = input.lines().map(|line| {
			let numbers = line
				.split_whitespace()
				.map(|num| num.parse::<i32>().unwrap())
				.collect::<Vec<i32>>();

			debug_assert!(numbers.len() == 2);

			(numbers[0], numbers[1])
		});

		Ok(Box::new(lines.collect::<Vec<_>>()))
	}

	fn part_one(&self, pairs: &Box<dyn core::any::Any>) -> Option<String> {
		let (left, right) = pairs
			.downcast_ref::<Vec<(i32, i32)>>()
			// Unpack the pairs into two separate left/right lists.
			.map(|pairs| {
				let mut left = Vec::with_capacity(pairs.len());
				let mut right = Vec::with_capacity(pairs.len());

				for (l, r) in pairs {
					left.push(*l);
					right.push(*r);
				}

				(left, right)
			})
			// Sort the lists.
			.map(|(mut left, mut right)| {
				left.sort_unstable();
				right.sort_unstable();

				(left, right)
			})
			.inspect(|(left, right)| {
				debug_assert!(left.len() == right.len());
			})?;

		// Compute the sum of differences.
		Some(
			left
				.iter()
				.zip(right.iter())
				.map(|(l, r)| (r - l).abs())
				.sum::<i32>()
				.to_string(),
		)
	}

	fn part_two(&self, pairs: &Box<dyn core::any::Any>) -> Option<String> {
		let (list, occurrences) = pairs.downcast_ref::<Vec<(i32, i32)>>().map(|pairs| {
			let mut left = Vec::with_capacity(pairs.len());
			let mut right: BTreeMap<i32, i32> = BTreeMap::new();

			// Left: Unstructure like Part One.
			// Right: Want BTreeMap<i32, i32> where i32 increments every time number appears in right list.
			for (l, r) in pairs {
				left.push(*l);
				*right.entry(*r).or_insert(0_i32) += 1;
			}

			(left, right)
		})?;

		Some(
			list
				.iter()
				.fold(0_i32, |score, &number| {
					score + occurrences.get(&number).unwrap_or(&0) * number
				})
				.to_string(),
		)
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day01.example.in.txt", part_one, file "day01.example.out-1.txt");
part_test!(part_two, Solution, file "day01.example.in.txt", part_two, file "day01.example.out-2.txt");
