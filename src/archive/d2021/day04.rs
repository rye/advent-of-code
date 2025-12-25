use core::num::ParseIntError;
use std::collections::{BTreeSet, HashSet};

type Number = u8;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
	#[error("failed to parse a number")]
	IntegerParse(#[from] ParseIntError),

	#[error("failed to parse board")]
	BoardParse,
}

mod board;
pub use board::*;

pub type Intermediate = (Vec<Number>, Vec<Board>);

/// Parses the input down to a list of calls and a set of boards.
pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let mut results = input.split("\n\n");
	let calls = results.next();
	let boards = results;

	let calls: Vec<Number> = calls
		.unwrap_or("")
		.split(',')
		.map(str::parse)
		.filter_map(Result::ok)
		.collect();

	let boards: Vec<Board> = boards.map(str::parse).filter_map(Result::ok).collect();

	Ok((calls, boards))
}

struct Turn<'a> {
	call: Number,
	winners: Vec<&'a Board>,
	calls_so_far: &'a [Number],
}

/// From starting conditions, produces an iterator over all of the 'turns'.
///
/// (A 'turn' consists of information about what happens when you call out a number. Some of the
/// boards are winners, and the remaining boards have some number of their spaces filled by the
/// calls you have made so far.)
fn record_all_rounds<'a>(
	calls: &'a [Number],
	boards: &'a [Board],
) -> impl Iterator<Item = Turn<'a>> {
	calls.iter().enumerate().scan(
		(boards.iter().collect(), BTreeSet::new()),
		|(remaining_boards, seen_calls): &mut (HashSet<&Board>, BTreeSet<Number>), (idx, &call)| {
			seen_calls.insert(call);

			let mut winners: Vec<&'a Board> = vec![];

			for &board in &*remaining_boards {
				if board.numbers().intersection(seen_calls).count() < 5 {
					continue;
				}
				if let Some(_winning_move) = board.find_winning_move(seen_calls) {
					winners.push(board);
				}
			}

			for &board in &winners {
				remaining_boards.remove(board);
			}

			let calls_so_far: &'a [Number] = &calls[0..=idx];

			Some(Turn {
				call,
				winners,
				calls_so_far,
			})
		},
	)
}

type Output = usize;

#[must_use]
pub fn part_one((calls, boards): &Intermediate) -> Option<Output> {
	// Since we are only looking for the _first_ winning board in this step, we use `Iterator::find`
	// to stop at the first case.
	let Turn {
		call,
		winners,
		calls_so_far,
	} = record_all_rounds(calls, boards)
		.find(|turn| !turn.winners.is_empty())
		.unwrap();

	let last_call = call;
	let winning_board = winners[0];
	let seen_calls = calls_so_far.iter().copied().collect();

	let score: u32 = winning_board.score(last_call, &seen_calls);

	Some(score as usize)
}

#[must_use]
pub fn part_two((calls, boards): &Intermediate) -> Option<Output> {
	// Sadly, Bingo games are temporal. If you want to find the last board, you do _actually_ have
	// to play all the boards out, since you get very different results depending on the order in
	// which you call numbers. (In hindsight, this is obvious.)
	let all_turns: Vec<Turn> = record_all_rounds(calls, boards).collect();

	let last_turn = all_turns
		.iter()
		.rfind(|Turn { winners, .. }| !winners.is_empty())
		.unwrap();

	let Turn {
		call,
		winners,
		calls_so_far,
	} = last_turn;

	let &last_call = call;
	let winning_board = winners[0];
	let seen_calls = calls_so_far.iter().copied().collect();

	let score: u32 = winning_board.score(last_call, &seen_calls);

	Some(score as usize)
}

crate::generate_solver!(solve, =>, self);
