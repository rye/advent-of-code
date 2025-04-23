use core::{
	borrow::Borrow,
	cmp::Ordering,
	ops::{Add, Mul},
	str::FromStr,
};

use std::collections::HashSet;

pub type Intermediate = (Wire, Wire, Vec<Vec2d>);
pub type Output = i32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let wires: Vec<Wire> = input
		.lines()
		.map(|s| -> Vec<Segment> {
			s.split(',')
				.map(|k: &str| k.parse::<Segment>())
				.map(Result::unwrap)
				.collect()
		})
		.map(Into::into)
		.collect();

	let a = wires[0].clone();
	let b = wires[1].clone();
	let intersections: Vec<Vec2d> = a.intersection(&b);

	Ok((a, b, intersections))
}

#[must_use]
pub fn part_one((_a, _b, intersections): &Intermediate) -> Option<Output> {
	let origin: Vec2d = Vec2d(0, 0);

	let mut intersection_distances: Vec<(Vec2d, i32)> = intersections
		.iter()
		.map(|intersection: &Vec2d| -> (Vec2d, i32) {
			(*intersection, intersection.manhattan_distance(&origin))
		})
		.collect();

	intersection_distances
		.sort_by(|a: &(Vec2d, i32), b: &(Vec2d, i32)| -> Ordering { a.1.cmp(&b.1) });

	Some(intersection_distances[1].1)
}

#[must_use]
pub fn part_two((a, b, intersections): &Intermediate) -> Option<Output> {
	let mut intersection_signal_distances: Vec<(Vec2d, i32, i32, i32)> = intersections
		.iter()
		.map(|intersection: &Vec2d| {
			let a: i32 = a
				.signal_distance_to(intersection)
				.expect("intersection not on wire a?");
			let b: i32 = b
				.signal_distance_to(intersection)
				.expect("intersection not on wire b?");
			(*intersection, a, b, a + b)
		})
		.collect();

	intersection_signal_distances.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

	Some(intersection_signal_distances[1].3)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2d(pub i32, pub i32);

impl Vec2d {
	pub fn manhattan_distance(&self, other: &Self) -> i32 {
		(self.0 - other.0).abs() + (self.1 - other.1).abs()
	}
}

impl Mul<i32> for Vec2d {
	type Output = Vec2d;

	fn mul(self, n: i32) -> Self::Output {
		Vec2d(self.0 * n, self.1 * n)
	}
}

impl<T: Borrow<Vec2d>> Add<T> for Vec2d {
	type Output = Vec2d;

	fn add(self, vec: T) -> Self::Output {
		Vec2d(self.0 + vec.borrow().0, self.1 + vec.borrow().1)
	}
}

impl From<Direction> for Vec2d {
	fn from(d: Direction) -> Vec2d {
		match d {
			Direction::Up => Vec2d(0, 1),
			Direction::Down => Vec2d(0, -1),
			Direction::Left => Vec2d(-1, 0),
			Direction::Right => Vec2d(1, 0),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug)]
pub enum SegmentParseError {
	MissingComponent,
	InvalidDirection,
}

impl TryFrom<char> for Direction {
	type Error = SegmentParseError;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'U' => Ok(Direction::Up),
			'D' => Ok(Direction::Down),
			'L' => Ok(Direction::Left),
			'R' => Ok(Direction::Right),
			_ => Err(SegmentParseError::InvalidDirection),
		}
	}
}

#[derive(Debug)]
pub struct Segment {
	direction: Direction,
	distance: i32,
}

#[allow(dead_code)]
impl Segment {
	pub fn direction(&self) -> &Direction {
		&self.direction
	}

	pub fn distance(&self) -> &i32 {
		&self.distance
	}
}

impl FromStr for Segment {
	type Err = SegmentParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let direction: char = s
			.chars()
			.nth(0)
			.ok_or(SegmentParseError::MissingComponent)?;
		let distance: i32 = s[1..]
			.parse()
			.or(Err(SegmentParseError::MissingComponent))?;

		let direction: Direction = direction.try_into()?;

		Ok(Segment {
			direction,
			distance,
		})
	}
}

#[derive(Clone, Debug)]
pub struct Wire {
	points: Vec<Vec2d>,
}

impl Wire {
	pub fn intersection(&self, other: &Wire) -> Vec<Vec2d> {
		let self_points: HashSet<&Vec2d> = self.points.iter().collect();
		let other_points: HashSet<&Vec2d> = other.points.iter().collect();

		self_points
			.intersection(&other_points)
			.copied()
			.copied()
			.collect()
	}

	pub fn signal_distance_to(&self, point: &Vec2d) -> Option<i32> {
		self
			.points
			.iter()
			.position(|self_point| self_point == point)
			.map(|us| us.try_into().unwrap())
	}
}

impl From<Vec<Segment>> for Wire {
	fn from(segments: Vec<Segment>) -> Wire {
		let origin: Vec2d = Vec2d(0, 0);
		let mut points: Vec<Vec2d> = Vec::new();
		points.push(origin);

		let mut last: Vec2d = origin;

		for segment in segments {
			let direction: Direction = segment.direction;
			let direction: Vec2d = direction.into();

			for i in 1..=segment.distance {
				points.push(last + direction * i);
			}

			last = last + direction * segment.distance;
		}

		Wire { points }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_parse_segment() {
		let segment: Segment = "D42".parse().unwrap();
		assert_eq!(segment.direction(), &Direction::Down);
		assert_eq!(segment.distance(), &42_i32);
	}

	#[test]
	fn can_add_vecs() {
		let v1: Vec2d = Vec2d(0, 0);
		let v2: Vec2d = Vec2d(3, 5);

		assert_eq!(v1 + v2, Vec2d(3, 5));
	}
}

crate::generate_solver!(solve, =>, self);
