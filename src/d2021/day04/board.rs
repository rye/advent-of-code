use std::collections::BTreeSet;

use super::{Error, Number};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Board {
	contents: [[Number; 5]; 5],
	winning_moves: Vec<BTreeSet<Number>>,
	all_contents: BTreeSet<Number>,
}

fn parse_board_line(line: &str) -> Result<[Number; 5], Error> {
	let result: Result<Vec<Number>, Error> = line
		.split_ascii_whitespace()
		.map(str::parse)
		.collect::<Result<Vec<Number>, _>>()
		.map_err(Error::from);

	match result {
		Ok(vec) => <[Number; 5]>::try_from(vec).map_err(|_| Error::BoardParse),
		Err(e) => Err(e),
	}
}

fn parse_board_block(board: &str) -> Result<[[Number; 5]; 5], Error> {
	let result: Result<Vec<[Number; 5]>, Error> = board
		.split('\n')
		.map(parse_board_line)
		.collect::<Result<Vec<[Number; 5]>, Error>>();

	match result {
		Ok(vec) => <[[Number; 5]; 5]>::try_from(vec).map_err(|_| Error::BoardParse),
		Err(e) => Err(e),
	}
}

impl Board {
	fn generate_winning_moves(contents: &[[Number; 5]; 5]) -> Vec<BTreeSet<Number>> {
		let mut winning_moves = Vec::new();

		for &row in contents {
			let rank_set: BTreeSet<Number> = BTreeSet::from(row);
			winning_moves.push(rank_set);
		}

		for column in 0..5 {
			let mut file_set: BTreeSet<Number> = BTreeSet::new();

			for &row in contents {
				file_set.insert(row[column]);
			}

			winning_moves.push(file_set);
		}

		winning_moves
	}

	#[must_use]
	pub fn score(&self, last_call: Number, seen_calls: &BTreeSet<Number>) -> u32 {
		u32::from(
			self
				.numbers()
				.difference(seen_calls)
				.map(|&n| u16::from(n))
				.sum::<u16>(),
		) * u32::from(last_call)
	}

	#[must_use]
	pub fn numbers(&self) -> &BTreeSet<Number> {
		&self.all_contents
	}

	#[must_use]
	pub fn find_winning_move(&self, seen_calls: &BTreeSet<Number>) -> Option<&BTreeSet<Number>> {
		self
			.winning_moves
			.iter()
			.find(|winning_move| seen_calls.is_superset(winning_move))
	}

	fn from_contents(contents: [[Number; 5]; 5]) -> Self {
		let winning_moves = Self::generate_winning_moves(&contents);

		let all_contents: BTreeSet<Number> = contents
			.iter()
			.flat_map(|row| row.iter().copied())
			.collect();

		Self {
			contents,
			winning_moves,
			all_contents,
		}
	}
}

impl core::str::FromStr for Board {
	type Err = Error;

	fn from_str(board: &str) -> Result<Self, Self::Err> {
		parse_board_block(board).map(Board::from_contents)
	}
}

#[cfg(test)]
mod parse_board_line {
	use super::{parse_board_line, Error};

	#[test]
	fn line_62_5_77_94_75() {
		assert_eq!(parse_board_line("62  5 77 94 75"), Ok([62, 5, 77, 94, 75]));
	}

	#[test]
	fn line_62_5_77_94() {
		assert_eq!(parse_board_line("62  5 77 94"), Err(Error::BoardParse));
	}

	#[test]
	fn line_62_5_77_94_asdf() {
		assert!(parse_board_line("62  5 77 94 asdf").is_err());
	}
}
