use {core::cmp::Reverse, std::collections::BinaryHeap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrabPosition(i32);

pub type Intermediate = Vec<CrabPosition>;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		input
			.trim()
			.split(',')
			.map(str::parse)
			.map(Result::unwrap)
			.map(CrabPosition)
			.collect::<Vec<CrabPosition>>(),
	)
}

type Output = i32;

#[cfg(test)]
fn example_positions() -> impl Iterator<Item = CrabPosition> {
	[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
		.into_iter()
		.map(CrabPosition)
}

fn cost_to_align_one_linear(position: CrabPosition, target: CrabPosition) -> i32 {
	(target.0 - position.0).abs()
}

fn cost_to_align_one_revised(position: CrabPosition, target: CrabPosition) -> i32 {
	let difference = (target.0 - position.0).abs();

	(difference * (difference + 1)) / 2
}

#[test]
fn example_cost_to_align_one_revised() {
	let position = CrabPosition(16);
	let target = CrabPosition(5);
	assert_eq!(cost_to_align_one_revised(position, target), 66);
}

fn cost_to_align_all<F>(
	positions: &[CrabPosition],
	chosen_position: CrabPosition,
	calculator: F,
) -> i32
where
	F: Fn(CrabPosition, CrabPosition) -> i32,
{
	positions
		.iter()
		.copied()
		.map(|position| calculator(position, chosen_position))
		.sum()
}

#[test]
fn example_cost_to_align_all_2_min() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(
		cost_to_align_all(&positions, CrabPosition(2), cost_to_align_one_linear),
		37
	);
}

#[test]
fn example_cost_to_align_all_1() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(
		cost_to_align_all(&positions, CrabPosition(1), cost_to_align_one_linear),
		41
	);
}

#[test]
fn example_cost_to_align_all_3() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(
		cost_to_align_all(&positions, CrabPosition(3), cost_to_align_one_linear),
		39
	);
}

#[test]
fn example_cost_to_align_all_10() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(
		cost_to_align_all(&positions, CrabPosition(10), cost_to_align_one_linear),
		71
	);
}

fn min_fuel<F>(positions: &[CrabPosition], individual_cost_calculator: F) -> i32
where
	F: Fn(CrabPosition, CrabPosition) -> i32 + Copy,
{
	let (min, max) = positions.iter().fold((None, None), |(min, max), position| {
		(
			match min {
				Some(cur_min) if position < cur_min => Some(position),
				Some(cur_min) => Some(cur_min),
				None => Some(position),
			},
			match max {
				Some(cur_max) if position > cur_max => Some(position),
				Some(cur_max) => Some(cur_max),
				None => Some(position),
			},
		)
	});

	let min = min.unwrap();
	let max = max.unwrap();

	let position_costs: BinaryHeap<Reverse<i32>> = (min.0..=max.0)
		.map(|pos| {
			Reverse(cost_to_align_all(
				positions,
				CrabPosition(pos),
				individual_cost_calculator,
			))
		})
		.collect();

	position_costs.peek().unwrap().0
}

#[test]
fn example_min_fuel() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(min_fuel(&positions, cost_to_align_one_linear), 37);
}

pub fn part_one(crabs: &Intermediate) -> Option<Output> {
	Some(min_fuel(crabs, cost_to_align_one_linear))
}

pub fn part_two(crabs: &Intermediate) -> Option<Output> {
	Some(min_fuel(crabs, cost_to_align_one_revised))
}

crate::generate_solver!(solve, =>, self);
