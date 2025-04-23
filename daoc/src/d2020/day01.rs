use std::collections::BTreeSet;

/// Find a pair of elements in `list` that sum to `target`.
pub fn find_pair(list: &BTreeSet<i64>, target: &i64) -> Option<(i64, i64)> {
	for item in list {
		if list.contains(&(target - item)) {
			return Some((*item, target - item));
		}
	}
	None
}

/// Find a triple of elements in `list` that sum to `target`.
pub fn find_triple(list: &BTreeSet<i64>, target: &i64) -> Option<(i64, i64, i64)> {
	for item in list {
		if let Some(pair) = find_pair(list, &(target - item)) {
			return Some((*item, pair.0, pair.1));
		}
	}
	None
}

pub type Intermediate = BTreeSet<i64>;
pub type Output = i64;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		data
			.lines()
			.map(|line| line.parse::<i64>().expect("malformed input"))
			.collect(),
	)
}

pub fn part_one(list: &Intermediate) -> Option<Output> {
	find_pair(list, &2020_i64).map(|(a, b): (i64, i64)| a * b)
}

pub fn part_two(list: &Intermediate) -> Option<Output> {
	find_triple(list, &2020_i64).map(|(a, b, c): (i64, i64, i64)| a * b * c)
}

#[cfg(test)]
mod tests;

crate::generate_solver!(solve, =>, self);
