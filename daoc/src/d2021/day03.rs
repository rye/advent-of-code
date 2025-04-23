use core::{cmp::Ordering, str::Chars};

pub type Intermediate = Vec<[char; 12]>;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		input
			.lines()
			.map(str::chars)
			.map(Chars::collect::<Vec<_>>)
			.map(<[char; 12]>::try_from)
			.map(Result::unwrap)
			.collect(),
	)
}

// Each string in the input is 12 bits long. This means that they fit in a u16.  However,
// we need to be able to multiply without overflow.  (log(2^16 * 2^16) / log(2) == 32).
type Output = u32;

fn bit_count(mut accumulator: [[usize; 2]; 12], string: &[char; 12]) -> [[usize; 2]; 12] {
	for idx in 0..12 {
		match string[idx] {
			'0' => accumulator[idx][0] += 1,
			'1' => accumulator[idx][1] += 1,
			_ => unreachable!(),
		}
	}

	accumulator
}

pub fn part_one(strings: &Intermediate) -> Option<Output> {
	let statistics: [[usize; 2]; 12] = strings.iter().fold([[0; 2]; 12], bit_count);

	let gamma_rate_bits: String = statistics
		.iter()
		.map(|[zc, oc]| match oc.cmp(zc) {
			Ordering::Greater => '1',
			Ordering::Less => '0',
			Ordering::Equal => unreachable!(),
		})
		.collect();

	let gamma_rate: u32 = u32::from_str_radix(&gamma_rate_bits, 2).unwrap();

	let epsilon_rate: u32 = !gamma_rate & 0b0000_1111_1111_1111;

	Some(gamma_rate * epsilon_rate)
}

const fn find_keep_bit(statistics: &[usize; 2], mode: Mode) -> char {
	if statistics[0] <= statistics[1] {
		match mode {
			Mode::KeepMostCommonOrOne => '1',
			Mode::KeepLeastCommonOrZero => '0',
		}
	} else {
		match mode {
			Mode::KeepMostCommonOrOne => '0',
			Mode::KeepLeastCommonOrZero => '1',
		}
	}
}

#[derive(Clone, Copy)]
enum Mode {
	KeepMostCommonOrOne,
	KeepLeastCommonOrZero,
}

fn find_component_rating(strings: &[[char; 12]], mode: Mode) -> String {
	let mut idx = 0;

	let mut partial: Vec<char> = vec![];

	let mut strings: Vec<[char; 12]> = strings.to_owned();

	loop {
		let statistics = strings.iter().fold([[0; 2]; 12], bit_count);

		let keep_bit = find_keep_bit(&statistics[idx], mode);

		partial.push(keep_bit);

		strings.retain(|string| string[0..=idx] == partial[0..=idx]);

		if strings.len() == 1 {
			break strings[0].iter().collect();
		}

		idx += 1;

		if idx >= 12 {
			unreachable!();
		}
	}
}

#[must_use]
pub fn part_two(strings: &Intermediate) -> Option<Output> {
	let oxygen_generator_rating_bits = find_component_rating(strings, Mode::KeepMostCommonOrOne);

	let oxygen_generator_rating: u32 = u32::from_str_radix(&oxygen_generator_rating_bits, 2).unwrap();

	let co2_scrubber_rating_bits = find_component_rating(strings, Mode::KeepLeastCommonOrZero);

	let co2_scrubber_rating: u32 = u32::from_str_radix(&co2_scrubber_rating_bits, 2).unwrap();

	Some(oxygen_generator_rating * co2_scrubber_rating)
}

daocutil::generate_solver!(solve, =>, self);
