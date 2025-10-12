use std::collections::HashSet;

pub type Intermediate = Vec<u64>;
pub type Output = usize;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(jolt_differences(
		input
			.trim_end()
			.split('\n')
			.map(|line| line.parse::<u64>().unwrap())
			.collect(),
	))
}

pub fn part_one(differences: &Intermediate) -> Option<Output> {
	let one_offs_count = differences
		.iter()
		.filter(|difference| **difference == 1)
		.count();

	let three_offs_count = differences
		.iter()
		.filter(|difference| **difference == 3)
		.count();

	Some(one_offs_count * three_offs_count)
}

pub fn part_two(differences: &Intermediate) -> Option<Output> {
	let mut arrangement = vec![0];

	differences.iter().fold(0, |current, jolts| {
		let current = current + *jolts;
		arrangement.push(current);
		current
	});

	let total_count = split_non_overlapping(&arrangement)
		.iter()
		.fold(1, |acc, problem| acc * count_arrangements(problem));

	Some(total_count)
}

fn split_non_overlapping(arrangement: &[u64]) -> Vec<Vec<u64>> {
	let mut v = vec![];
	let mut prev_idx = 0;

	for idx in non_removable(arrangement) {
		if idx - prev_idx > 1 {
			let chunk = arrangement[prev_idx..=idx].to_vec();

			v.push(chunk);
		}

		prev_idx = idx;
	}

	v
}

fn non_removable(init: &[u64]) -> Vec<usize> {
	let mut v = vec![0, init.len() - 1];

	for (idx, num) in init.iter().enumerate() {
		if !v.contains(&idx) && (num - init[idx - 1] == 3 || init[idx + 1] - num == 3) {
			v.push(idx);
		}
	}

	v.sort_unstable();
	v
}

fn count_arrangements(problem: &Vec<u64>) -> usize {
	let mut counts = HashSet::new();
	count_arrangements_inner(&mut counts, problem);
	counts.len()
}

fn count_arrangements_inner(counts: &mut HashSet<Vec<u64>>, problem: &Vec<u64>) {
	if !counts.contains(problem) {
		counts.insert(problem.clone());

		for idx in 0..(problem.len() - 1) {
			if idx != 0 && idx != problem.len() - 1 && problem[idx + 1] - problem[idx - 1] <= 3 {
				let mut arrangement = problem.clone();
				arrangement.remove(idx);

				count_arrangements_inner(counts, &arrangement);
			}
		}
	}
}

fn jolt_differences(adapters: Vec<u64>) -> Vec<u64> {
	let mut adapters = adapters.into_iter().collect::<HashSet<_>>();

	let mut result = vec![];
	let mut current = 0;

	while !adapters.is_empty() {
		if adapters.contains(&(current + 1)) {
			result.push(1);
			current += 1;
		} else if adapters.contains(&(current + 2)) {
			result.push(2);
			current += 2;
		} else if adapters.contains(&(current + 3)) {
			result.push(3);
			current += 3;
		} else {
			panic!(
				"impossible joltage difference: current = {}, adapters = {:?}",
				&current, &adapters
			);
		}

		adapters.remove(&current);
	}

	result.push(3);
	result
}

#[cfg(test)]
mod tests {}

crate::generate_solver!(solve, =>, self);
