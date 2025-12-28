use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

fn find_max_joltage(bank: &[u8], n_batteries: usize) -> u64 {
	let len = bank.len();
	assert!(
		n_batteries <= len,
		"cannot select {n_batteries} batteries from a bank of length {len}"
	);

	if n_batteries == 0 {
		return 0;
	}

	// pow10[k] stores 10^k to conveniently position digits correctly.
	let mut pow10 = vec![1_u64; n_batteries + 1];
	for i in 1..=n_batteries {
		pow10[i] = pow10[i - 1] * 10;
	}

	// dp[pos][remaining] holds the best value using bank[pos..] with `remaining` picks.
	let mut dp = vec![vec![None; n_batteries + 1]; len + 1];
	dp[len][0] = Some(0);

	for pos in (0..len).rev() {
		for remaining in 0..=n_batteries {
			let skip = dp[pos + 1][remaining];

			let take = if remaining > 0 {
				dp[pos + 1][remaining - 1].map(|tail| {
					let digit = u64::from(bank[pos]);
					digit * pow10[remaining - 1] + tail
				})
			} else {
				None
			};

			dp[pos][remaining] = match (skip, take) {
				(Some(a), Some(b)) => Some(a.max(b)),
				(Some(a), None) => Some(a),
				(None, Some(b)) => Some(b),
				(None, None) => None,
			};
		}
	}

	dp[0][n_batteries].expect("failed to assemble a valid joltage")
}

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let banks: Vec<Vec<u8>> = input
			.lines()
			.map(|line| {
				line
					.chars()
					.filter_map(|c| {
						c.to_digit(10)
							.map(|joltage| u8::try_from(joltage).expect("joltage out of range"))
					})
					.collect::<Vec<u8>>()
			})
			.collect();

		debug_assert!(
			banks.iter().all(|bank| !bank.is_empty()),
			"All banks should have at least one entry"
		);

		debug_assert!(
			banks
				.iter()
				.all(|bank| bank.iter().all(|&joltage| (0..=9).contains(&joltage))),
			"All joltages should be between 0 and 9"
		);

		debug_assert!(
			!banks.is_empty(),
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

		let total_max_joltage: u32 = banks
			.iter()
			.map(|bank| u32::try_from(find_max_joltage(bank, 2)).expect("joltage out of range"))
			.sum();

		Some(total_max_joltage.to_string())
	}

	fn part_two(&self, banks: &dyn core::any::Any) -> Option<String> {
		let banks = banks
			.downcast_ref::<Vec<Vec<u8>>>()
			.expect("Failed to downcast banks");

		let total_max_joltage: u64 = banks.iter().map(|bank| find_max_joltage(bank, 12)).sum();

		Some(total_max_joltage.to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day03.example.in.txt", part_one, literal "357");
part_test!(part_two, Solution, file "day03.example.in.txt", part_two, literal "3121910778619");
