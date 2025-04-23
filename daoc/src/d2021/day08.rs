use std::collections::HashSet;

pub type Intermediate<'a> = Vec<([&'a str; 10], [&'a str; 4])>;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		input
			.lines()
			.map(|line| line.split(" | "))
			.map(|mut split| (split.next().unwrap(), split.next().unwrap()))
			.map(|(signals, outputs)| {
				(
					signals.split(' ').collect::<Vec<_>>().try_into().unwrap(),
					outputs.split(' ').collect::<Vec<_>>().try_into().unwrap(),
				)
			})
			.collect(),
	)
}

type Output = usize;

#[must_use]
pub fn part_one(parts: &Intermediate) -> Option<Output> {
	Some(
		parts
			.iter()
			.flat_map(|s| s.1.iter())
			.filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
			.count(),
	)
}

fn all_possibilities() -> HashSet<char> {
	['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect()
}

const SEGMENT_TOP: usize = 0;
const SEGMENT_TOP_RIGHT: usize = 1;
const SEGMENT_BOTTOM_RIGHT: usize = 2;
const SEGMENT_BOTTOM: usize = 3;
const SEGMENT_BOTTOM_LEFT: usize = 4;
const SEGMENT_TOP_LEFT: usize = 5;
const SEGMENT_CENTER: usize = 6;

fn solve_segments(signals: &[&str; 10]) -> [HashSet<char>; 10] {
	let mut possibilities: [HashSet<char>; 7] = [
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
	];

	let signals: Vec<HashSet<char>> = signals.iter().map(|s| s.chars().collect()).collect();

	// First, we can trivially infer which of the signals corresponds to 1, 4, 7, and 8, because
	// these each have a unique number of connections.

	// One is the only signal mapping with only two parts.  Either of those two parts could be the
	// top and bottom right segments, so both are recorded as possibilities for now.
	let one_signal = signals.iter().find(|&s| s.len() == 2).unwrap();

	possibilities[SEGMENT_TOP_RIGHT].clone_from(one_signal);
	possibilities[SEGMENT_BOTTOM_RIGHT].clone_from(one_signal);

	// Four is the only signal mapping with only four parts.
	let four_signal = signals.iter().find(|&s| s.len() == 4).unwrap();

	// But two of four's parts are already in one, and so we can figure out could be in the center
	// and top left accordingly.
	possibilities[SEGMENT_CENTER] = four_signal - one_signal;
	possibilities[SEGMENT_TOP_LEFT] = four_signal - one_signal;

	// Seven is the only signal mapping with only three parts.
	let seven_signal = signals.iter().find(|&s| s.len() == 3).unwrap();

	// And again, we can use the parts from one to figure out which of the three is the top segment.
	possibilities[SEGMENT_TOP] = seven_signal - one_signal;

	// For completeness, the eight signal mapping has seven parts.  We don't need to use it anywhere
	// though, technically, since we know that, and it's the union of everything else.
	let _eight_signal = signals.iter().find(|&s| s.len() == 7).unwrap();

	// Okay, now we move on to deducing some of the other digits, starting with the digits having
	// six segments.

	// Nine is the only 6-segment signal mapping that is a superset of four.
	let nine_signal = signals
		.iter()
		.find(|&s| s.len() == 6 && s.is_superset(four_signal))
		.unwrap();

	// And once we know which signal mapping is the nine, we can figure out the only segment
	// left out, bottom left, by subtracting that from the total set of all possibilities.
	possibilities[SEGMENT_BOTTOM_LEFT] = &all_possibilities() - nine_signal;

	// Next, six is another 6-segment digit, but its special characteristic is that it is _not_ a
	// superset of the one signal mapping.
	let six_signal = signals
		.iter()
		.find(|&s| s.len() == 6 && !s.is_superset(one_signal))
		.unwrap();

	// From this, we can figure out which segment is the top-right segment...
	possibilities[SEGMENT_TOP_RIGHT] = &all_possibilities() - six_signal;

	// and refine our determination from earlier as far as what the bottom right is, since we
	// figured out which of the segments is the top-right one.
	possibilities[SEGMENT_BOTTOM_RIGHT] =
		&possibilities[SEGMENT_BOTTOM_RIGHT] - &possibilities[SEGMENT_TOP_RIGHT];

	// Lastly, zero is the last 6-segment and it contains both the top-right and bottom-left digits.
	// NOTE(rye): Could probably refine this by keeping a set of the 6-segments and removing only one,
	// since this is the _remainder_ after 9 and 6 are used.
	let zero_signal = signals
		.iter()
		.find(|&s| {
			s.len() == 6
				&& s.is_superset(&(&possibilities[SEGMENT_TOP_RIGHT] | &possibilities[SEGMENT_BOTTOM_LEFT]))
		})
		.unwrap();

	// From zero, we can figure out the center as well.
	possibilities[SEGMENT_CENTER] = &all_possibilities() - zero_signal;

	// Now, the possibilities left over from the four mapping can be reduced, since we know which of
	// the segments is the center.
	possibilities[SEGMENT_TOP_LEFT] =
		&possibilities[SEGMENT_TOP_LEFT] - &possibilities[SEGMENT_CENTER];

	// Finally, the bottom is the 9 signal minus the 4 signal and minus the top segment.
	possibilities[SEGMENT_BOTTOM] = &(nine_signal - four_signal) - &possibilities[SEGMENT_TOP];

	// Now, take unions of the appropriate segments to generate the sets of segments on a per-digit
	// basis so that callers only have to do a set comparison.
	let digit_map: [HashSet<char>; 10] = [
		// 0:
		&(&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_BOTTOM_LEFT])
			| &possibilities[SEGMENT_TOP_LEFT],
		// 1:
		&possibilities[SEGMENT_TOP_RIGHT] | &possibilities[SEGMENT_BOTTOM_RIGHT],
		// 2
		&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_BOTTOM_LEFT])
			| &possibilities[SEGMENT_CENTER],
		// 3
		&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_CENTER],
		// 4
		&(&(&possibilities[SEGMENT_TOP_LEFT] | &possibilities[SEGMENT_CENTER])
			| &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT],
		// 5
		&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_TOP_LEFT])
			| &possibilities[SEGMENT_CENTER],
		// 6
		&(&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_BOTTOM_LEFT])
			| &possibilities[SEGMENT_TOP_LEFT])
			| &possibilities[SEGMENT_CENTER],
		// 7
		&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT],
		// 8
		&(&(&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_BOTTOM_LEFT])
			| &possibilities[SEGMENT_TOP_LEFT])
			| &possibilities[SEGMENT_CENTER],
		// 9
		&(&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_TOP_LEFT])
			| &possibilities[SEGMENT_CENTER],
	];

	digit_map
}

