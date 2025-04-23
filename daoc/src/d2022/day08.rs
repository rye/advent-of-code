use std::collections::BTreeMap;

pub type Intermediate = BTreeMap<(u32, u32), u8>;
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let lines: BTreeMap<(u32, u32), u8> = input
		.lines()
		.enumerate()
		.flat_map(|(y, line)| {
			line
				.chars()
				.enumerate()
				.filter_map(|(x, char)| char.to_digit(10).map(|u32| (x, u32)))
				.filter_map(
					move |(x, u32)| match (u32::try_from(x), u32::try_from(y), u8::try_from(u32)) {
						(Ok(x), Ok(y), Ok(u8)) => Some(((x, y), u8)),
						_ => None,
					},
				)
		})
		.collect();

	Ok(lines)
}

const SOUTH: (i8, i8) = (0, 1);
const NORTH: (i8, i8) = (0, -1);
const EAST: (i8, i8) = (1, 0);
const WEST: (i8, i8) = (-1, 0);

fn count_visible(visibility: &BTreeMap<(u32, u32), bool>) -> usize {
	visibility.values().filter(|bool| **bool).count()
}

#[must_use]
pub fn part_one(lines: &Intermediate) -> Option<Output> {
	let mut visibility: BTreeMap<(u32, u32), bool> = BTreeMap::default();

	for ((pos_x, pos_y), height_at_position) in lines {
		let mut visible_in_any_direction: bool = false;

		for (dir_x, dir_y) in [SOUTH, NORTH, EAST, WEST] {
			let mut visible_in_direction: Option<bool> = None;

			let mut c = 1_u16;

			loop {
				let cur_x = (i64::from(*pos_x)) + i64::from(i32::from(dir_x) * i32::from(c));
				let cur_y = (i64::from(*pos_y)) + i64::from(i32::from(dir_y) * i32::from(c));

				if let (Some(cur_x), Some(cur_y)) = (u32::try_from(cur_x).ok(), u32::try_from(cur_y).ok()) {
					match lines.get(&(cur_x, cur_y)) {
						None => break,
						Some(height) if height >= height_at_position => {
							visible_in_direction = Some(false);
							break;
						}
						_ => c += 1,
					}
				} else {
					break;
				}
			}

			if let Some(visible_in_direction) = visible_in_direction {
				if visible_in_direction {
					visible_in_any_direction = true;
					break;
				}
			} else {
				visible_in_any_direction = true;
			}
		}

		visibility.insert((*pos_x, *pos_y), visible_in_any_direction);
	}

	Some(count_visible(&visibility))
}

#[test]
fn part_one_example() {
	crate::test_example!(
		"30373\n25512\n65332\n33549\n35390",
		part_one,
		parse,
		Some(21)
	);
}

#[must_use]
pub fn part_two(lines: &Intermediate) -> Option<Output> {
	let mut scores: BTreeMap<(u32, u32), usize> = BTreeMap::default();

	for ((pos_x, pos_y), height_at_position) in lines {
		let mut score = 1_usize;

		for (dir_x, dir_y) in [SOUTH, NORTH, EAST, WEST] {
			let mut c = 1_u16;

			loop {
				let cur_x = (i64::from(*pos_x)) + i64::from(i32::from(dir_x) * i32::from(c));
				let cur_y = (i64::from(*pos_y)) + i64::from(i32::from(dir_y) * i32::from(c));

				if let (Some(cur_x), Some(cur_y)) = (u32::try_from(cur_x).ok(), u32::try_from(cur_y).ok()) {
					match lines.get(&(cur_x, cur_y)) {
						None => {
							score *= (c as usize) - 1;
							break;
						}
						Some(height) if height >= height_at_position => {
							score *= c as usize;
							break;
						}
						_ => {}
					}
				} else {
					score *= (c as usize) - 1;
					break;
				}

				c += 1;
			}
		}

		scores.insert((*pos_x, *pos_y), score);
	}

	scores.values().max().copied()
}

#[test]
fn part_two_example() {
	crate::test_example!(
		"30373\n25512\n65332\n33549\n35390",
		part_two,
		parse,
		Some(8)
	);
}

crate::generate_solver!(solve, =>, self);
