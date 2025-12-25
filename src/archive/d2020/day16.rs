use regex::Regex;

use core::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
	pub field: String,
	pub ranges: [RangeInclusive<u64>; 2],
}

impl Rule {
	pub fn matches(&self, x: u64) -> bool {
		self.ranges.iter().any(|range| range.contains(&x))
	}
}

impl From<&str> for Rule {
	fn from(s: &str) -> Self {
		let rule_regex: Regex = Regex::new(r"^(.*+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

		let caps = rule_regex.captures(s).unwrap();

		Rule {
			field: caps.get(1).unwrap().as_str().into(),
			ranges: [
				caps.get(2).unwrap().as_str().parse().unwrap()
					..=caps.get(3).unwrap().as_str().parse().unwrap(),
				caps.get(4).unwrap().as_str().parse().unwrap()
					..=caps.get(5).unwrap().as_str().parse().unwrap(),
			],
		}
	}
}

#[derive(Debug, Clone)]
pub struct Input {
	rules: Vec<Rule>,
	mine: Vec<u64>,
	others: Vec<Vec<u64>>,
}

pub type Intermediate = Input;
type Output = u64;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let mut sections = input.split("\n\n");

	let rules: Vec<Rule> = sections.next().unwrap().lines().map(Rule::from).collect();

	let mine = sections
		.next()
		.unwrap()
		.lines()
		.nth(1)
		.unwrap()
		.split(',')
		.map(|n| n.parse().unwrap())
		.collect();

	let others = sections
		.next()
		.unwrap()
		.lines()
		.skip(1)
		.map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
		.collect();

	Ok(Input {
		rules,
		mine,
		others,
	})
}

pub fn part_one(input: &Intermediate) -> Option<Output> {
	let (rules, _mine, others): (&Vec<Rule>, &Vec<u64>, &Vec<Vec<u64>>) =
		(&input.rules, &input.mine, &input.others);

	Some(
		others
			.iter()
			.flatten()
			.filter(|field| !rules.iter().any(|rule| rule.matches(**field)))
			.sum(),
	)
}

#[cfg(test)]
mod part_one {
	use super::{parse, part_one};

	#[test]
	fn example() {
		let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

		let intermediate = parse(input).unwrap();

		assert_eq!(
			intermediate.rules,
			vec![
				super::Rule {
					field: "class".to_string(),
					ranges: [1..=3, 5..=7]
				},
				super::Rule {
					field: "row".to_string(),
					ranges: [6..=11, 33..=44]
				},
				super::Rule {
					field: "seat".to_string(),
					ranges: [13..=40, 45..=50]
				}
			]
		);

		assert_eq!(intermediate.mine, vec![7, 1, 14]);

		assert_eq!(
			intermediate.others,
			vec![
				vec![7, 3, 47],
				vec![40, 4, 50],
				vec![55, 2, 20],
				vec![38, 6, 12],
			]
		);

		let part_one = part_one(&intermediate);

		assert_eq!(part_one, Some(71));
	}
}

pub fn part_two(input: &Intermediate) -> Option<Output> {
	let (rules, mine, others): (&Vec<Rule>, &Vec<u64>, &Vec<Vec<u64>>) =
		(&input.rules, &input.mine, &input.others);

	// Discard invalid tickets entirely as we now only care about tickets that
	// contain valid values.
	let valid_others: Vec<_> = others
		.iter()
		.filter(|ticket| {
			ticket.iter().all(|field| {
				rules
					.iter()
					.any(|rule| rule.ranges.iter().any(|range| range.contains(field)))
			})
		})
		.collect();

	// Build a map of field names to possible indices.
	//
	// For each rule, loop over all ticket fields (idx), and for each field index,
	// check all the valid tickets (other) to verify that the rule matches the
	// value contained within.  Using iter.all, if any of the tickets contains
	// an invalid value at the index, then that index cannot match the rules.
	let mut unknown: HashMap<String, HashSet<usize>> = rules
		.iter()
		.map(|rule| {
			(
				rule.field.clone(),
				(0..mine.len())
					.filter(|idx| valid_others.iter().all(|other| rule.matches(other[*idx])))
					.collect(),
			)
		})
		.collect();

	// Now, solve the constraints.
	let mut known = HashMap::<String, usize>::new();

	// As long as there is one field with only one possible index, and as long
	// as we haven't solved all the fields, pick a field with only one possible
	// index, remove its index from all other fields' lists of possible indices,
	// and repeat until all fields are solved.
	while !unknown.is_empty() {
		// Find a field with only one possible index.
		let (field, idx): (String, usize) = unknown
			.iter()
			.find(|(_, idxes)| idxes.len() == 1)
			.map(|(f, i)| (f.clone(), *i.iter().next().unwrap()))
			.unwrap();

		// We now know the index for that field.  Remove it from the
		// list of unknown fields, and insert it (and the known index)
		// into the known set.
		unknown.remove(&field);
		known.insert(field, idx);

		// Now, no other field can have that index, so remove it from
		// all other fields.
		for idxes in unknown.values_mut() {
			idxes.remove(&idx);
		}
	}

	// Now that we know the locations for all the fields, the solution is the
	// product of the values in all of "our" "departure" fields.
	Some(
		known
			.iter()
			.filter(|(field, _idx)| field.starts_with("departure "))
			.fold(1_u64, |product, (_field, idx)| product * mine[*idx]),
	)
}

#[cfg(test)]
mod part_two {
	use super::{parse, part_two};

	#[test]
	fn example() {
		let input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

		let intermediate = parse(input).unwrap();

		assert_eq!(
			intermediate.rules,
			vec![
				super::Rule {
					field: "class".to_string(),
					ranges: [0..=1, 4..=19]
				},
				super::Rule {
					field: "row".to_string(),
					ranges: [0..=5, 8..=19]
				},
				super::Rule {
					field: "seat".to_string(),
					ranges: [0..=13, 16..=19]
				}
			]
		);

		assert_eq!(intermediate.mine, vec![11, 12, 13]);

		assert_eq!(
			intermediate.others,
			vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9],]
		);

		let part_two = part_two(&intermediate);

		assert_eq!(part_two, Some(1));
	}
}

crate::generate_solver!(solve, =>, self);
