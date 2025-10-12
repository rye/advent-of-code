use core::str::FromStr;

pub type Intermediate = Vec<Game>;
pub type Output = u32;

#[derive(Debug)]
pub struct Game {
	number: u32,
	handfuls: Vec<HandfulCounts>,
}

impl FromStr for Game {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (game_n, handfuls) = s.split_at(s.find(": ").expect("expected \": \""));

		let number = game_n
			.split(' ')
			.next_back()
			.expect("expected \" \" in game_n part")
			.parse()
			.expect("expected game number");

		let handfuls = handfuls[": ".len()..]
			.split("; ")
			.map(str::parse::<HandfulCounts>)
			.collect::<Result<Vec<HandfulCounts>, _>>()?;

		Ok(Game { number, handfuls })
	}
}

#[derive(Debug)]
pub struct HandfulCounts {
	red: usize,
	green: usize,
	blue: usize,
}

impl FromStr for HandfulCounts {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<Vec<&str>> = s
			.split(", ")
			.map(|part| part.split(' ').collect::<Vec<&str>>())
			.collect();

		let mut red_count: Option<usize> = None;
		let mut green_count: Option<usize> = None;
		let mut blue_count: Option<usize> = None;

		for part in parts {
			match part.get(1) {
				Some(&"red") => red_count = Some(part[0].parse().expect("expected to parse as number")),
				Some(&"green") => green_count = Some(part[0].parse().expect("expected to parse as number")),
				Some(&"blue") => blue_count = Some(part[0].parse().expect("expected to parse as number")),
				_ => {}
			}
		}

		Ok(HandfulCounts {
			red: red_count.unwrap_or(0),
			green: green_count.unwrap_or(0),
			blue: blue_count.unwrap_or(0),
		})
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input.lines().map(str::parse).collect()
}

fn game_ids_not_exceeding_counts_with_replacement(
	games: &[Game],
	red_count: usize,
	green_count: usize,
	blue_count: usize,
) -> impl Iterator<Item = u32> + '_ {
	games
		.iter()
		.filter(move |game| {
			let mut game_ok: bool = true;

			for handful in &game.handfuls {
				if handful.red > red_count || handful.blue > blue_count || handful.green > green_count {
					game_ok = false;
					break;
				}
			}

			game_ok
		})
		.map(|game| game.number)
}

#[must_use]
pub fn part_one(games: &Intermediate) -> Option<Output> {
	Some(game_ids_not_exceeding_counts_with_replacement(games, 12, 13, 14).sum())
}

crate::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day02.example.in.txt"),
	Some(8)
);

fn game_minimum_power(game: &Game) -> usize {
	let mut min_red_count: usize = 0;
	let mut min_green_count: usize = 0;
	let mut min_blue_count: usize = 0;

	for handful in &game.handfuls {
		if handful.red > min_red_count {
			min_red_count = handful.red;
		}

		if handful.green > min_green_count {
			min_green_count = handful.green;
		}

		if handful.blue > min_blue_count {
			min_blue_count = handful.blue;
		}
	}

	min_red_count * min_green_count * min_blue_count
}

#[test]
fn game_minimum_power_examples() {
	assert_eq!(
		48,
		game_minimum_power(
			&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
				.parse()
				.unwrap()
		)
	);
	assert_eq!(
		12,
		game_minimum_power(
			&"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"
				.parse()
				.unwrap()
		)
	);

	assert_eq!(
		1560,
		game_minimum_power(
			&"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
				.parse()
				.unwrap()
		)
	);

	assert_eq!(
		630,
		game_minimum_power(
			&"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
				.parse()
				.unwrap()
		)
	);

	assert_eq!(
		36,
		game_minimum_power(
			&"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
				.parse()
				.unwrap()
		)
	);
}

#[must_use]
pub fn part_two(games: &Intermediate) -> Option<Output> {
	let prod: usize = games.iter().map(game_minimum_power).sum();
	prod.try_into().ok()
}

crate::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("examples/day02.example.in.txt"),
	Some(2286)
);

crate::generate_solver!(solve, =>, self);
