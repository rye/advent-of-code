use itertools::Itertools;

pub type Intermediate = Vec<char>;
pub type Solution = usize;

pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	match input.lines().next() {
		Some(line) => Ok(line.chars().collect()),
		_ => panic!("no input given"),
	}
}

#[must_use] pub fn part_one(chars: &Intermediate) -> Option<Solution> {
	Some(
		chars
			.iter()
			.circular_tuple_windows()
			.filter(|(a, b)| a == b)
			.map(|(a, _b)| a)
			.filter_map(|c| c.to_digit(10))
			.sum::<u32>() as usize,
	)
}

daocutil::test_example!(part_one_1122, parse, part_one, "1122", Some(3));
daocutil::test_example!(part_one_1111, parse, part_one, "1111", Some(4));
daocutil::test_example!(part_one_1234, parse, part_one, "1234", Some(0));
daocutil::test_example!(part_one_91212129, parse, part_one, "91212129", Some(9));

#[must_use] pub fn part_two(chars: &Intermediate) -> Option<Solution> {
	let mut sum: u32 = 0;
	let halfway: usize = chars.len() / 2;
	for i in 0..halfway {
		if chars[i] == chars[halfway + i] {
			sum += chars[i].to_digit(10).unwrap();
			sum += chars[halfway + i].to_digit(10).unwrap();
		}
	}
	Some(sum as usize)
}

daocutil::test_example!(part_two_1212, parse, part_two, "1212", Some(6));
daocutil::test_example!(part_two_1221, parse, part_two, "1221", Some(0));
daocutil::test_example!(part_two_123425, parse, part_two, "123425", Some(4));
daocutil::test_example!(part_two_123123, parse, part_two, "123123", Some(12));
daocutil::test_example!(part_two_12131415, parse, part_two, "12131415", Some(4));
