use core::{convert::Infallible, str::FromStr};

pub enum Move {
	Rock,
	Paper,
	Scissors,
}

impl Move {
	fn shape_score(&self) -> u32 {
		match self {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissors => 3,
		}
	}
}

impl FromStr for Move {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" | "A" => Ok(Self::Rock),
			"Y" | "B" => Ok(Self::Paper),
			"Z" | "C" => Ok(Self::Scissors),
			_ => unreachable!(),
		}
	}
}

pub struct StrategyPart<'p>(&'p str, &'p str);

enum Outcome {
	Win,
	Draw,
	Loss,
}

impl Outcome {
	fn outcome_score(&self) -> u32 {
		match self {
			Outcome::Win => 6,
			Outcome::Draw => 3,
			Outcome::Loss => 0,
		}
	}
}

impl StrategyPart<'_> {
	fn score_as_move_move(&self) -> u32 {
		use {
			Move::{Paper, Rock, Scissors},
			Outcome::{Draw, Loss, Win},
		};

		let self_move: Move = self.1.parse().expect("failed to parse as move");
		let opponent_move: Move = self.0.parse().expect("failed to parse as move");

		let outcome = match (&self_move, &opponent_move) {
			(Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
			(Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Loss,
			(Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
		};

		self_move.shape_score() + outcome.outcome_score()
	}

	fn score_as_move_outcome(&self) -> u32 {
		use {
			Move::{Paper, Rock, Scissors},
			Outcome::{Draw, Loss, Win},
		};

		let opponent_move: Move = self.0.parse().expect("failed to parse opponent move");

		let desired_outcome: Outcome = match self.1 {
			"X" => Loss,
			"Y" => Draw,
			"Z" => Win,
			_ => unreachable!(),
		};

		let my_choice: Move = match (opponent_move, &desired_outcome) {
			(Rock, Win) | (Paper, Draw) | (Scissors, Loss) => Paper,
			(Rock, Draw) | (Paper, Loss) | (Scissors, Win) => Rock,
			(Rock, Loss) | (Paper, Win) | (Scissors, Draw) => Scissors,
		};

		my_choice.shape_score() + desired_outcome.outcome_score()
	}
}

impl<'p> TryFrom<&'p str> for StrategyPart<'p> {
	type Error = Infallible;

	fn try_from(str: &'p str) -> Result<Self, Self::Error> {
		let (left, right) = {
			let mut split = str.split(' ');
			(
				split.next().expect("missing first piece on line"),
				split.next().expect("missing second piece on line"),
			)
		};

		Ok(Self(left, right))
	}
}

pub type Intermediate<'i> = Vec<StrategyPart<'i>>;
pub type Output = u32;

/// # Errors
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	Ok(
		str
			.lines()
			.map(TryFrom::try_from)
			.collect::<Result<Intermediate, _>>()?,
	)
}

#[must_use]
pub fn part_one(guide: &Intermediate) -> Option<Output> {
	Some(guide.iter().map(StrategyPart::score_as_move_move).sum())
}

#[cfg(test)]
mod part_one {
	use super::{parse, part_one};

	#[test]
	fn example() {
		let input = "A Y\nB X\nC Z";
		assert_eq!(part_one(&parse(input).unwrap()), Some(15));
	}
}

#[must_use]
pub fn part_two(guide: &Intermediate) -> Option<Output> {
	Some(guide.iter().map(StrategyPart::score_as_move_outcome).sum())
}

#[cfg(test)]
mod part_two {
	use super::{parse, part_two};

	#[test]
	fn example() {
		let input = "A Y\nB X\nC Z";
		assert_eq!(part_two(&parse(input).unwrap()), Some(12));
	}
}

crate::generate_solver!(solve, =>, self);
