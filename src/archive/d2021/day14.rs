use std::collections::{BTreeMap, btree_map::Entry};

use itertools::Itertools;

pub type Intermediate<'input> = (&'input str, BTreeMap<&'input str, char>);

pub fn parse<'input>(
	input: &'input str,
) -> Result<Intermediate<'input>, core::convert::Infallible> {
	let mut split = input.split("\n\n");
	let first_line: &'input str = split.next().expect("missing template");
	let insertion_rules: &'input str = split.next().expect("missing rules");

	let insertion_rules: BTreeMap<&'input str, char> = insertion_rules
		.lines()
		.map(|line| {
			let mut parts = line.split(" -> ");

			let pair = parts.next().expect("failed to get pair for matching");
			let element: char = parts
				.next()
				.expect("missing element")
				.chars()
				.next()
				.expect("missing insertion character");

			(pair, element)
		})
		.collect();

	Ok((first_line, insertion_rules))
}

type Output = usize;

fn one_step_inner(
	(_a_idx, a_c): (usize, char),
	(b_idx, b_c): (usize, char),
	rules: &BTreeMap<&str, char>,
	len: usize,
) -> String {
	let last = b_idx == len - 1;

	let lookup_pair: String = [a_c, b_c].iter().collect();

	let mut pair: Vec<char> = vec![a_c];

	let lookup_pair: &str = &lookup_pair;

	if let Some(&element) = rules.get(lookup_pair) {
		pair.push(element);
	}

	if last {
		pair.push(b_c);
	}

	pair.iter().collect()
}

fn one_step(existing_polymer: &str, rules: &BTreeMap<&str, char>) -> String {
	let len = existing_polymer.len();
	existing_polymer
		.chars()
		.enumerate()
		.tuple_windows()
		.map(|(a, b)| one_step_inner(a, b, rules, len))
		.collect()
}

#[test]
fn one_step_example_nncb_nn_c() {
	let rules = vec![("NN", 'C')].into_iter().collect();

	assert_eq!(one_step("NNCB", &rules), "NCNCB");
}

#[test]
fn one_step_example_nncb_nn_c_nc_b() {
	let rules = vec![("NN", 'C'), ("NC", 'B')].into_iter().collect();
	assert_eq!(one_step("NNCB", &rules), "NCNBCB");
}

#[test]
fn one_step_example_nncb_nn_c_nc_b_cb_h() {
	let rules = vec![("NN", 'C'), ("NC", 'B'), ("CB", 'H')]
		.into_iter()
		.collect();

	assert_eq!(one_step("NNCB", &rules), "NCNBCHB");
}

fn statistics(string: &str) -> BTreeMap<char, usize> {
	let mut statistics: BTreeMap<char, usize> = BTreeMap::default();

	for c in string.chars() {
		match statistics.entry(c) {
			Entry::Occupied(mut e) => e.insert(e.get() + 1),
			Entry::Vacant(e) => *e.insert(1),
		};
	}

	statistics
}

#[must_use]
pub fn part_one((template, rules): &Intermediate) -> Option<Output> {
	let mut template = (*template).to_string();

	for step in 0..10 {
		template = one_step(&template, rules);
		println!("Step {}: length = {}", step + 1, template.len());
	}

	let statistics = statistics(&template);

	let qty_max = statistics.values().max().expect("expected a maximum");
	let qty_min = statistics.values().min().expect("expected a minimum");

	Some(qty_max - qty_min)
}

#[must_use]
pub fn part_two((template, rules): &Intermediate) -> Option<Output> {
	let mut template = (*template).to_string();

	for step in 0..40 {
		template = one_step(&template, rules);
		println!("Step {}: length = {}", step + 1, template.len());
	}

	let statistics = statistics(&template);

	let qty_max = statistics.values().max().expect("expected a maximum");
	let qty_min = statistics.values().min().expect("expected a minimum");

	Some(qty_max - qty_min)
}

crate::generate_solver!(solve, =>, self);
