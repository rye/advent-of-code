pub mod neighbors;

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum SolverMode {
	PartOne,
	PartTwo,
}

/// Generate the solver for a given day.
#[macro_export]
macro_rules! generate_solver {
	($fn_name:ident, =>, $place:path ) => {
		pub(crate) fn $fn_name(data: &str) -> Result<(), Box<dyn core::error::Error>> {
			use $place::{Intermediate, parse, part_one, part_two};

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
}

#[macro_export]
macro_rules! test_example {
	($input:expr, $solver:ident, $parser:ident, $expected:expr) => {
		assert_eq!($solver(&$parser($input).expect("parse failed")), $expected);
	};

	($test_fn:ident, $parser:ident, $solver:ident, $input:expr, $expected:expr) => {
		#[test]
		fn $test_fn() {
			$crate::test_example!($input, $solver, $parser, $expected);
		}
	};
}

#[macro_export]
macro_rules! generate_example_tests {
	($parser:ident, $solver:ident, $($test_fn:ident | $input:expr => $expected:expr),* $(,)?) => {
		$(
			$crate::test_example!($test_fn, $parser, $solver, $input, $expected);
		)*
	};
}

#[macro_export]
macro_rules! test_examples {
	($parser:ident, $solver:ident, $($input:expr => $expected:expr),* $(,)?) => {
		$(
			$crate::test_example!($input, $solver, $parser, $expected);
		)*
	};
}
