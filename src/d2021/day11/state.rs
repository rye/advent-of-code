use super::neighbors::neighbors;

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
struct EnergyLevel(u8);

impl From<u8> for EnergyLevel {
	fn from(u8: u8) -> Self {
		Self(u8)
	}
}

impl From<char> for EnergyLevel {
	fn from(char: char) -> Self {
		EnergyLevel(match char {
			'0' => 0_u8,
			'1' => 1,
			'2' => 2,
			'3' => 3,
			'4' => 4,
			'5' => 5,
			'6' => 6,
			'7' => 7,
			'8' => 8,
			'9' => 9,
			_ => panic!("unrecognized energy level!"),
		})
	}
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
struct Octopus(EnergyLevel);

impl From<char> for Octopus {
	fn from(char: char) -> Self {
		Self(char.into())
	}
}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct State<const N: usize> {
	octopi: [[Octopus; N]; N],
}

impl<const N: usize> core::str::FromStr for State<N> {
	type Err = core::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let octopi: [[Octopus; N]; N] = s
			.lines()
			.map(|line| {
				let octopi: [Octopus; N] = line
					.chars()
					.map(std::convert::Into::into)
					.collect::<Vec<_>>()
					.try_into()
					.expect("failed to parse {N} octopi on a line");
				octopi
			})
			.collect::<Vec<_>>()
			.try_into()
			.expect("failed to parse {N} lines of octopi");

		Ok(State { octopi })
	}
}

impl<const N: usize> State<N> {
	pub fn tick(&mut self) -> usize {
		use std::collections::VecDeque;

		assert!(N <= u8::MAX.into());

		let mut flashed_cells: [[bool; N]; N] = [[false; N]; N];
		let mut flashes_seen: usize = 0_usize; // This *could* be reconstructed from flashed later.

		let mut increases: VecDeque<(u8, u8)> = VecDeque::new();

		// First, the energy level of each octopus increases by 1.
		for (y, x) in (0..N).flat_map(|y| {
			(0..N).map(move |x| {
				// This "as" is safe due to assert at start.
				(u8::try_from(y).unwrap(), u8::try_from(x).unwrap())
			})
		}) {
			increases.push_back((y, x));
		}

		// Loop until there are no more increases to handle.
		while let Some((y, x)) = increases.pop_front() {
			let y_idx: usize = y as usize;
			let x_idx: usize = x as usize;

			// If we already flashed in this spot, we're done with this increase.
			if flashed_cells[y_idx][x_idx] {
				continue;
			}

			{
				// First, bump the value.
				self.octopi[y_idx][x_idx].0 .0 += 1;

				// If that caused the octopus to have an energy level greater than 9, it flashes.
				if self.octopi[y_idx][x_idx].0 .0 > 9 {
					// Mark this cell as flashed and reset its value to 0.
					flashes_seen += 1;
					flashed_cells[y_idx][x_idx] = true;
					self.octopi[y_idx][x_idx].0 .0 = 0;

					// And also causes its (unflashed) neighbors to increase in level.
					for (y, x) in neighbors::<N>(y, x) {
						increases.push_back((y, x));
					}
				}
			}
		}

		flashes_seen
	}
}

#[cfg(test)]
mod tick {
	use super::{EnergyLevel, Octopus, State};

	macro_rules! state {
		($lvl:literal) => {
			Octopus(EnergyLevel($lvl))
		};

		[$($lvl:literal)+] => {
			[$(state!($lvl)),+]
		};

		[
			$(
				$($lit:literal)+
			),+ ,
		] => {
			State {
				octopi: [
				$(
					state![$($lit)+],
				)+
				]
			}
		};
	}

	#[cfg(test)]
	mod _5 {
		use super::{EnergyLevel, Octopus, State};

		fn state_0() -> State<5> {
			state![
				1 1 1 1 1,
				1 9 9 9 1,
				1 9 1 9 1,
				1 9 9 9 1,
				1 1 1 1 1,
			]
		}

		fn state_1() -> State<5> {
			state![
				3 4 5 4 3,
				4 0 0 0 4,
				5 0 0 0 5,
				4 0 0 0 4,
				3 4 5 4 3,
			]
		}

		fn state_2() -> State<5> {
			state![
				4 5 6 5 4,
				5 1 1 1 5,
				6 1 1 1 6,
				5 1 1 1 5,
				4 5 6 5 4,
			]
		}

		#[test]
		fn tick_flow() {
			let mut state = state_0();
			assert_eq!(state, state_0());
			assert_eq!(state.tick(), 9_usize);
			assert_eq!(state, state_1());
			assert_eq!(state.tick(), 0_usize);
			assert_eq!(state, state_2());
		}
	}

	#[cfg(test)]
	mod _10 {
		use super::{EnergyLevel, Octopus, State};

		fn state_0() -> State<10> {
			state![
				5 4 8 3 1 4 3 2 2 3,
				2 7 4 5 8 5 4 7 1 1,
				5 2 6 4 5 5 6 1 7 3,
				6 1 4 1 3 3 6 1 4 6,
				6 3 5 7 3 8 5 4 7 8,
				4 1 6 7 5 2 4 6 4 5,
				2 1 7 6 8 4 1 7 2 1,
				6 8 8 2 8 8 1 1 3 4,
				4 8 4 6 8 4 8 5 5 4,
				5 2 8 3 7 5 1 5 2 6,
			]
		}

		fn state_1() -> State<10> {
			state![
				6 5 9 4 2 5 4 3 3 4,
				3 8 5 6 9 6 5 8 2 2,
				6 3 7 5 6 6 7 2 8 4,
				7 2 5 2 4 4 7 2 5 7,
				7 4 6 8 4 9 6 5 8 9,
				5 2 7 8 6 3 5 7 5 6,
				3 2 8 7 9 5 2 8 3 2,
				7 9 9 3 9 9 2 2 4 5,
				5 9 5 7 9 5 9 6 6 5,
				6 3 9 4 8 6 2 6 3 7,
			]
		}

		#[test]
		fn tick_flow() {
			let mut state = state_0();
			assert_eq!(state, state_0());
			assert_eq!(state.tick(), 0_usize);
			assert_eq!(state, state_1());
			assert_eq!(state.tick(), 35_usize);
		}
	}
}
