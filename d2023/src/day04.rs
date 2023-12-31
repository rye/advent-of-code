use std::{collections::BTreeSet, ops::ShlAssign};

#[derive(Clone, Debug)]
pub struct Card {
	number: u32,

	winning_numbers: BTreeSet<u32>,
	our_numbers: BTreeSet<u32>,
}

impl core::str::FromStr for Card {
	type Err = anyhow::Error;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let (card_part, rest) = str.split_once(':').unwrap();
		let (winning_numbers_part, numbers_we_have_part) = rest.split_once('|').unwrap();

		let card_number_pieces: Vec<&str> = card_part.split_ascii_whitespace().collect();
		let winning_numbers_pieces: Vec<&str> = winning_numbers_part.split_ascii_whitespace().collect();
		let numbers_we_have_pieces: Vec<&str> = numbers_we_have_part.split_ascii_whitespace().collect();

		debug_assert_eq!(card_number_pieces[0], "Card");

		let number = card_number_pieces[1].parse()?;

		let winning_numbers: BTreeSet<u32> = winning_numbers_pieces
			.into_iter()
			.map(str::parse::<u32>)
			.collect::<Result<BTreeSet<u32>, _>>()?;

		let our_numbers = numbers_we_have_pieces
			.into_iter()
			.map(str::parse::<u32>)
			.collect::<Result<BTreeSet<u32>, _>>()?;

		Ok(Card {
			number,
			winning_numbers,
			our_numbers,
		})
	}
}

pub type Intermediate = Vec<Card>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input.lines().map(str::parse).collect()
}

#[must_use]
pub fn part_one(cards: &Intermediate) -> Option<Output> {
	fn score_card(card: &Card) -> u32 {
		let matches = card.winning_numbers.intersection(&card.our_numbers).count();

		let mut score = 0_u32;

		for _ in 0..matches {
			if score == 0 {
				score = 1;
			} else {
				score.shl_assign(1);
			}
		}

		score
	}

	Some(cards.iter().map(score_card).sum())
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day04"),
	Some(13)
);

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
