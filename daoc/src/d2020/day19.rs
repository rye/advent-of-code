#![allow(unused_imports)]

use std::io::{Read, stdin};
use std::{collections::*, str::FromStr};

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
	SingleChar(char),
	Compound(Vec<Vec<usize>>),
}

impl FromStr for Rule {
	type Err = ();
	fn from_str(str: &str) -> Result<Self, ()> {
		let chars: Vec<char> = str.chars().collect();

		if chars[0] == '"' {
			Ok(Self::SingleChar(chars[1]))
		} else {
			let groups: Vec<Vec<usize>> = str
				.split(" | ")
				.map(|group| {
					group
						.split(' ')
						.map(|n| n.parse::<usize>().unwrap())
						.collect()
				})
				.collect();

			Ok(Self::Compound(groups))
		}
	}
}

#[derive(Debug, Clone)]
pub struct RuleSet {
	rules: Vec<Rule>,
}

impl FromStr for RuleSet {
	type Err = ();

	fn from_str(str: &str) -> Result<Self, ()> {
		let split: Vec<(usize, &str)> = str
			.lines()
			.map(|line| {
				let mut split = line.split(": ");

				(
					split.next().unwrap().parse::<usize>().unwrap(),
					split.next().unwrap(),
				)
			})
			.collect();

		let size: usize = *split.iter().map(|(idx, _)| idx).max().unwrap();

		let mut rules: Vec<Option<Rule>> = Vec::new();
		rules.resize(size + 1_usize, None);

		for (idx, str) in split {
			let rule: Rule = str.parse().unwrap();
			let idx: usize = idx;

			debug_assert!(idx < rules.len());
			assert_eq!(rules[idx], None);

			rules[idx] = Some(rule);
		}

		let rules: Vec<Option<Rule>> = rules;

		let rules: Vec<Rule> = rules.into_iter().flatten().collect();

		assert_eq!(rules.len(), size + 1);

		Ok(RuleSet { rules })
	}
}

fn convert_rule(ruleset: &RuleSet, rule_idx: usize) -> String {
	match &ruleset.rules[rule_idx] {
		Rule::SingleChar(c) => c.to_string(),
		Rule::Compound(groups) => {
			let group_string = groups
				.iter()
				.map(|inner_rules| {
					inner_rules
						.iter()
						.map(|inner_rule_idx| convert_rule(ruleset, *inner_rule_idx))
						.collect::<String>()
				})
				.collect::<Vec<_>>()
				.join("|");

			if groups.len() == 1 {
				group_string
			} else {
				format!("(?:{group_string})")
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn simple() {
		let rules = "0: 1 1\n1: \"a\"\n";
		let rule_set: RuleSet = rules.parse().unwrap();

		let regex = Regex::from(&rule_set);

		assert_eq!(regex.as_str(), "^aa$");
		assert!(regex.is_match("aa"));
		assert!(!regex.is_match("ab"));
	}
	#[test]
	fn slightly_longer() {
		let rules = "0: 1 2\n1: \"a\"\n2: \"b\"";
		let rule_set: RuleSet = rules.parse().unwrap();

		let regex = Regex::from(&rule_set);

		assert_eq!(regex.as_str(), "^ab$");
		assert!(!regex.is_match("aa"));
		assert!(regex.is_match("ab"));
	}
	#[test]
	fn alternation() {
		let rules = "0: 1 2 | 2 1\n1: \"a\"\n2: \"b\"";
		let rule_set: RuleSet = rules.parse().unwrap();

		let regex = Regex::from(&rule_set);

		assert_eq!(regex.as_str(), "^(?:ab|ba)$");
		assert!(regex.is_match("ab"));
		assert!(regex.is_match("ba"));
	}

	#[test]
	fn given_more_complex() {
		let rules = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"";
		let rule_set: RuleSet = rules.parse().unwrap();

		let regex = Regex::from(&rule_set);

		println!("{}", regex.as_str());

		assert!(regex.is_match("ababbb"));
		assert!(regex.is_match("abbbab"));
		assert!(!regex.is_match("bababa"));
		assert!(!regex.is_match("aaabbb"));
		assert!(!regex.is_match("aaaaabbb"));
	}
}

impl From<&RuleSet> for Regex {
	fn from(rule_set: &RuleSet) -> Regex {
		let rule_string = convert_rule(rule_set, 0_usize);
		Regex::new(&format!("^{rule_string}$")).unwrap()
	}
}

#[derive(Debug)]
pub struct Message<'x>(&'x str);

pub type Intermediate<'input> = (RuleSet, Vec<Message<'input>>);
type Output = usize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	let split: Vec<&str> = data.split("\n\n").collect();

	let messages: Vec<Message> = split[1].lines().map(Message).collect();

	Ok((split[0].parse().unwrap(), messages))
}

pub fn part_one((rules, messages): &Intermediate) -> Option<Output> {
	let ruleset_regex = regex::Regex::from(rules);

	let count = messages
		.iter()
		.filter(|message| ruleset_regex.is_match(message.0))
		.count();

	Some(count)
}

pub fn part_two(_: &Intermediate) -> Option<Output> {
	None
}

daocutil::generate_solver!(solve, =>, self);
