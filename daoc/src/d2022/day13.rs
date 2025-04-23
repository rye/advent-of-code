use core::str::FromStr;
use std::{cmp::Ordering, collections::VecDeque};

pub type Intermediate = Vec<Pair>;
pub type Output = usize;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
#[cfg_attr(test, derive(Debug))]
pub enum PacketData {
	Integer(u32),
	List(Vec<PacketData>),
}

impl core::fmt::Display for PacketData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PacketData::Integer(u32) => write!(f, "{u32}"),
			PacketData::List(children) => write!(
				f,
				"[{}]",
				children
					.iter()
					.map(ToString::to_string)
					.collect::<Vec<_>>()
					.join(",")
			),
		}
	}
}

fn find_split_points(str: &str) -> Option<Vec<usize>> {
	let split_points: Vec<usize> = str
		.chars()
		.scan(1, |stack_level: &mut usize, item: char| -> Option<usize> {
			match (item, *stack_level) {
				// Open brackets always increase the stack level.
				('[', cur_level) if cur_level >= 1 => *stack_level += 1,
				// Special case for [ immediately following a ,.
				('[', 0) => *stack_level = 2,

				// Close brackets decrease the stack level only if the stack level was elevated.
				// This prevents bugs with inputs like "]]]]]", where we never had valid opens.
				(']', cur_level) if cur_level > 1 => *stack_level -= 1,

				// Commas only decrease the stack level to 0 if we're at 1.
				// Sequential commas are technically a valid
				(',', cur_level) if cur_level <= 1 => *stack_level = 0,

				// If the current level is 0 but the character is not a ,, then it is safe to reset
				// the stack level back to 1 for this character.
				(_, 0) => *stack_level = 1,

				(_, _) => {}
			}

			Some(*stack_level)
		})
		.enumerate()
		.filter_map(|(index, stack_level)| match stack_level {
			0_usize => Some(index),
			_ => None,
		})
		.collect();

	if split_points.is_empty() {
		None
	} else {
		Some(split_points)
	}
}

#[cfg(test)]
mod split_point {
	use super::find_split_points;

	#[test]
	fn singletons() {
		assert_eq!(None, find_split_points("a"));
		assert_eq!(None, find_split_points("1"));
		assert_eq!(None, find_split_points("123456"));
		assert_eq!(None, find_split_points("[]"));
		assert_eq!(None, find_split_points("[[[1,2]]]"));
	}

	#[test]
	fn doubles() {
		assert_eq!(Some(vec![1]), find_split_points("1,2"));
		assert_eq!(Some(vec![9]), find_split_points("[[1,2],3],2"));
	}

	#[test]
	fn weird() {
		assert_eq!(
			Some(vec![0, 2, 4, 6, 7, 8]),
			find_split_points(",],],],,,]")
		);
		assert_eq!(Some(vec![0, 1, 2]), find_split_points(",,,"));
		assert_eq!(Some(vec![0, 1, 2, 4]), find_split_points(",,,],"));
		assert_eq!(
			Some(vec![0, 4, 16, 20, 23]),
			find_split_points(",[,],[,[,[,],],],[,],[],")
		);
	}
}

fn split_at_split_points(str: &str) -> Vec<&str> {
	let split_points = find_split_points(str);

	if let Some(split_points) = split_points {
		let mut remainder = str;
		let mut pos_abs = 0_usize;

		let mut chunks = Vec::new();

		for split_point_abs in split_points {
			let (chunk, new_remainder) = remainder.split_at(split_point_abs - pos_abs);
			remainder = &new_remainder[1..];
			pos_abs += chunk.chars().count() + 1;
			chunks.push(chunk);
		}

		chunks.push(remainder);

		chunks
	} else {
		vec![str]
	}
}

#[test]
fn split() {
	assert_eq!(vec!["a"], split_at_split_points("a"));
	assert_eq!(vec!["a", "b", ""], split_at_split_points("a,b,"));
	assert_eq!(
		vec!["a", "b", "[,]", "[[[[,]]]]"],
		split_at_split_points("a,b,[,],[[[[,]]]]")
	);
}

fn list_from(s: &str) -> Result<PacketData, ParseError> {
	debug_assert_eq!(Some('['), s.chars().next());
	debug_assert_eq!(Some(']'), s.chars().last());

	let range = 1..(s.chars().count() - 1);

	let mut queue: VecDeque<&str> = split_at_split_points(&s[range]).into();

	let mut list: Vec<PacketData> = vec![];

	while let Some(part) = queue.pop_front() {
		let part = part.trim();
		match part.bytes().next() {
			Some(b'[') => list.push(list_from(part)?),
			Some(c) if c.is_ascii_digit() => list.push(number_from(part)?),
			Some(_) => todo!("error handling A"),
			None => {}
		}
	}

	Ok(PacketData::List(list))
}

fn number_from(s: &str) -> Result<PacketData, ParseError> {
	debug_assert!(s.bytes().all(|byte| byte.is_ascii_digit()));

	let number = s.parse();

	if let Ok(u32) = number {
		Ok(PacketData::Integer(u32))
	} else {
		Err(ParseError::MissingCharacter)
	}
}

