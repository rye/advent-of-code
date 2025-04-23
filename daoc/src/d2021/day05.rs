use {
	core::str::FromStr,
	std::{
		collections::{BTreeMap, btree_map::Entry},
		convert::Infallible,
	},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vector(i16, i16);

impl Vector {
	fn unit_and_steps(self) -> (Vector, i16) {
		let unit: (i16, i16) = (
			if self.0 == 0 {
				0
			} else {
				self.0 / self.0.abs()
			},
			if self.1 == 0 {
				0
			} else {
				self.1 / self.1.abs()
			},
		);
		let steps: i16 = if unit.0 == 0 {
			self.1 / unit.1
		} else {
			self.0 / unit.0
		};

		(Vector(unit.0, unit.1), steps)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point(Vector);

impl Point {
	fn step(&self, v: Vector) -> impl Iterator<Item = Point> + '_ {
		let (unit, steps) = v.unit_and_steps();

		(0..=steps).map(move |i| Point(Vector(self.0.0 + unit.0 * i, self.0.1 + unit.1 * i)))
	}
}

impl core::ops::Sub for Point {
	type Output = Vector;

	fn sub(self, rhs: Self) -> Self::Output {
		Vector((self.0).0 - (rhs.0).0, (self.0).1 - (rhs.0).1)
	}
}

impl core::ops::Add<Vector> for Point {
	type Output = Point;

	fn add(self, rhs: Vector) -> Self::Output {
		Self(Vector(self.0.0 + rhs.0, self.0.1 + rhs.1))
	}
}

impl FromStr for Point {
	// I think you will find this FromStr quite fallible!
	// For the input, though, this works just fine.
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(',');

		let x = split.next().map(str::parse);
		let y = split.next().map(str::parse);

		match (x, y) {
			(Some(Ok(x)), Some(Ok(y))) => Ok(Self(Vector(x, y))),
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod point {
	use super::{Point, Vector};

	#[test]
	fn from_str_simple() {
		assert_eq!("1,2".parse(), Ok(Point(Vector(1, 2))));
	}
}

#[derive(Clone, Copy, Debug)]
pub struct LineSegment {
	origin: Point,
	direction: Vector,
}

impl LineSegment {
	fn from_points(a: Point, b: Point) -> Self {
		let origin = a;
		let direction = b - a;

		Self { origin, direction }
	}

	fn is_horizontal(self) -> bool {
		self.direction.1 == 0
	}

	fn is_vertical(self) -> bool {
		self.direction.0 == 0
	}

	#[cfg(test)]
	fn is_diagonal(self) -> bool {
		self.direction.0 != 0 && (self.direction.1 / self.direction.0).abs() == 1
	}

	fn points(&self) -> impl Iterator<Item = Point> + '_ {
		self.origin.step(self.direction)
	}
}

#[test]
fn points_diagonal_asc() {
	let segment = LineSegment::from_points(Point(Vector(1, 1)), Point(Vector(3, 3)));
	assert!(segment.is_diagonal());

	let points: Vec<Point> = segment.points().collect();
	assert_eq!(
		points,
		vec![
			Point(Vector(1, 1)),
			Point(Vector(2, 2)),
			Point(Vector(3, 3))
		]
	);
}

#[test]
fn points_diagonal_dsc() {
	let segment = LineSegment::from_points(Point(Vector(9, 7)), Point(Vector(7, 9)));
	assert!(segment.is_diagonal());

	let points: Vec<Point> = segment.points().collect();
	assert_eq!(
		points,
		vec![
			Point(Vector(9, 7)),
			Point(Vector(8, 8)),
			Point(Vector(7, 9))
		]
	);
}

pub type Intermediate = Vec<LineSegment>;

impl FromStr for LineSegment {
	type Err = Infallible;

	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let points: Vec<Point> = line
			.split(" -> ")
			.map(str::parse)
			.collect::<Result<Vec<_>, Infallible>>()?;

		assert_eq!(points.len(), 2);

		Ok(LineSegment::from_points(points[0], points[1]))
	}
}

pub fn parse(input: &str) -> Result<Intermediate, Infallible> {
	input
		.lines()
		.map(str::parse)
		.collect::<Result<Vec<LineSegment>, Infallible>>()
}

type Output = usize;

#[must_use]
pub fn part_one(segments: &Intermediate) -> Option<Output> {
	let segments: Vec<LineSegment> = segments
		.iter()
		.filter(|segment| segment.is_horizontal() || segment.is_vertical())
		.copied()
		.collect();

	let mut points: BTreeMap<Point, usize> = BTreeMap::new();

	for segment in segments {
		for point in segment.points() {
			match points.entry(point) {
				Entry::Occupied(mut e) => e.insert(e.get() + 1),
				Entry::Vacant(e) => *e.insert(1),
			};
		}
	}

	let overlaps: usize = points
		.iter()
		.filter(|&(&_point, &count)| count >= 2)
		.count();

	Some(overlaps)
}

#[must_use]
pub fn part_two(segments: &Intermediate) -> Option<Output> {
	let mut points: BTreeMap<Point, usize> = BTreeMap::new();

	for segment in segments {
		for point in segment.points() {
			match points.entry(point) {
				Entry::Occupied(mut e) => e.insert(e.get() + 1),
				Entry::Vacant(e) => *e.insert(1),
			};
		}
	}

	let overlaps: usize = points
		.iter()
		.filter(|&(&_point, &count)| count >= 2)
		.count();

	Some(overlaps)
}

crate::generate_solver!(solve, =>, self);
