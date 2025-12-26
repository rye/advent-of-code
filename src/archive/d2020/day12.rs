#![allow(unused_imports)]

use core::str::FromStr;
use std::collections::*;
use std::io::{Read, stdin};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
	North,
	South,
	East,
	West,
	Left,
	Right,
	Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
	action: Action,
	value: f64,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(str: &str) -> Result<Self, ()> {
		let action = match str.chars().next() {
			Some('N') => Action::North,
			Some('S') => Action::South,
			Some('E') => Action::East,
			Some('W') => Action::West,
			Some('L') => Action::Left,
			Some('R') => Action::Right,
			Some('F') => Action::Forward,
			_ => todo!(),
		};

		let value = str[1..].parse::<f64>().unwrap();

		Ok(Self { action, value })
	}
}

pub type Intermediate = Vec<Instruction>;
pub type Output = f64;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		input
			.lines()
			.map(|line| line.parse().expect("invalid line"))
			.collect(),
	)
}

pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	#[derive(Debug, Clone)]
	struct ShipState {
		position: (f64, f64),
		orientation: (f64, f64),
	}

	let ship = ShipState {
		position: (0.0, 0.0),
		orientation: (1.0, 0.0),
	};

	let final_state: ShipState =
		instructions
			.iter()
			.fold(ship, |ship_state: ShipState, instruction: &Instruction| {
				let pos = ship_state.position;
				let vec = ship_state.orientation;

				match (instruction.action, instruction.value) {
					(Action::North, v) => ShipState {
						position: (pos.0, pos.1 + v),
						orientation: vec,
					},
					(Action::South, v) => ShipState {
						position: (pos.0, pos.1 - v),
						orientation: vec,
					},
					(Action::East, v) => ShipState {
						position: (pos.0 + v, pos.1),
						orientation: vec,
					},
					(Action::West, v) => ShipState {
						position: (pos.0 - v, pos.1),
						orientation: vec,
					},
					(Action::Left, deg) => {
						let heading = (vec.1).atan2(vec.0).to_degrees();

						let heading = (heading + deg).to_radians();

						ShipState {
							position: pos,
							orientation: (heading.cos(), heading.sin()),
						}
					}
					(Action::Right, deg) => {
						let heading = (vec.1).atan2(vec.0).to_degrees();

						let heading = (heading - deg).to_radians();

						ShipState {
							position: pos,
							orientation: (heading.cos(), heading.sin()),
						}
					}
					(Action::Forward, units) => ShipState {
						position: (pos.0 + vec.0 * units, pos.1 + vec.1 * units),
						orientation: vec,
					},
				}
			});

	Some(final_state.position.0.abs() + final_state.position.1.abs())
}

#[test]
fn part_one_example() {
	let instructions: Intermediate = parse("F10\nN3\nF7\nR90\nF11").unwrap();
	assert_eq!(part_one(&instructions), Some(25.0_f64));
}

pub fn part_two(instructions: &Intermediate) -> Option<Output> {
	#[derive(Debug, Clone)]
	struct ShipState {
		position: (f64, f64),
		waypoint: (f64, f64),
	}

	let ship = ShipState {
		position: (0.0, 0.0),
		waypoint: (10.0, 1.0),
	};

	let final_state: ShipState =
		instructions
			.iter()
			.fold(ship, |ship_state: ShipState, instruction: &Instruction| {
				let position = ship_state.position;
				let waypoint = ship_state.waypoint;

				match (instruction.action, instruction.value) {
					(Action::North, v) => ShipState {
						position,
						waypoint: (waypoint.0, waypoint.1 + v),
					},
					(Action::South, v) => ShipState {
						position,
						waypoint: (waypoint.0, waypoint.1 - v),
					},
					(Action::East, v) => ShipState {
						position,
						waypoint: (waypoint.0 + v, waypoint.1),
					},
					(Action::West, v) => ShipState {
						position,
						waypoint: (waypoint.0 - v, waypoint.1),
					},

					(Action::Left, deg) => {
						// Compute the sine and cosine of the specified angle.
						let sin = deg.to_radians().sin();
						let cos = deg.to_radians().cos();

						// Translate coordinate system such that ship is origin
						let about_origin: (f64, f64) = (waypoint.0 - position.0, waypoint.1 - position.1);

						// Apply rotation matrix
						let new_waypoint_offset: (f64, f64) = (
							about_origin.0 * cos - about_origin.1 * sin,
							about_origin.0 * sin + about_origin.1 * cos,
						);

						// Translate back into absolute coordinates
						let waypoint = (
							(new_waypoint_offset.0 + position.0).round(),
							(new_waypoint_offset.1 + position.1).round(),
						);

						ShipState { position, waypoint }
					}
					(Action::Right, deg) => {
						// Compute the sine and cosine of the specified angle.
						let sin = (-deg).to_radians().sin();
						let cos = (-deg).to_radians().cos();

						// Translate coordinate system such that ship is origin
						let about_origin: (f64, f64) = (waypoint.0 - position.0, waypoint.1 - position.1);

						// Apply rotation matrix
						let new_waypoint_offset: (f64, f64) = (
							about_origin.0 * cos - about_origin.1 * sin,
							about_origin.0 * sin + about_origin.1 * cos,
						);

						// Translate back into absolute coordinates
						let waypoint = (
							(new_waypoint_offset.0 + position.0).round(),
							(new_waypoint_offset.1 + position.1).round(),
						);

						ShipState { position, waypoint }
					}
					(Action::Forward, times) => {
						let mut state = ship_state;
						let dx = state.waypoint.0 - state.position.0;
						let dy = state.waypoint.1 - state.position.1;

						#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
						for _ in 0..(times.trunc() as u64) {
							state.position.0 += dx;
							state.position.1 += dy;
						}

						state.waypoint.0 = state.position.0 + dx;
						state.waypoint.1 = state.position.1 + dy;

						state
					}
				}
			});

	Some(final_state.position.0.abs() + final_state.position.1.abs())
}

#[test]
fn part_two_example() {
	let instructions: Intermediate = parse("F10\nN3\nF7\nR90\nF11").unwrap();
	assert_eq!(part_two(&instructions), Some(286.0_f64));
}

crate::generate_solver!(solve, =>, self);
