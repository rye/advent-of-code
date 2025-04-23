#[derive(PartialEq, Debug)]
pub enum Line {
	Valid(Vec<char>),
	Corrupted(char),
}

#[inline]
const fn is_open(char: char) -> bool {
	matches!(char, '(' | '[' | '{' | '<')
}

const fn is_close(char: char) -> bool {
	matches!(char, ')' | ']' | '}' | '>')
}

const fn corresponding_close(open: char) -> char {
	match open {
		'(' => ')',
		'[' => ']',
		'{' => '}',
		'<' => '>',
		_ => unreachable!(),
	}
}

const fn score_found_missing_closing(char: char) -> u64 {
	match char {
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => 0,
	}
}

const fn score_completion(char: char) -> u64 {
	match char {
		')' => 1,
		']' => 2,
		'}' => 3,
		'>' => 4,
		_ => 0,
	}
}

impl core::str::FromStr for Line {
	type Err = core::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut stack: Vec<char> = Vec::new();

		let mut corruption_value: Option<char> = None;

		for c in s.chars() {
			if is_open(c) {
				stack.push(c);
			} else if is_close(c) {
				match stack.pop() {
					Some(open) if c != corresponding_close(open) => {
						corruption_value = Some(c);
						break;
					}
					Some(_open) => {}
					None => panic!("stack empty :("),
				}
			} else {
				panic!("unexpected {c}");
			}
		}

		if let Some(char) = corruption_value {
			Ok(Line::Corrupted(char))
		} else {
			Ok(Line::Valid(stack))
		}
	}
}

#[cfg(test)]
mod line {
	use super::Line;

	mod parse {
		use super::Line;

		mod valid {
			use super::Line;

			#[test]
			fn simple() {
				assert_eq!(Ok(Line::Valid(vec![])), "()".parse());
				assert_eq!(Ok(Line::Valid(vec![])), "[]".parse());
				assert_eq!(Ok(Line::Valid(vec![])), "{}".parse());
				assert_eq!(Ok(Line::Valid(vec![])), "<>".parse());
			}

			#[test]
			fn simple_nest() {
				assert_eq!(Ok(Line::Valid(vec![])), "(((((((((())))))))))".parse());
				assert_eq!(Ok(Line::Valid(vec![])), "([])".parse());
				assert_eq!(Ok(Line::Valid(vec![])), "{}".parse());
				assert_eq!(Ok(Line::Valid(vec![])), "<([{}])>".parse());
			}

			#[test]
			fn complex() {
				assert_eq!(Ok(Line::Valid(vec![])), "{()()()}".parse());
				assert_eq!(Ok(Line::Valid(vec![])), "[<>({}){}[([])<>]]".parse());
			}
		}
	}
}

pub type Intermediate = Vec<Line>;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	input.lines().map(str::parse).collect()
}

type Solution = u64;

#[must_use]
pub fn part_one(subsystem: &Intermediate) -> Option<Solution> {
	Some(subsystem.iter().fold(0_u64, |acc, line| {
		acc
			+ match line {
				Line::Corrupted(char) => score_found_missing_closing(*char),
				Line::Valid(_v) => 0_u64,
			}
	}))
}

fn score_stack(stack: &[char]) -> u64 {
	get_completion_for_stack(stack)
		.iter()
		.fold(0_u64, |acc, completion| {
			acc * 5 + score_completion(*completion)
		})
}

fn get_completion_for_stack(stack: &[char]) -> Vec<char> {
	stack
		.iter()
		.rev()
		.map(|open| corresponding_close(*open))
		.collect()
}

#[test]
fn stack_completion() {
	let stack: Vec<char> = "[({([[{{".chars().collect();

	assert_eq!(
		vec!['}', '}', ']', ']', ')', '}', ')', ']'],
		get_completion_for_stack(&stack)
	);
}

#[must_use]
pub fn part_two(subsystem: &Intermediate) -> Option<Solution> {
	use std::collections::BTreeSet;

	let valid_line_scores: BTreeSet<u64> = subsystem
		.iter()
		.filter_map(|line| match line {
			Line::Corrupted(_) => None,
			Line::Valid(stack) => Some(score_stack(stack)),
		})
		.collect();

	let valid_line_scores: Vec<u64> = valid_line_scores.into_iter().collect();

	for (idx, line) in valid_line_scores.iter().enumerate() {
		println!("{:2} {}", idx + 1, line);
	}

	Some(valid_line_scores[valid_line_scores.len() / 2])
}
