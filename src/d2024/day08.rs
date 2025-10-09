use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Frequency(char);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State {
	Antenna(Frequency),
}

#[derive(Clone, Debug)]
struct Map {
	width: usize,
	height: usize,

	antennas: HashMap<Frequency, BTreeSet<(usize, usize)>>,
	antinodes: BTreeSet<(usize, usize)>,
}

impl Map {
	#[allow(dead_code)]
	fn draw(&self, antinodes_over_antennas: bool) {
		for y in 0..self.height {
			for x in 0..self.width {
				if self.antinodes.contains(&(x, y)) && antinodes_over_antennas {
					print!("#");
				} else if let Some(freq) = self
					.antennas
					.iter()
					.find_map(|(freq, locations)| locations.contains(&(x, y)).then_some(freq))
				{
					print!("{}", freq.0);
				} else if !antinodes_over_antennas && self.antinodes.contains(&(x, y)) {
					print!("#");
				} else {
					print!(".");
				}
			}
			println!();
		}
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, map: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let map_chars: Vec<Vec<char>> = map
			.lines()
			.map(move |line| line.chars().collect())
			.collect();

		let width = map_chars.first().map_or(0, Vec::len);
		let height = map_chars.len();

		let map: Vec<(usize, usize, State)> = map_chars
			.iter()
			.enumerate()
			.flat_map(|(y, line)| {
				line.iter().enumerate().filter_map(move |(x, c)| {
					if c.is_alphanumeric() {
						Some((x, y, State::Antenna(Frequency(*c))))
					} else {
						None
					}
				})
			})
			.collect();

		let antennas = map.iter().fold(
			HashMap::<Frequency, BTreeSet<(usize, usize)>>::new(),
			|mut acc, (x, y, state)| {
				match state {
					State::Antenna(freq) => {
						acc.entry(*freq).or_default().insert((*x, *y));
					}
				}
				acc
			},
		);

		let antinodes = BTreeSet::new();

		let map = Map {
			width,
			height,
			antennas,
			antinodes,
		};

		Ok(Box::new(map))
	}

	fn part_one(&self, map: &dyn core::any::Any) -> Option<String> {
		let map = map.downcast_ref::<Map>()?;

		let mut map: Map = map.to_owned();

		// Read from map.antennas, write to map.antinodes.
		for antenna_locations in map.antennas.values() {
			for locations in antenna_locations.iter().permutations(2) {
				// Invariant: permutations(2) always returns Vec of length 2.
				// Assumption is that locations are not going to be above isize::MAX.
				let locations = locations.clone();

				let (new_x, new_y) = {
					let start = locations.first().unwrap();
					let end = locations.last().unwrap();

					let dx = isize::try_from(end.0).unwrap() - isize::try_from(start.0).unwrap();
					let dy = isize::try_from(end.1).unwrap() - isize::try_from(start.1).unwrap();

					let x_prime = end.0.checked_add_signed(dx);
					let y_prime = end.1.checked_add_signed(dy);

					(x_prime, y_prime)
				};

				let Some(x) = new_x else {
					continue;
				};

				let Some(y) = new_y else {
					continue;
				};

				if (0..map.width).contains(&x) && (0..map.height).contains(&y) {
					map.antinodes.insert((x, y));
				}
			}
		}

		Some(map.antinodes.len().to_string())
	}

	fn part_two(&self, map: &dyn core::any::Any) -> Option<String> {
		let map = map.downcast_ref::<Map>()?;

		let mut map: Map = map.to_owned();

		// Read from map.antennas, write to map.antinodes.
		for antenna_locations in map.antennas.values() {
			for locations in antenna_locations.iter().permutations(2) {
				// Invariant: permutations(2) always returns Vec of length 2.
				// Assumption is that locations are not going to be above isize::MAX.
				let locations = locations.clone();

				let next_antinodes = {
					let start = locations.first().unwrap();
					let end = locations.last().unwrap();

					let dx = isize::try_from(end.0).unwrap() - isize::try_from(start.0).unwrap();
					let dy = isize::try_from(end.1).unwrap() - isize::try_from(start.1).unwrap();

					let end: (usize, usize) = **end;

					(0_isize..).scan(end, move |state, multiplier| {
						let new_x = state.0.checked_add_signed(dx * multiplier);
						let new_y = state.1.checked_add_signed(dy * multiplier);

						let x = new_x?;
						let y = new_y?;

						if (0..map.width).contains(&x) && (0..map.height).contains(&y) {
							Some((x, y))
						} else {
							None
						}
					})
				};

				for (x, y) in next_antinodes {
					map.antinodes.insert((x, y));
				}
			}
		}

		Some(map.antinodes.len().to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day08.example.in.txt", part_one, literal "14");

part_test!(part_two, Solution, file "day08.example.in.txt", part_two, literal "34");
