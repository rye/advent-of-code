use std::collections::BTreeSet;

fn sample() -> BTreeSet<i64> {
	vec![1721, 979, 366, 299, 675, 1456].drain(..).collect()
}

#[cfg(test)]
mod find_pair {
	use super::super::find_pair;
	use super::*;

	#[test]
	fn sample_correct() {
		let list = sample();
		let target = 2020_i64;
		assert_eq!(find_pair(&list, &target), Some((299, 1721)));
	}

	#[test]
	fn empty_list_correct() {
		let list = BTreeSet::new();
		let target = 2020_i64;
		assert_eq!(find_pair(&list, &target), None);
	}
}

#[cfg(test)]
mod find_triple {
	use super::super::find_triple;
	use super::*;

	#[test]
	fn sample_correct() {
		let list = sample();
		let target = 2020_i64;
		assert_eq!(find_triple(&list, &target), Some((366, 675, 979)));
	}

	#[test]
	fn empty_list_correct() {
		let list = BTreeSet::new();
		let target = 2020_i64;
		assert_eq!(find_triple(&list, &target), None);
	}
}
