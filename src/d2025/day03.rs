use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

fn find_max_joltage(bank: &[u8], n_batteries: usize) -> u64 {
	// In Part One, bank is 100 digits long and there are only 2 batteries to select, leading to a total of
	// 100 choose 2 = 4_950 combinations.
	//
	// In Part Two, though, there are 12 batteries to select, a total of 100 choose 12 = 1_050_421_051_106_700
	// combinations, which is obviously way more than we want to iterate over by brute force. Instead, we can use a
	// recursive DFS approach with memoization to find the maximum joltage efficiently.

	let len = bank.len();
	assert!(
		n_batteries <= len,
		"cannot select {n_batteries} batteries from a bank of length {len}"
	);

	if n_batteries == 0 {
		return 0;
	}

	// pow10[k] stores 10^k, letting us assign digits to their positional value when assembling the joltage.
	let mut pow10 = vec![1_u64; n_batteries + 1];
	for i in 1..=n_batteries {
		pow10[i] = pow10[i - 1] * 10;
	}

	// memo[pos][remaining] caches the best value obtainable from bank[pos..] with `remaining` picks.
	let mut memo = vec![vec![None; n_batteries + 1]; len + 1];
	let mut visited = vec![vec![false; n_batteries + 1]; len + 1];

	// Depth-first search with memoization: at each position we may skip or take the current digit.
	fn dfs(
		pos: usize,
		remaining: usize,
		bank: &[u8],
		pow10: &[u64],
		memo: &mut Vec<Vec<Option<u64>>>,
		visited: &mut Vec<Vec<bool>>,
	) -> Option<u64> {
		// All required digits chosen; nothing left to add to the result.
		if remaining == 0 {
			return Some(0);
		}

		// Not enough digits left to fulfill requirements.
		if bank.len() - pos < remaining {
			return None;
		}

		// Return cached result if available.
		if visited[pos][remaining] {
			return memo[pos][remaining];
		}

		visited[pos][remaining] = true;

		// Explore the option of skipping the current digit.
		let mut best = dfs(pos + 1, remaining, bank, pow10, memo, visited);

		// Explore the option of taking the current digit.
		if let Some(tail) = dfs(pos + 1, remaining - 1, bank, pow10, memo, visited) {
			let digit = bank[pos] as u64;
			// Place the chosen digit at the correct power of ten ahead of the already-built tail.
			let value = digit * pow10[remaining - 1] + tail;

			best = Some(match best {
				Some(current) => current.max(value),
				None => value,
			});
		}

		memo[pos][remaining] = best;

		best
	}

	dfs(0, n_batteries, bank, &pow10, &mut memo, &mut visited)
		.expect("failed to assemble a valid joltage")
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

		let total_max_joltage: u32 = banks
			.iter()
			.map(|bank| find_max_joltage(bank, 2) as u32)
			.sum();

		Some(total_max_joltage.to_string())
	}

	fn part_two(&self, banks: &dyn core::any::Any) -> Option<String> {
		let banks = banks
			.downcast_ref::<Vec<Vec<u8>>>()
			.expect("Failed to downcast banks");

		let total_max_joltage: u64 = banks
			.iter()
			.map(|bank| find_max_joltage(bank, 12) as u64)
			.sum();

		Some(total_max_joltage.to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day03.example.in.txt", part_one, literal "357");
part_test!(part_two, Solution, file "day03.example.in.txt", part_two, literal "3121910778619");
