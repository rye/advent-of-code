pub type Intermediate = Vec<Instruction>;
pub type Output = usize;

#[derive(Debug, PartialEq)]
enum Action {
	TurnOn,
	TurnOff,
	Toggle,
}

#[derive(Debug, PartialEq)]
struct Coord {
	x: u16,
	y: u16,
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(core::str::FromStr::from_str)
			.filter_map(Result::ok)
			.collect(),
	)
}

#[must_use]
pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	let mut grid: Grid = Grid::default();

	for instruction in instructions {
		grid.apply_instruction_misinterpretation(instruction);
	}

	Some(grid.num_lights_lit())
}

#[must_use]
pub fn part_two(instructions: &Intermediate) -> Option<Output> {
	let mut grid: Grid = Grid::default();

	for instruction in instructions {
		grid.apply_instruction_correct(instruction);
	}

	Some(grid.total_brightness())
}

impl core::str::FromStr for Coord {
	type Err = Error;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		let coords: Vec<&str> = string.split(',').collect();

		if coords.len() == 2 {
			let x = coords[0].parse::<u16>().ok();
			let y = coords[1].parse::<u16>().ok();

			if let (Some(x), Some(y)) = (x, y) {
				Ok(Coord { x, y })
			} else {
				Err(Error::CoordParse)
			}
		} else {
			Err(Error::CoordParse)
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
	action: Action,
	start: Coord,
	end: Coord,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	#[error("required keyword 'through' not found in instruction range specifier")]
	ThroughNotFound,
	#[error("Coord could not be parsed")]
	CoordParse,
	#[error("missing a required part of an instruction")]
	MissingPart,
	#[error("unrecognized token")]
	UnrecognizedToken,
}

impl core::str::FromStr for Instruction {
	type Err = Error;

	fn from_str(string: &str) -> Result<Self, Self::Err> {
		let chunks: Vec<&str> = string.split(' ').collect();

		let action: Action = match chunks.first() {
			Some(&"toggle") => Ok(Action::Toggle),
			Some(&"turn") => match chunks.get(1) {
				Some(&"on") => Ok(Action::TurnOn),
				Some(&"off") => Ok(Action::TurnOff),
				_ => Err(Error::UnrecognizedToken),
			},
			_ => Err(Error::UnrecognizedToken),
		}?;

		let through_index: usize = chunks
			.iter()
			.position(|&chunk| chunk == "through")
			.ok_or(Error::ThroughNotFound)?;

		let start: Coord = chunks
			.get(through_index - 1)
			.ok_or(Error::MissingPart)?
			.parse::<Coord>()?;

		let end: Coord = chunks
			.get(through_index + 1)
			.ok_or(Error::MissingPart)?
			.parse::<Coord>()?;

		Ok(Instruction { action, start, end })
	}
}

#[cfg(test)]
mod instruction_fromstr {
	use super::{Action, Coord, Instruction};
	#[test]
	fn turn_on_0_0_through_999_999() {
		assert_eq!(
			"turn on 0,0 through 999,999".parse::<Instruction>(),
			Ok(Instruction {
				action: Action::TurnOn,
				start: Coord { x: 0, y: 0 },
				end: Coord { x: 999, y: 999 },
			})
		);
	}

	#[test]
	fn toggle_0_0_through_999_0() {
		assert_eq!(
			"toggle 0,0 through 999,0".parse::<Instruction>(),
			Ok(Instruction {
				action: Action::Toggle,
				start: Coord { x: 0, y: 0 },
				end: Coord { x: 999, y: 0 },
			})
		);
	}

	#[test]
	fn turn_off_499_499_through_500_500() {
		assert_eq!(
			"turn off 499,499 through 500,500".parse::<Instruction>(),
			Ok(Instruction {
				action: Action::TurnOff,
				start: Coord { x: 499, y: 499 },
				end: Coord { x: 500, y: 500 },
			})
		);
	}
}

#[derive(Clone, Copy)]
enum LightState {
	Off,
	On(u16),
}

impl core::ops::Not for LightState {
	type Output = Self;
	fn not(self) -> Self {
		match self {
			Self::Off => Self::On(1),
			Self::On(_) => Self::Off,
		}
	}
}

#[derive(Clone, Copy)]
struct Light(LightState);

struct Grid {
	lights: Vec<Vec<Light>>,
}

impl Default for Grid {
	fn default() -> Self {
		Self {
			lights: vec![vec![Light(LightState::Off); 1000]; 1000],
		}
	}
}

impl Grid {
	fn apply_instruction_misinterpretation(&mut self, instruction: &Instruction) {
		let Instruction { action, start, end } = instruction;

		for y in start.y..=end.y {
			for x in start.x..=end.x {
				let new_state: LightState = match action {
					Action::Toggle => !self.lights[y as usize][x as usize].0,
					Action::TurnOn => LightState::On(1),
					Action::TurnOff => LightState::Off,
				};

				self.lights[y as usize][x as usize].0 = new_state;
			}
		}
	}

	fn apply_instruction_correct(&mut self, instruction: &Instruction) {
		let Instruction { action, start, end } = instruction;

		for y in start.y..=end.y {
			for x in start.x..=end.x {
				let old_state = self.lights[y as usize][x as usize].0;

				let new_state: LightState = match (old_state, action) {
					(LightState::On(brightness), Action::Toggle) => LightState::On(brightness + 2),
					(LightState::On(brightness), Action::TurnOn) => LightState::On(brightness + 1),
					(LightState::On(brightness), Action::TurnOff) => {
						if brightness > 1 {
							LightState::On(brightness - 1)
						} else {
							LightState::Off
						}
					}
					(LightState::Off, Action::Toggle) => LightState::On(2),
					(LightState::Off, Action::TurnOn) => LightState::On(1),
					(LightState::Off, Action::TurnOff) => LightState::Off,
				};

				self.lights[y as usize][x as usize].0 = new_state;
			}
		}
	}

	fn num_lights_lit(&self) -> usize {
		self
			.lights
			.iter()
			.map(|row| {
				row
					.iter()
					.map(|light| light.0)
					.map(|state| match state {
						LightState::On(_) => 1,
						LightState::Off => 0,
					})
					.sum::<usize>()
			})
			.sum()
	}

	fn total_brightness(&self) -> usize {
		self
			.lights
			.iter()
			.map(|row| {
				row
					.iter()
					.map(|light| light.0)
					.map(|state| match state {
						LightState::On(value) => value as usize,
						LightState::Off => 0,
					})
					.sum::<usize>()
			})
			.sum()
	}
}

crate::generate_solver!(solve, =>, self);
