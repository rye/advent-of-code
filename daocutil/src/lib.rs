use {
	core::num::ParseIntError,
	std::{error::Error, io},
};

pub type Solver = fn(&str) -> Result<(), Box<dyn Error>>;

/// Fully consumes a reader of type `std::io::Read` and produces a `String` containing all read text.
///
/// # Errors
///
/// An error is only returned if the underlying [`std::io::Read::read_to_string`] operation
/// returns an error.
///
/// See [`std::io::Read::read_to_string`] for all error semantics.
pub fn string_from(mut read: impl io::Read) -> io::Result<String> {
	let mut buf: String = String::new();
	read.read_to_string(&mut buf)?;
	Ok(buf)
}

fn parse_just_numbers(str: &str) -> Result<u8, ParseIntError> {
	str.matches(char::is_numeric).collect::<String>().parse()
}

/// Attempts to parse a given string into a `u8` representing the number of the day.
///
/// This function will first try to parse the string directly as a `u8`.
/// If parsing the string directly as a `u8` fails, then another attempt is made to parse the string by only parsing the numeric characters from it.
///
/// In either case, if the parsing succeeds, the resulting `u8` is returned.
/// If both attempts fail, `None` is returned.
pub fn parse_day_identifier(str: &str) -> Option<u8> {
	match (str.parse(), parse_just_numbers(str)) {
		(Ok(u8), _) | (Err(_), Ok(u8)) => Some(u8),
		(_, _) => None,
	}
}

#[cfg(test)]
mod parse_day_identifier {
	use super::parse_day_identifier;

	#[test]
	fn parse_bare() {
		let str = "23";
		assert_eq!(Some(23), parse_day_identifier(str));
	}
}

/// Generate the solver for a given day.
#[macro_export]
macro_rules! generate_solver {
	($fn_name:ident, =>, $place:path ) => {
		pub(crate) fn $fn_name(data: &str) -> Result<(), Box<dyn std::error::Error>> {
			use $place::{parse, part_one, part_two, Intermediate};

			let intermediate: Intermediate = parse(data)?;

			if let Some(part_one) = part_one(&intermediate) {
				println!("Part One: {}", part_one);
			}

			if let Some(part_two) = part_two(&intermediate) {
				println!("Part Two: {}", part_two);
			}

			Ok(())
		}
	};

	($fn_name:ident, -> , $inner:path) => {
		fn $fn_name(data: &str) -> Result<(), Box<dyn std::error::Error>> {
			$inner(data)
		}
	};
}

#[macro_export]
macro_rules! generate_solvers {
	($($id:literal $fn_name:ident | $tok:tt $expr:path ),* $(,)?) => {
		{
			$(
				daocutil::generate_solver!($fn_name, $tok, $expr);
			)*

			{
				let mut map: std::collections::HashMap<u8, daocutil::Solver> = std::collections::HashMap::new();

				$(
					map.insert($id, $fn_name);
				)*

				map
			}
		}
	};

	($place:path) => {
		{
			use $place as base;

			daocutil::generate_solvers![
				1_u8 day01 | => base::day01,
				2_u8 day02 | => base::day02,
				3_u8 day03 | => base::day03,
				4_u8 day04 | => base::day04,
				5_u8 day05 | => base::day05,
				6_u8 day06 | => base::day06,
				7_u8 day07 | => base::day07,
				8_u8 day08 | => base::day08,
				9_u8 day09 | => base::day09,
				10_u8 day10 | => base::day10,
				11_u8 day11 | => base::day11,
				12_u8 day12 | => base::day12,
				13_u8 day13 | => base::day13,
				14_u8 day14 | => base::day14,
				15_u8 day15 | => base::day15,
				16_u8 day16 | => base::day16,
				17_u8 day17 | => base::day17,
				18_u8 day18 | => base::day18,
				19_u8 day19 | => base::day19,
				20_u8 day20 | => base::day20,
				21_u8 day21 | => base::day21,
				22_u8 day22 | => base::day22,
				23_u8 day23 | => base::day23,
				24_u8 day24 | => base::day24,
				25_u8 day25 | => base::day25,
			]
		}
	};
}

#[macro_export]
macro_rules! generate_main {
	($loc:path) => {
		daocutil::generate_main!(daocutil::generate_solvers!($loc));
	};

	($solvers_expr:expr) => {
		fn main() -> Result<(), Box<dyn std::error::Error>> {
			let solvers: std::collections::HashMap<u8, daocutil::Solver> = { $solvers_expr };

			let mut args = std::env::args();

			let _ = args.next();

			if let Some(ident) = args.next() {
				if let Some(ident) = daocutil::parse_day_identifier(&ident) {
					if let Some(handler) = solvers.get(&ident) {
						let data: String = match (
							std::fs::File::open(format!("inputs/day{:02}", ident)),
							args.next(),
						) {
							(_, Some(filename)) => daocutil::string_from(std::fs::File::open(filename)?)?,
							(Ok(file), _) => daocutil::string_from(file)?,
							(_, None) => daocutil::string_from(std::io::stdin())?,
						};

						handler(&data)?;
					} else {
						println!("Day has no handler: {}", ident);
					}
				} else {
					println!("Unknown day identifier: {}", ident);
				}
			}

			Ok(())
		}
	};
}

#[macro_export]
macro_rules! test_example {
	($input:expr, $solver:ident, $parser:ident, $expected:expr) => {
		assert_eq!($solver(&$parser($input).expect("parse failed")), $expected);
	};

	($test_fn:ident, $parser:ident, $solver:ident, $input:expr, $expected:expr) => {
		#[test]
		fn $test_fn() {
			daocutil::test_example!($input, $solver, $parser, $expected);
		}
	};
}

#[macro_export]
macro_rules! generate_example_tests {
	($parser:ident, $solver:ident, $($test_fn:ident | $input:expr => $expected:expr),* $(,)?) => {
		$(
			daocutil::test_example!($test_fn, $parser, $solver, $input, $expected);
		)*
	};
}

#[macro_export]
macro_rules! test_examples {
	($parser:ident, $solver:ident, $($input:expr => $expected:expr),* $(,)?) => {
		$(
			daocutil::test_example!($input, $solver, $parser, $expected);
		)*
	};
}

mod neighbors;
pub use neighbors::*;

#[derive(PartialEq)]
pub enum SolverMode {
	PartOne,
	PartTwo,
}
