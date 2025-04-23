#![allow(dead_code, unused)]

use std::collections::{BTreeMap, BTreeSet, HashSet};

pub struct HeightMap {
	points: BTreeMap<(u32, u32), u32>,
}

fn neighbors(x: u32, y: u32) -> impl Iterator<Item = (u32, u32)> {
	let neighbor_offsets: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

	neighbor_offsets
		.into_iter()
		.filter_map(move |(x_offset, y_offset)| {
			if let (Some(x), Some(y)) = (
				u32::try_from(i32::try_from(x).unwrap() + x_offset).ok(),
				u32::try_from(i32::try_from(y).unwrap() + y_offset).ok(),
			) {
				Some((x, y))
			} else {
				None
			}
		})
}

impl HeightMap {
	fn low_points(&self) -> impl Iterator<Item = (&(u32, u32), &u32)> {
		self.points.iter().filter(|((x, y), height)| {
			let neighbors: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

			for (x_offset, y_offset) in neighbors {
				if let (Some(x), Some(y)) = (
					u32::try_from(i32::try_from(*x).unwrap() + x_offset).ok(),
					u32::try_from(i32::try_from(*y).unwrap() + y_offset).ok(),
				) {
					if let Some(neighbor_value) = self.points.get(&(x, y)) {
						if neighbor_value <= height {
							return false;
						}
					}
				}
			}

			true
		})
	}

	fn height_at(&self, (x, y): (u32, u32)) -> u32 {
		match self.points.get(&(x, y)) {
			Some(height) => *height,
			None => 9,
		}
	}

	fn count(&self) -> usize {
		self.points.len()
	}
}

pub type Intermediate = HeightMap;

pub fn parse(points: &str) -> Result<Intermediate, core::convert::Infallible> {
	let points = points
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line.chars().enumerate().map(move |(x, c)| {
				(
					(x.try_into().unwrap(), y.try_into().unwrap()),
					c.to_digit(10).unwrap(),
				)
			})
		})
		.collect();

	Ok(HeightMap { points })
}

type Output = u32;

#[must_use]
pub fn part_one(height_map: &Intermediate) -> Option<Output> {
	let low_points = height_map.low_points().map(|(_, height)| height);

	let risk_levels = low_points.map(|height| height + 1).sum();

	Some(risk_levels)
}

#[must_use]
pub fn part_two(height_map: &Intermediate) -> Option<Output> {
	use std::collections::{
		hash_map::Entry,
		{BTreeMap, BTreeSet}, {HashMap, HashSet, VecDeque},
	};

	// First, figure out all the basin sizes for all the low points.
	let set_sizes = height_map
		.low_points()
		.map(|((x, y), _height)| (basin_size(height_map, (*x, *y)), (*x, *y)))
		.collect::<BTreeSet<(u32, (u32, u32))>>();

	// Take the top three.
	let top_three = set_sizes.iter().rev().take(3);

	// And then product of their sizes.
	Some(top_three.map(|(size, _point)| size).product())
}

fn basin_size(height_map: &HeightMap, (x, y): (u32, u32)) -> u32 {
	use std::collections::{
		hash_map::Entry,
		{BTreeMap, BTreeSet}, {HashMap, HashSet, VecDeque},
	};

	let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
	queue.push_back((x, y));

	let mut set: HashSet<(u32, u32)> = HashSet::new();

	// Pop a point, add it to the set, and then add its higher-level neighbors.
	while let Some((x, y)) = queue.pop_front() {
		set.insert((x, y));

		let height = height_map.height_at((x, y));

		for (nx, ny) in neighbors(x, y) {
			let neighbor_height = height_map.height_at((nx, ny));

			if neighbor_height == 9 {
				continue;
			}

			if neighbor_height > height {
				queue.push_back((nx, ny));
			}
		}
	}

	u32::try_from(set.len()).expect("set size should not exceed u32::MAX")
}

#[test]
fn part_two_examples() {
	let string = "2199943210
3987894921
9856789892
8767896789
9899965678";
	let height_map = parse(string).expect("failed to parse");

	assert_eq!(3, basin_size(&height_map, (1, 0)));
	assert_eq!(9, basin_size(&height_map, (9, 0)));
	assert_eq!(14, basin_size(&height_map, (2, 2)));
	assert_eq!(9, basin_size(&height_map, (6, 4)));

	assert_eq!(Some(1134), part_two(&height_map));
}

daocutil::generate_solver!(solve, =>, self);
