use {
	core::{num::ParseIntError, ops::Deref, str::FromStr},
	std::collections::BTreeSet,
};

pub type Intermediate = Vec<(Assignment, Assignment)>;
pub type Output = usize;

#[derive(Debug)]
pub struct Assignment(BTreeSet<u32>);

impl Deref for Assignment {
	type Target = BTreeSet<u32>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl FromStr for Assignment {
	type Err = ParseIntError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let split: Vec<&str> = str.split('-').collect();
		let contents: BTreeSet<u32> = ((split[0].parse()?)..=(split[1].parse()?)).collect();
		Ok(Self(contents))
	}
}

#[derive(thiserror::Error, Debug)]
enum AssignmentParseError {
	#[error("failed to parse line: {0}")]
	LineParse(&'static str),
}

/// # Errors
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	str
		.lines()
		.map(|line| {
			let mut assignments = line.split(',');

			match (
				assignments.next().map(str::parse),
				assignments.next().map(str::parse),
				assignments.next(),
			) {
				(Some(Ok(p0)), Some(Ok(p1)), None) => Ok((p0, p1)),
				(Some(_), Some(_), Some(_)) | (Some(_), None, _) | (None, _, _) => {
					Err(AssignmentParseError::LineParse("incorrect number of parts"))?
				}
				(Some(_), Some(_), None) => Err(AssignmentParseError::LineParse(
					"failed to parse one of the assignments",
				))?,
			}
		})
		.collect::<Result<Vec<(Assignment, Assignment)>, _>>()
}

#[must_use]
pub fn part_one(assignments: &Intermediate) -> Option<Output> {
	Some(
		assignments
			.iter()
			.filter(|(left, right)| left.is_superset(right) || right.is_superset(left))
			.count(),
	)
}

#[must_use]
pub fn part_two(assignments: &Intermediate) -> Option<Output> {
	Some(
		assignments
			.iter()
			.filter(|(left, right)| left.intersection(right).count() > 0)
			.count(),
	)
}

crate::generate_solver!(solve, =>, self);
