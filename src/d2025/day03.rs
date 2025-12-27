use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

fn find_max_joltage_slow(bank: &[u8]) -> u16 {
	// The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)
	//
	// Iterative process. We look for a joltage of 9, then 8, etc. Once we find it, we look for the highest joltage to the right of it.
	// This process might require backtracking , e.g. if the bank is 729, the max joltage would be 79.
	// However, if the bank is 798, the max joltage would be 98.
	// This means that we need to keep track of the last found joltage and its positions

	// Brute force approach:
	let mut max_joltage = 0u16;
	let len = bank.len();

	for i in 0..len {
		for j in (i + 1)..len {
			let joltage = (bank[i] as u16) * 10 + (bank[j] as u16);
			if joltage > max_joltage {
				max_joltage = joltage;
			}
		}
	}

	max_joltage
}

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let banks: Vec<Vec<u8>> = input
			.lines()
			.map(|line| {
				line
					.chars()
					.filter_map(|c| c.to_digit(10).map(|joltage| joltage as u8))
					.collect::<Vec<u8>>()
			})
			.collect();

		debug_assert!(
			banks.iter().all(|bank| bank.len() > 0),
			"All banks should have at least one entry"
		);

		debug_assert!(
			banks
				.iter()
				.all(|bank| bank.iter().all(|&joltage| (0..=9).contains(&joltage))),
			"All joltages should be between 0 and 9"
		);

		debug_assert!(
			banks.len() > 0,
			"There should be at least one bank of joltages"
		);

		// Check all lines are of the same length
		let first_len = banks[0].len();
		debug_assert!(
			banks.iter().all(|bank| bank.len() == first_len),
			"All banks should have the same number of entries"
		);

		Ok(Box::new(banks))
	}

	fn part_one(&self, banks: &dyn core::any::Any) -> Option<String> {
		let banks = banks
			.downcast_ref::<Vec<Vec<u8>>>()
			.expect("Failed to downcast banks");

		let total_max_joltage: u32 = banks.iter().map(|bank| find_max_joltage(bank) as u32).sum();

		Some(total_max_joltage.to_string())
	}

	fn part_two(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
		None
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day03.example.in.txt", part_one, literal "357");
part_test!(part_two, Solution, file "day03.example.in.txt", part_two, None);