#[test]
fn solve_segments_works() {
	use itertools::Itertools;

	const SIGNALS: [&str; 10] = [
		"acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
	];

	let signal_digits: Vec<Vec<char>> = solve_segments(&SIGNALS)
		.into_iter()
		.map(|c| c.into_iter().sorted().collect::<Vec<_>>())
		.collect();

	// Check the digits we can easily solve for.
	assert_eq!(signal_digits[1], vec!['a', 'b']);
	assert_eq!(signal_digits[4], vec!['a', 'b', 'e', 'f']);
	assert_eq!(signal_digits[7], vec!['a', 'b', 'd']);
	assert_eq!(signal_digits[8], vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);

	// Check the 6-segment digits...
	assert_eq!(signal_digits[9], vec!['a', 'b', 'c', 'd', 'e', 'f']);
	assert_eq!(signal_digits[6], vec!['b', 'c', 'd', 'e', 'f', 'g']);
	assert_eq!(signal_digits[0], vec!['a', 'b', 'c', 'd', 'e', 'g']);

	// Then check the 5-segment digits.
	assert_eq!(signal_digits[2], vec!['a', 'c', 'd', 'f', 'g']);
	assert_eq!(signal_digits[3], vec!['a', 'b', 'c', 'd', 'f']);
	assert_eq!(signal_digits[5], vec!['b', 'c', 'd', 'e', 'f']);
}

fn apply_segments(solved_digits: &[HashSet<char>; 10], outputs: &[&str; 4]) -> usize {
	let digits: Vec<usize> = outputs
		.iter()
		.map(|&output| {
			let output_chars: HashSet<char> = output.chars().collect();
			solved_digits
				.iter()
				.enumerate()
				.find(|(_idx, digit)| digit == &&output_chars)
				.map(|(idx, _parts)| idx)
				.unwrap()
		})
		.collect();

	digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]
}

#[must_use]
pub fn part_two(parts: &Intermediate) -> Option<Output> {
	Some(
		parts
			.iter()
			.map(|(signals, outputs)| apply_segments(&solve_segments(signals), outputs))
			.sum(),
	)
}

crate::generate_solver!(solve, =>, self);
