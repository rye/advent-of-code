use core::{cell::Cell, str::FromStr};
use std::{
	collections::{BTreeMap, BTreeSet, VecDeque},
	sync::OnceLock,
};

use regex::Regex;

use daocutil::SolverMode;

pub type Intermediate = BTreeMap<u8, Monkey>;
pub type Output = u64;

const PARSE_RE_STR: &str = r"(?s)^Monkey (?P<id>\d+):$\n^\s+Starting items: (?P<si>[\d, ]+)$\n^\s+Operation: new = (?P<oprd1>\b[\w\d]+\b) (?P<oprtr>[*+]) (?P<oprd2>\b[\w\d]+\b)$\n^\s+Test: divisible by (?P<divisor>\d+)$\n^\s+If true: throw to monkey (?P<true_dest>\d+).*?$\n^\s+If false: throw to monkey (?P<false_dest>\d+)$";

static PARSE_RE: OnceLock<Regex> = OnceLock::new();

#[derive(Clone, Copy, Debug)]
enum Operation {
	Add(u32),
	Mul(u32),
	Square,
}

#[derive(Clone, Debug)]
pub struct Monkey {
	id: u8,
	items: VecDeque<u32>,
	operation: Operation,
	divisor: u32,
	true_dest: u8,
	false_dest: u8,
	inspect_counter: u64,
}

impl FromStr for Monkey {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> anyhow::Result<Self> {
		let regex = PARSE_RE.get_or_init(|| {
			regex::RegexBuilder::new(PARSE_RE_STR)
				.multi_line(true)
				.dot_matches_new_line(true)
				.build()
				.expect("failed to compile regex")
		});

		let captures = regex.captures(s).expect("no captures for monkey spec");

		let id: u8 = captures
			.name("id")
			.map(|mtch| mtch.as_str())
			.map(str::parse)
			.expect("expected to get id")
			.expect("expected to be able to parse id as u8");

		let items: VecDeque<u32> = captures
			.name("si")
			.expect("no match for starting items")
			.as_str()
			.split(", ")
			.map(str::parse)
			.collect::<Result<Vec<_>, _>>()?
			.into();

		let operation: Operation = match (
			captures.name("oprd1").map(|m| m.as_str()),
			captures.name("oprtr").map(|m| m.as_str()),
			captures.name("oprd2").map(|m| m.as_str()),
		) {
			(Some("old"), Some("*"), Some("old")) => Operation::Square,
			(Some("old"), Some("*"), Some(k)) => {
				Operation::Mul(k.parse().expect("failed to parse second operator"))
			}
			(Some("old"), Some("+"), Some(k)) => {
				Operation::Add(k.parse().expect("failed to parse second operator"))
			}
			_ => unreachable!(),
		};
		let divisor: u32 = captures
			.name("divisor")
			.expect("did not get match for divisor")
			.as_str()
			.parse()?;
		let true_dest: u8 = captures
			.name("true_dest")
			.expect("did not get match for true dest")
			.as_str()
			.parse()?;
		let false_dest: u8 = captures
			.name("false_dest")
			.expect("did not get match for false dest")
			.as_str()
			.parse()?;

		let monkey = Monkey {
			id,
			items,
			operation,
			divisor,
			true_dest,
			false_dest,
			inspect_counter: 0_u64,
		};

		Ok(monkey)
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input
		.split("\n\n")
		.map(str::parse)
		.map(|maybe_monkey| maybe_monkey.map(|monkey: Monkey| (monkey.id, monkey)))
		.collect::<Result<BTreeMap<u8, Monkey>, _>>()
}

#[allow(clippy::needless_pass_by_value)]
fn turn(monkey_set: &mut BTreeMap<u8, Cell<Monkey>>, mode: SolverMode) {
	let lcm_divisor: u32 = monkey_set
		.values_mut()
		.map(|monkey| monkey.get_mut().divisor)
		.product();

	let mut in_flight_items: BTreeMap<u8, Vec<u32>> = BTreeMap::default();

	// Handle each monkey's turns:
	for (monkey_id, monkey) in &mut *monkey_set {
		// If there any items incoming to the monkey, go ahead and add them to the list.
		if let Some(incoming_items) = in_flight_items.get_mut(monkey_id) {
			let monkey = monkey.get_mut();
			for item in incoming_items.drain(..) {
				monkey.items.push_back(item);
			}
		}

		// Snag all the items so we don't have to mutate the monkey in-place.
		let items: Vec<u32> = {
			let monkey = monkey.get_mut();
			monkey.items.drain(..).collect()
		};

		let monkey: &mut Monkey = monkey.get_mut();

		for item in items {
			let mut new_value = item;

			monkey.inspect_counter += 1;

			match monkey.operation {
				Operation::Add(amt) => new_value += amt,
				Operation::Mul(amt) => new_value *= amt,
				Operation::Square => {
					new_value = u32::try_from(u64::from(new_value).pow(2) % u64::from(lcm_divisor))
						.expect("divisor lcm should not be greater than u32::MAX");
				}
			}

			match mode {
				SolverMode::PartOne => new_value /= 3,
				SolverMode::PartTwo => new_value %= lcm_divisor,
			}

			if new_value % monkey.divisor == 0 {
				in_flight_items
					.entry(monkey.true_dest)
					.or_default()
					.push(new_value);
			} else {
				in_flight_items
					.entry(monkey.false_dest)
					.or_default()
					.push(new_value);
			}
		}
	}

	for (monkey_id, mut incoming) in in_flight_items {
		let monkey = monkey_set
			.get_mut(&monkey_id)
			.expect("monke gone")
			.get_mut();
		for item in incoming.drain(..) {
			monkey.items.push_back(item);
		}
	}
}

#[must_use]
pub fn part_one(monkeys: &Intermediate) -> Option<Output> {
	let mut monkey_set: BTreeMap<u8, Cell<Monkey>> = monkeys
		.iter()
		.map(|(id, monkey)| (id.to_owned(), Cell::new((*monkey).clone())))
		.collect();

	for _n in 0..20 {
		turn(&mut monkey_set, SolverMode::PartOne);
	}

	let monkey_business: u64 = monkey_set
		.into_values()
		.map(Cell::into_inner)
		.map(|monkey| monkey.inspect_counter)
		.collect::<BTreeSet<u64>>()
		.iter()
		.rev()
		.take(2)
		.product();

	Some(monkey_business)
}

#[must_use]
pub fn part_two(monkeys: &Intermediate) -> Option<Output> {
	let mut monkey_set: BTreeMap<u8, Cell<Monkey>> = monkeys
		.iter()
		.map(|(id, monkey)| (id.to_owned(), Cell::new((*monkey).clone())))
		.collect();

	for _n in 0..10000 {
		turn(&mut monkey_set, SolverMode::PartTwo);
	}

	let monkey_business: u64 = monkey_set
		.into_values()
		.map(Cell::into_inner)
		.map(|monkey| monkey.inspect_counter)
		.collect::<BTreeSet<u64>>()
		.iter()
		.rev()
		.take(2)
		.product();

	Some(monkey_business)
}

daocutil::generate_solver!(solve, =>, self);