impl FromStr for PacketData {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().peekable().peek() {
			Some('[') => list_from(s),
			Some(c) if c.is_ascii_digit() => number_from(s),
			Some(c) => Err(ParseError::UnexpectedCharacter(*c)),
			None => Err(ParseError::MissingCharacter),
		}
	}
}

pub struct Pair(PacketData, PacketData);

impl FromStr for Pair {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();
		let first_line = lines.next().ok_or(ParseError::PairMissingLine)?.parse()?;
		let second_line = lines.next().ok_or(ParseError::PairMissingLine)?.parse()?;
		Ok(Self(first_line, second_line))
	}
}

#[derive(thiserror::Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ParseError {
	#[error("missing a line in a pair")]
	PairMissingLine,
	#[error("missing a character where one was expected")]
	MissingCharacter,
	#[error("unexpected character: {0}")]
	UnexpectedCharacter(char),
}

#[cfg(test)]
mod packet_data_parse {
	use super::{PacketData as PD, PacketData, ParseError};

	#[test]
	fn basic_refute_asdf() {
		let input = "asdf";
		assert_eq!(
			Err(ParseError::UnexpectedCharacter('a')),
			input.parse::<PacketData>()
		);
	}

	#[test]
	fn basic_list_empty() {
		let input = "[]";
		assert_eq!(Ok(PD::List(vec![])), input.parse::<PacketData>());
	}

	#[test]
	fn basic_list_nested() {
		let input = "[[]]";
		assert_eq!(
			Ok(PD::List(vec![PD::List(vec![])])),
			input.parse::<PacketData>()
		);
		let input = "[[1,2], 3]";
		assert_eq!(
			Ok(PD::List(vec![
				PD::List(vec![PD::Integer(1), PD::Integer(2)]),
				PD::Integer(3)
			])),
			input.parse::<PacketData>()
		);
	}

	#[test]
	fn basic_num_0() {
		let input = "0";
		assert_eq!(Ok(PD::Integer(0_u32)), input.parse::<PacketData>());
	}
}

/// # Errors
pub fn parse(data: &str) -> anyhow::Result<Intermediate> {
	let pairs = data
		.split("\n\n")
		.map(str::parse)
		.collect::<Result<Vec<Pair>, ParseError>>()?;

	Ok(pairs)
}

fn compare(a: &PacketData, b: &PacketData) -> Option<Ordering> {
	use PacketData as PD;

	match (a, b) {
		(PD::Integer(a), PD::Integer(b)) if a == b => None,
		(PD::Integer(a), PD::Integer(b)) => Some(a.cmp(b)),

		(PD::List(a), PD::List(b)) => {
			let a = a.iter().map(Some).chain(core::iter::repeat(None));
			let b = b.iter().map(Some).chain(core::iter::repeat(None));
			// Careful! Both a and b are infinite iterators...

			for (a, b) in a.zip(b) {
				let result = match (a, b) {
					// If we got to the end without refuting, a and b are in order.
					(None, None) => return None,
					// If a runs out of items first, a and b are in order.
					(None, Some(_)) => Some(Ordering::Less),
					// If a runs out of items after b, a and b are not in order.
					(Some(_), None) => Some(Ordering::Greater),
					// Otherwise, compare elementwise.
					(Some(a), Some(b)) => compare(a, b),
				};

				if result.is_some() {
					return result;
				}
			}

			None
		}
		(PD::Integer(a), PD::List(b)) => {
			compare(&PD::List(vec![PD::Integer(*a)]), &PD::List(b.clone()))
		}
		(PD::List(a), PD::Integer(b)) => {
			compare(&PD::List(a.clone()), &PD::List(vec![PD::Integer(*b)]))
		}
	}
}

#[cfg(test)]
mod compare {
	use super::{Ordering::*, PacketData as PD, compare};

	#[test]
	fn int_lower() {
		assert_eq!(Some(Less), compare(&PD::Integer(1), &PD::Integer(2)));
		assert_eq!(Some(Less), compare(&PD::Integer(0), &PD::Integer(69)));
	}
}

#[must_use]
pub fn part_one(pairs: &Intermediate) -> Option<Output> {
	let mut sum = 0;
	for (idx, pair) in pairs.iter().enumerate() {
		let comparison_result = compare(&pair.0, &pair.1);
		match comparison_result {
			Some(Ordering::Greater) => {}
			_ => sum += idx + 1,
		}
	}

	Some(sum)
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("../../inputs/2022/examples/day13.example.in.txt"),
	Some(13)
);

#[must_use]
pub fn part_two(pairs: &Intermediate) -> Option<Output> {
	let mut all_packets: Vec<&PacketData> = pairs
		.iter()
		.flat_map(|pair| vec![&pair.0, &pair.1])
		.collect();

	let marker_a = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]);
	let marker_b = PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])]);

	all_packets.push(&marker_a);
	all_packets.push(&marker_b);

	all_packets.sort_by(|a, b| compare(a, b).expect("expected to get an ordering"));

	let result = [&marker_a, &marker_b]
		.map(|needle| {
			all_packets
				.iter()
				.position(|&c| c == needle)
				.expect("can't find the markers we just put in the array")
				+ 1
		})
		.iter()
		.product();

	Some(result)
}

daocutil::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("../../inputs/2022/examples/day13.example.in.txt"),
	Some(140)
);

daocutil::generate_solver!(solve, =>, self);
