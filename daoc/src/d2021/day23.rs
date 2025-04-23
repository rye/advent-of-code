#![allow(dead_code, unused)]

use std::collections::HashSet;

// The puzzle input is given in the form
//
// ```
// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########
//
// To lay these out in a reasonable way, we have the following numbering scheme:
//
// Hallway (0) ->
// Hallway (1) ->
// Hallway (2) ->
//  -> Side Room 0-0 (11) -> Side Room 0-1 (12)
//  -> Hallway (3) ->
//

type SlotIdentifier = u8;

const ADJACENCIES: [(SlotIdentifier, SlotIdentifier); 18] = [
	// The main hallway is 11 units long:
	(0, 1),
	(1, 2),
	(2, 3),
	(3, 4),
	(4, 5),
	(5, 6),
	(6, 7),
	(7, 8),
	(8, 9),
	(9, 10),
	// The first side room is adjacent to slot 2
	(2, 11),
	(11, 12),
	// The second side room is adjacent to slot 4
	(4, 13),
	(13, 14),
	// The third side room is adjacent to slot 6
	(6, 15),
	(15, 16),
	// The fourth side room is adjacent to slot 8
	(8, 17),
	(17, 18),
];

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Amphipod {
	Amber,
	Bronze,
	Copper,
	Desert,
}

#[derive(thiserror::Error, Debug)]
pub enum AmphipodParseError {
	#[error("invalid character `{0}`")]
	InvalidCharacter(char),
	#[error("invalid Amphipod `{0}`")]
	InvalidString(String),
}

impl TryFrom<char> for Amphipod {
	type Error = AmphipodParseError;

	fn try_from(char: char) -> Result<Self, Self::Error> {
		match char {
			'A' => Ok(Self::Amber),
			'B' => Ok(Self::Bronze),
			'C' => Ok(Self::Copper),
			'D' => Ok(Self::Desert),
			e => Err(AmphipodParseError::InvalidString(e.to_string())),
		}
	}
}

impl core::str::FromStr for Amphipod {
	type Err = AmphipodParseError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		if str.len() == 1 {
			let char = str.chars().next().unwrap();
			char.try_into()
		} else {
			Err(AmphipodParseError::InvalidString(str.to_string()))
		}
	}
}

pub type Intermediate = HashSet<(Amphipod, SlotIdentifier)>;

#[inline]
fn is_blank_space(char: char) -> bool {
	char == '.'
}

#[inline]
fn is_not_blank(char: char) -> bool {
	!is_blank_space(char)
}

#[inline]
fn is_wall(char: char) -> bool {
	char == '#'
}

#[inline]
fn is_not_wall(char: char) -> bool {
	!is_wall(char)
}

#[derive(thiserror::Error, Debug)]
enum MoveCalculationError {
	#[error("move was blocked")]
	MoveWasBlocked,
}

fn cost_to_move(
	state: &HashSet<(Amphipod, SlotIdentifier)>,
	(amphipod, slot): &(Amphipod, SlotIdentifier),
	destination: SlotIdentifier,
) -> Result<usize, MoveCalculationError> {
	//debug_assert!(state.contains(&(amphipod, slot)));
	todo!()
}

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	const HALLWAY_INDICES: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

	const ROOM_INDICES_ROW_1: [u8; 4] = [11, 13, 15, 17];
	const ROOM_INDICES_ROW_2: [u8; 4] = [12, 14, 16, 18];

	let input_lines: Vec<&str> = input.lines().collect();

	let hallway_line = input_lines[1];
	debug_assert_eq!(hallway_line.chars().filter(|&c| is_wall(c)).count(), 2);

	let room_line_row_1 = input_lines[2];
	debug_assert_eq!(room_line_row_1.chars().filter(|&c| is_wall(c)).count(), 9);
	let room_line_row_2 = input_lines[3];
	debug_assert_eq!(room_line_row_2.chars().filter(|&c| is_wall(c)).count(), 5);

	let hallway_chars = hallway_line.chars().filter(|&c| c != '#' && c != ' ');
	let room_line_row_1_chars = room_line_row_1.chars().filter(|&c| c != '#' && c != ' ');
	let room_line_row_2_chars = room_line_row_2.chars().filter(|&c| c != '#' && c != ' ');

	let hallway = hallway_chars
		.zip(HALLWAY_INDICES)
		.filter(|&(c, _)| c != '.')
		.map(|(c, idx)| (c.try_into().unwrap(), idx));

	let room_line_1 = room_line_row_1_chars
		.zip(ROOM_INDICES_ROW_1)
		.filter(|&(c, _)| c != '.')
		.map(|(c, idx)| (c.try_into().unwrap(), idx));

	let room_line_2 = room_line_row_2_chars
		.zip(ROOM_INDICES_ROW_2)
		.filter(|&(c, _)| c != '.')
		.map(|(c, idx)| (c.try_into().unwrap(), idx));

	Ok(hallway.chain(room_line_1).chain(room_line_2).collect())
}

#[test]
fn parse_example() {
	let example = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

	assert_eq!(
		parse(example),
		Ok(
			[
				(Amphipod::Amber, 12),
				(Amphipod::Amber, 18),
				(Amphipod::Bronze, 11),
				(Amphipod::Bronze, 15),
				(Amphipod::Copper, 13),
				(Amphipod::Copper, 16),
				(Amphipod::Desert, 14),
				(Amphipod::Desert, 17),
			]
			.into_iter()
			.collect()
		)
	);
}

#[test]
fn parse_example_1() {
	let example = "#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########";

	assert_eq!(
		parse(example),
		Ok(
			[
				(Amphipod::Amber, 9),
				(Amphipod::Amber, 12),
				(Amphipod::Bronze, 13),
				(Amphipod::Bronze, 14),
				(Amphipod::Copper, 15),
				(Amphipod::Copper, 16),
				(Amphipod::Desert, 17),
				(Amphipod::Desert, 18),
			]
			.into_iter()
			.collect()
		)
	);
}

type Output = i32;

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}

crate::generate_solver!(solve, =>, self);
