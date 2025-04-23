#![allow(dead_code)]

use std::collections::VecDeque;

type Card = u8;

#[derive(Debug, Clone)]
struct Deck(VecDeque<Card>);

impl Deck {
	fn is_exhausted(&self) -> bool {
		self.0.is_empty()
	}
}

#[derive(Debug)]
pub struct Game {
	players: Vec<Deck>,
}

pub type Intermediate = Game;
type Output = usize;

#[derive(Debug, PartialEq)]
struct Player(usize);

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let players: Vec<Deck> = input
		.split("\n\n")
		.map(|block| {
			Deck(
				block
					.lines()
					.skip(1)
					.map(|line| line.parse::<Card>().expect("invalid input"))
					.collect::<VecDeque<Card>>(),
			)
		})
		.collect();

	Ok(Game { players })
}

fn round_is_playable(deck_a: &Deck, deck_b: &Deck) -> bool {
	!deck_a.0.is_empty() && !deck_b.0.is_empty()
}

#[derive(Debug, PartialEq)]
enum RoundResults {
	RoundWon(Player),
	GameWon(GameResults),
}

#[derive(Debug, PartialEq)]
struct GameResults {
	winner: Player,
	score: usize,
}

fn deck_score(deck: &Deck) -> usize {
	let cards = deck.0.iter().rev();
	let multipliers = 1..=deck.0.len();

	cards
		.zip(multipliers)
		.map(|(card, multiplier)| *card as usize * multiplier)
		.sum()
}

#[cfg(test)]
mod deck_score {
	use super::*;

	#[test]
	fn example() {
		let deck = Deck([3_u8, 2, 10, 6, 8, 5, 9, 4, 7, 1].iter().copied().collect());
		assert_eq!(deck_score(&deck), 306_usize);
	}
}

fn play_one_round<'round>(deck_a: &'round mut Deck, deck_b: &'round mut Deck) -> RoundResults {
	if round_is_playable(deck_a, deck_b) {
		let a_plays = deck_a
			.0
			.pop_front()
			.expect("round is playable but deck is empty?");
		let b_plays = deck_b
			.0
			.pop_front()
			.expect("round is playable but deck is empty?");

		if a_plays > b_plays {
			deck_a.0.push_back(a_plays);
			deck_a.0.push_back(b_plays);
			RoundResults::RoundWon(Player(0_usize))
		} else {
			deck_b.0.push_back(b_plays);
			deck_b.0.push_back(a_plays);
			RoundResults::RoundWon(Player(1_usize))
		}
	} else {
		let winner = if deck_a.is_exhausted() {
			Player(1_usize)
		} else {
			Player(0_usize)
		};

		let score = match winner {
			Player(0) => deck_score(deck_a),
			Player(1) => deck_score(deck_b),
			Player(_) => unreachable!(),
		};

		RoundResults::GameWon(GameResults { winner, score })
	}
}

#[cfg(test)]
mod play_one_round {
	use super::*;

	#[test]
	fn example_round_1() {
		let mut player_a = Deck([9, 2, 6, 3, 1].iter().copied().collect());
		let mut player_b = Deck([5, 8, 4, 7, 10].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(0_usize)));

		assert_eq!(player_a.0, [2, 6, 3, 1, 9, 5]);
		assert_eq!(player_b.0, [8, 4, 7, 10]);
	}

	#[test]
	fn example_round_2() {
		let mut player_a = Deck([2, 6, 3, 1, 9, 5].iter().copied().collect());
		let mut player_b = Deck([8, 4, 7, 10].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(1_usize)));

		assert_eq!(player_a.0, [6, 3, 1, 9, 5]);
		assert_eq!(player_b.0, [4, 7, 10, 8, 2]);
	}

	#[test]
	fn example_round_3() {
		let mut player_a = Deck([6, 3, 1, 9, 5].iter().copied().collect());
		let mut player_b = Deck([4, 7, 10, 8, 2].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(0_usize)));

		assert_eq!(player_a.0, [3, 1, 9, 5, 6, 4]);
		assert_eq!(player_b.0, [7, 10, 8, 2]);
	}

	#[test]
	fn example_round_4() {
		let mut player_a = Deck([3, 1, 9, 5, 6, 4].iter().copied().collect());
		let mut player_b = Deck([7, 10, 8, 2].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(1_usize)));

		assert_eq!(player_a.0, [1, 9, 5, 6, 4]);
		assert_eq!(player_b.0, [10, 8, 2, 7, 3]);
	}

	#[test]
	fn example_round_5() {
		let mut player_a = Deck([1, 9, 5, 6, 4].iter().copied().collect());
		let mut player_b = Deck([10, 8, 2, 7, 3].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(1_usize)));

		assert_eq!(player_a.0, [9, 5, 6, 4]);
		assert_eq!(player_b.0, [8, 2, 7, 3, 10, 1]);
	}

	#[test]
	fn example_round_27() {
		let mut player_a = Deck([5, 4, 1].iter().copied().collect());
		let mut player_b = Deck([8, 9, 7, 3, 2, 10, 6].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(1_usize)));

		assert_eq!(player_a.0, [4, 1]);
		assert_eq!(player_b.0, [9, 7, 3, 2, 10, 6, 8, 5]);
	}

	#[test]
	fn example_round_28() {
		let mut player_a = Deck([4, 1].iter().copied().collect());
		let mut player_b = Deck([9, 7, 3, 2, 10, 6, 8, 5].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(1_usize)));

		assert_eq!(player_a.0, [1]);
		assert_eq!(player_b.0, [7, 3, 2, 10, 6, 8, 5, 9, 4]);
	}

	#[test]
	fn example_round_29() {
		let mut player_a = Deck([1].iter().copied().collect());
		let mut player_b = Deck([7, 3, 2, 10, 6, 8, 5, 9, 4].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(result, RoundResults::RoundWon(Player(1_usize)));

		assert_eq!(player_a.0, []);
		assert_eq!(player_b.0, [3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
	}

	#[test]
	fn example_round_29_next_win() {
		let mut player_a = Deck([].iter().copied().collect());
		let mut player_b = Deck([3, 2, 10, 6, 8, 5, 9, 4, 7, 1].iter().copied().collect());

		let result = play_one_round(&mut player_a, &mut player_b);

		assert_eq!(player_a.0, []);
		assert_eq!(player_b.0, [3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);

		assert_eq!(
			result,
			RoundResults::GameWon(GameResults {
				winner: Player(1),
				score: 306_usize
			})
		);
	}
}

fn play(mut deck_a: Deck, mut deck_b: Deck) -> GameResults {
	loop {
		match play_one_round(&mut deck_a, &mut deck_b) {
			RoundResults::RoundWon(_) => {}
			RoundResults::GameWon(results) => break results,
		}
	}
}

#[cfg(test)]
mod play {
	use super::{Deck, Player, play};

	#[test]
	fn example_full_game() {
		let player_a = Deck([9, 2, 6, 3, 1].iter().copied().collect());
		let player_b = Deck([5, 8, 4, 7, 10].iter().copied().collect());

		let results = play(player_a, player_b);

		assert_eq!(results.winner, Player(1));
		assert_eq!(results.score, 306_usize);
	}
}

pub fn part_one(game: &Game) -> Option<Output> {
	let player_a: Deck = game.players[0].clone();
	let player_b: Deck = game.players[1].clone();

	let game_results: GameResults = play(player_a, player_b);

	let score = game_results.score;

	Some(score)
}

pub fn part_two(_game: &Game) -> Option<Output> {
	None
}

daocutil::generate_solver!(solve, =>, self);
