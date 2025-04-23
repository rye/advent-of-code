use std::collections::{BTreeMap, BTreeSet};

use regex::Regex;

pub type Intermediate = (BTreeMap<(u32, u32), char>, BTreeMap<(u32, u32), u32>);
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let mut chars = BTreeMap::new();
	let mut number_spans = BTreeMap::new();

	let re = Regex::new(r"\d+").unwrap();

	for (y, line) in input.lines().enumerate() {
		let y: u32 = y.try_into().expect("too many lines");

		for ((x, y), number) in re.find_iter(line).map(|matzch| {
			(
				(
					matzch.start().try_into().expect("start exceeds u32::max"),
					y,
				),
				matzch
					.as_str()
					.parse()
					.expect("match cannot be parsed as u32"),
			)
		}) {
			number_spans.insert((x, y), number);
		}

		for (x, byte) in line.bytes().enumerate() {
			let x: u32 = x.try_into().expect("too many characters on line");
			let char: char = byte.into();

			chars.insert((x, y), char);
		}
	}

	Ok((chars, number_spans))
}

#[must_use]
pub fn part_one(schematic: &Intermediate) -> Option<Output> {
	Some(part_numbers(schematic).sum())
}

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
	(-1, 1),
	(0, 1),
	(1, 1),
	(-1, 0),
	(1, 0),
	(-1, -1),
	(0, -1),
	(1, -1),
];

fn part_numbers(schematic: &Intermediate) -> impl Iterator<Item = u32> + '_ {
	schematic
		.1
		.iter()
		.filter(|&((start_pos_x, start_pos_y), number)| {
			let number_len = format!("{number}").len();

			let digit_pos = (0..number_len).map(|xi| {
				(
					start_pos_x + u32::try_from(xi).expect("expected to be able to map digit offset to u32"),
					start_pos_y,
				)
			});

			let neighbors_to_check: BTreeSet<(u32, u32)> = digit_pos
				.flat_map(|(x, y)| {
					NEIGHBOR_OFFSETS.iter().filter_map(move |(xo, yo)| {
						let x = x.checked_add_signed(*xo);
						let y = y.checked_add_signed(*yo);

						match (x, y) {
							(Some(x), Some(y)) => Some((x, y)),
							_ => None,
						}
					})
				})
				.collect();

			neighbors_to_check
				.iter()
				.any(|neighbor| match schematic.0.get(neighbor) {
					Some('#' | '$' | '%' | '&' | '*' | '+' | '-' | '/' | '=' | '@') => true,
					Some(_) | None => false,
				})
		})
		.map(|x| x.1)
		.copied()
}

fn parts(schematic: &Intermediate) -> impl Iterator<Item = ((u32, u32), char, u32)> + '_ {
	schematic
		.1
		.iter()
		.filter_map(|((start_pos_x, start_pos_y), number)| {
			let number_len = format!("{number}").len();

			let digit_pos = (0..number_len).map(|xi| {
				(
					start_pos_x + u32::try_from(xi).expect("expected to be able to map digit offset to u32"),
					start_pos_y,
				)
			});

			let neighbors_to_check: BTreeSet<(u32, u32)> = digit_pos
				.flat_map(|(x, y)| {
					NEIGHBOR_OFFSETS.iter().filter_map(move |(xo, yo)| {
						let x = x.checked_add_signed(*xo);
						let y = y.checked_add_signed(*yo);

						match (x, y) {
							(Some(x), Some(y)) => Some((x, y)),
							_ => None,
						}
					})
				})
				.collect();

			let neighbor = neighbors_to_check
				.iter()
				.find(|neighbor| match schematic.0.get(neighbor) {
					Some('#' | '$' | '%' | '&' | '*' | '+' | '-' | '/' | '=' | '@') => true,
					Some(_) | None => false,
				});

			neighbor.map(|c| (*c, *schematic.0.get(c).expect("whoa"), *number))
		})
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("../../inputs/2023/examples/day03.example.in.txt"),
	Some(4361)
);

#[must_use]
pub fn part_two(schematic: &Intermediate) -> Option<Output> {
	let gear_candidate_locations: BTreeMap<(u32, u32), Vec<u32>> = parts(schematic)
		.filter(|(_part_pos, c, _part_num)| c == &'*')
		.fold(BTreeMap::new(), |mut map, (pos, _c, part_number)| {
			map.entry(pos).or_default().push(part_number);
			map
		});

	Some(
		gear_candidate_locations
			.iter()
			.filter(|(_loc, part_nums)| part_nums.len() == 2)
			.map(|(_loc, part_nums)| part_nums.iter().product::<u32>())
			.sum(),
	)
}

daocutil::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("../../inputs/2023/examples/day03.example.in.txt"),
	Some(467_835)
);

daocutil::generate_solver!(solve, =>, self);
