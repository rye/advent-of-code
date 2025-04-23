use {core::hash::Hash, std::collections::HashSet};

pub type Intermediate = Vec<char>;
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.lines().flat_map(str::chars).collect())
}

fn find_end_of_first_non_identical_seq<T: Copy + Hash + Eq>(
	length: usize,
	slice: &[T],
) -> Option<usize> {
	// Loop over all the windows, enumerating the _start-of-window_ indices as we go.
	slice
		.windows(length)
		.enumerate()
		// Count the number of unique elements in T and reduce the iterator to (idx, unique element count).
		.map(|(idx, window)| (idx, window.iter().copied().collect::<HashSet<T>>().len()))
		// Look for the idx of the first window that has a unique number of elements matching
		// our desired window length, then return the idx following the end of that window.
		.find_map(|(idx, unique_count)| match unique_count {
			sz if sz == length => Some(idx + sz),
			_ => None,
		})
}

#[must_use]
pub fn part_one(chars: &Intermediate) -> Option<Output> {
	find_end_of_first_non_identical_seq(4, chars)
}

#[test]
fn part_one_example() {
	let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(7)
	);
}

#[test]
fn part_one_other_examples() {
	let example = "bvwbjplbgvbhsrlpgdmjqwftvncz";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(5)
	);

	let example = "nppdvjthqldpwncqszvftbrmjlhg";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(6)
	);

	let example = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(10)
	);

	let example = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(11)
	);
}

#[must_use]
pub fn part_two(chars: &Intermediate) -> Option<Output> {
	find_end_of_first_non_identical_seq(14, chars)
}

#[test]
fn part_two_examples() {
	daocutil::test_examples!(
		parse,
		part_two,
		"mjqjpqmgbljsphdztnvjfqwrcgsmlb" => Some(19),
		"bvwbjplbgvbhsrlpgdmjqwftvncz" => Some(23),
		"nppdvjthqldpwncqszvftbrmjlhg" => Some(23),
		"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => Some(29),
		"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => Some(26),
	);
}

daocutil::generate_example_tests!(parse, part_two,
	part_two_example_0 | "mjqjpqmgbljsphdztnvjfqwrcgsmlb" => Some(19),
	part_two_example_1 | "bvwbjplbgvbhsrlpgdmjqwftvncz" => Some(23),
	part_two_example_2 | "nppdvjthqldpwncqszvftbrmjlhg" => Some(23),
	part_two_example_3 | "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => Some(29),
	part_two_example_4 | "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => Some(26),
);

daocutil::generate_solver!(solve, =>, self);
