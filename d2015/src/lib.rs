pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
// day 08 is implemented in Python; see src/day08.py
pub mod day09;

/// Fully consumes a reader of type `std::io::Read` and produces a `String` containing all read text.
///
/// # Errors
///
/// An error is only returned if the underlying [`std::io::Read::read_to_string`] operation returns an error.
/// See [`std::io::Read::read_to_string`] for all error semantics.
pub fn string_from(mut read: impl std::io::Read) -> std::io::Result<String> {
	let mut buf: String = String::new();
	read.read_to_string(&mut buf)?;
	Ok(buf)
}

#[macro_export]
macro_rules! day_solver {
	( $transform:expr, $part_one:expr, $part_two:expr ) => {
		fn main() -> Result<(), Box<dyn std::error::Error>> {
			use ::std::io::stdin;
			use $crate::string_from;

			let data: String = string_from(stdin())?;

			let intermediate = $transform(&data);

			if let Some(part_one) = $part_one(&intermediate) {
				println!("Part One: {}", part_one);
			}

			if let Some(part_two) = $part_two(&intermediate) {
				println!("Part Two: {}", part_two);
			}

			Ok(())
		}
	};
}

#[macro_export]
macro_rules! day_solver_std {
	() => {
		d2015::day_solver_from!(self);
	};
}

#[macro_export]
macro_rules! day_solver_from {
	($place:path) => {
		use $place::{parse, part_one, part_two};

		d2015::day_solver!(
			|data| { parse(data) },
			|intermediate| { part_one(intermediate) },
			|intermediate| { part_two(intermediate) }
		);
	};

	($place:path, $parser:ident) => {
		use $place::{part_one, part_two, $parser};

		d2015::day_solver!(
			|data| { $parser(data) },
			|intermediate| { part_one(intermediate) },
			|intermediate| { part_two(intermediate) }
		);
	};

	($place:path, $parser:ident, $part_one:ident, $part_two:ident) => {
		use $place::{$parser, $part_one, $part_two};

		d2015::day_solver!(
			|data| { $parser(data) },
			|intermediate| { $part_one(intermediate) },
			|intermediate| { $part_two(intermediate) }
		);
	};
}
