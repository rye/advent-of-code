use core::{any::Any, error::Error};
use std::collections::BTreeSet;

mod archive;
#[cfg(feature = "y2024")]
mod d2024;
#[cfg(feature = "y2025")]
mod d2025;

mod dtemplate;

mod util;

// Years -> Days -> Solvers

/// Types of solvers
#[non_exhaustive]
pub enum Solver {
	/// A solver that is a function that takes a string and prints its results.
	Original(fn(&str) -> Result<(), Box<dyn Error>>),
	/// A solver that holds a struct implementing `PartSolve`.
	PartSolve(Box<dyn PartSolve>),
}

/// Describes the behavior of common Advent of Code solvers
///
/// This trait arose out of the observation that Advent of Code challenges typically ask the player (that's you) to:
///
/// 1. Parse the input into some data structure.
/// 2. Use the data in that data structure to compute the answer for part one.
/// 3. Use the data in that data structure to compute the answer for part two.
///
/// This trait has several design aims:
///
/// - Parsing is fully separated from solving. The input can be deallocated before any solving begins.
///
/// - Part Solutions are fully independent from each other. They cannot mutate any shared state through the trait.
///
/// - Part Solutions can return `None` if they do not have an answer to provide (yet).
///
/// - Part Solutions return their answer as a heap-allocated `String`. This is flexible enough to handle days where
///   alphanumeric output is required to solve the puzzle (e.g. password challenges).
///
/// - In theory, one could run both parts in parallel, since they ought to be independent.
///
/// - The intermediate data structure is type-erased to `Box<dyn Any>`, allowing the implementer to choose any type
///   they wish to use as their "Intermediate".
pub trait PartSolve {
	/// Parse the provided `input` to an intermediate type.
	/// The resulting data will be passed in to `part_one` and `part_two`.
	///
	/// # Errors
	///
	/// If parsing fails for an unrecoverable reason, implementations can/should
	/// return an `Err` value.
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn Any>>;

	/// Solve the first part of the puzzle.
	///
	/// # Examples
	///
	/// Without accessing the intermediate data, you can simply return a value:
	///
	/// ```
	/// struct Solution;
	///
	/// impl aoc::PartSolve for Solution {
	///#    fn parse(&mut self, _input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
	///#        Ok(Box::new(42_u32))
	///#    }
	///#
	///     // parse, part_two omitted for brevity
	///
	///     fn part_one(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
	///         Some("42".to_string())
	///     }
	///#
	///#    fn part_two(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
	///#        Some("42".to_string())
	///#    }
	/// }
	/// ```
	///
	/// To access the intermediate data, use [`Any::downcast_ref`](core::any::Any::downcast_ref):
	///
	/// ```
	/// # use aoc::PartSolve;
	/// struct Solution;
	///
	/// impl aoc::PartSolve for Solution {
	///     // Split a comma-separated list of numbers...
	///     fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
	///         let numbers: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();
	///         Ok(Box::new(numbers))
	///     }
	///
	///     // ... sum them for part one ...
	///     fn part_one(&self, numbers: &dyn core::any::Any) -> Option<String> {
	///         let numbers: &Vec<u32> = numbers.downcast_ref()?;
	///         let sum: u32 = numbers.iter().sum();
	///         Some(sum.to_string())
	///     }
	///
	///     // ... and product them for part two.
	///     fn part_two(&self, numbers: &dyn core::any::Any) -> Option<String> {
	///         let numbers: &Vec<u32> = numbers.downcast_ref()?;
	///         let sum: u32 = numbers.iter().product();
	///         Some(sum.to_string())
	///     }
	/// }
	///
	/// let mut solver = Solution;
	/// let intermediate = solver.parse("1,2,3,4").unwrap();
	/// assert_eq!(Some("10".to_string()), solver.part_one(intermediate.as_ref()));
	/// assert_eq!(Some("24".to_string()), solver.part_two(intermediate.as_ref()));
	/// ```
	fn part_one(&self, intermediate: &dyn Any) -> Option<String>;

	/// Solve the second part of the puzzle.
	fn part_two(&self, intermediate: &dyn Any) -> Option<String>;
}

/// Execution constraints controlling which solvers will be selected.
#[derive(Debug, Clone)]
pub struct RunConstraints {
	years: Option<BTreeSet<u16>>,
	days: Option<BTreeSet<u8>>,
	/// If true and both year/day are None, run all solvers.
	run_all_if_unconstrained: bool,
}

impl RunConstraints {
	/// Determine constraints from command-line arguments.
	pub fn parse_from_args(args: std::env::Args) -> Self {
		#[derive(Debug)]
		enum SolverParam {
			#[allow(dead_code)]
			Unknown(String),
			Year(u16),
			Day(u8),
		}

		impl core::str::FromStr for SolverParam {
			type Err = core::convert::Infallible;

			fn from_str(str: &str) -> Result<Self, Self::Err> {
				match (str.parse::<u16>(), str.parse::<u8>()) {
					(_, Ok(u8)) if (1..=25).contains(&u8) => Ok(SolverParam::Day(u8)),
					(Ok(u16), _) if (2015..).contains(&u16) => Ok(SolverParam::Year(u16)),
					_ => Ok(SolverParam::Unknown(str.to_string())),
				}
			}
		}

		let mut years: Option<BTreeSet<u16>> = None;
		let mut days: Option<BTreeSet<u8>> = None;

		let run_all_if_unconstrained = false;

		let Some(inferred_constraint_parameters) = args
			.map(|arg| arg.parse::<SolverParam>())
			.collect::<Result<Vec<SolverParam>, _>>()
			.ok()
		else {
			return Self {
				years,
				days,
				run_all_if_unconstrained,
			};
		};

		for param in inferred_constraint_parameters {
			match param {
				SolverParam::Year(y) => {
					years.get_or_insert_with(BTreeSet::new).insert(y);
				}
				SolverParam::Day(d) => {
					days.get_or_insert_with(BTreeSet::new).insert(d);
				}
				SolverParam::Unknown(_) => {}
			}
		}

		Self {
			years,
			days,
			run_all_if_unconstrained,
		}
	}

	/// Returns true if a solver for a particular year and day is allowed under this set of constraints.
	fn allows(&self, year: u16, day: u8) -> bool {
		match (self.run_all_if_unconstrained, &self.years, &self.days) {
			// If no year/day specified, fall back to run_all_if_unconstrained.
			(true, None, None) => true,
			(false, None, None) => false,
			// If either year or day is specified, match against what was given (could be odd in the day case).
			(_, Some(req_year), None) => req_year.contains(&year),
			(_, None, Some(req_day)) => req_day.contains(&day),
			// If both year and day are specified, use both constraints (strictest).
			(_, Some(req_year), Some(req_day)) => req_year.contains(&year) && req_day.contains(&day),
		}
	}

	/// Conditionally offer a solver; if constraints allow, push onto output list.
	pub fn offer(
		&self,
		year: u16,
		day: u8,
		solver: Solver,
		out: &mut impl Extend<(u16, u8, Solver)>,
	) {
		if self.allows(year, day) {
			out.extend([(year, day, solver)]);
		}
	}
}

/// Gather all solvers matching the provided constraints.
#[must_use]
pub fn gather_matching_solvers(constraints: &RunConstraints) -> Vec<(u16, u8, Solver)> {
	let mut solvers: Vec<(u16, u8, Solver)> = Vec::new();

	#[cfg(feature = "y2015")]
	archive::d2015::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2016")]
	archive::d2016::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2017")]
	archive::d2017::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2018")]
	archive::d2018::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2019")]
	archive::d2019::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2020")]
	archive::d2020::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2021")]
	archive::d2021::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2022")]
	archive::d2022::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2023")]
	archive::d2023::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2024")]
	d2024::gather_matching_solvers(constraints, &mut solvers);
	#[cfg(feature = "y2025")]
	d2025::gather_matching_solvers(constraints, &mut solvers);

	// dtemplate::gather_matching_solvers(constraints, &mut v);

	solvers
}

#[macro_export]
macro_rules! export_solver {
	($fn_name:ident, $make_solver_expr:expr) => {
		pub(crate) fn $fn_name() -> $crate::Solver {
			$make_solver_expr
		}
	};
}

#[macro_export]
macro_rules! gen_gather_matching_solvers {
	($year:literal, $( $mod_ident:ident ( $day:literal ) => $make_day_solver:expr ),+ $(,)?) => {
		// Generate module declarations.
		$(
			mod $mod_ident;
		)+

		// Generate gatherer function for this year.
		pub(crate) fn gather_matching_solvers(
			constraints: &$crate::RunConstraints,
			out: &mut impl Extend<(u16, u8, $crate::Solver)>,
		) {
			let year = $year;
			$(
				constraints.offer(year, $day, $make_day_solver, out);
			)+
		}
	};
}

/// Generate a test function for a part function of a solver.
///
/// # Examples
///
/// There are several variations of this macro to help easily build out the tests as information becomes available.
///
/// ## No input, no expected output
///
/// ```
/// use aoc::{PartSolve, Solver, export_solver, part_test};
///
/// #[derive(Default)]
/// struct Solution;
///
/// impl PartSolve for Solution {
///     fn parse(&mut self, _input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
///         Ok(Box::new(()))
///     }
///
///     fn part_one(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
///         None
///     }
///
///     fn part_two(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
///         None
///     }
/// }
///
/// export_solver!(solver, Solver::PartSolve(Box::new(Solution)));
///
/// part_test!(part_one_no_input_no_output, Solution, None, part_one, None);
/// //         ^--+----------------------^  ^---+--^  ^--^  ^--+---^  ^--^
/// //            |                             |      ^^      |       ^^
/// //            |                             |  (no input)  |     (no expected output)
/// //            |                             |              |
/// //            |                             |              +-- fn to call on the solver after parsing completed
/// //            |                             |
/// //            |                             +-- Expr describing how to generate the "Solution" struct.
/// //            |
/// //            +-- Name of the test fn to create.
///
/// part_test!(part_two, Solution, None, part_two, None);
/// ```
#[macro_export]
macro_rules! part_test {
	($test_fn_name:ident, $make_solver_expr:expr, file $input_fname:literal, $part_fn_name:ident, None) => {
		$crate::part_test! {
			$test_fn_name,
			let mut solver: Solution = $make_solver_expr,
			let input: &str = include_str!($input_fname),
			let intermediate: Box<dyn std::any::Any> = solver.parse(input).unwrap(),
			let part_result = solver.$part_fn_name(intermediate.as_ref()),
			assert_eq!(None, part_result)
		}
	};

	($test_fn_name:ident, $make_solver_expr:expr, None, $part_fn_name:ident, None) => {
		$crate::part_test! {
			$test_fn_name,
			let mut solver: Solution = $make_solver_expr,
			let input: &str = "",
			let intermediate: Box<dyn core::any::Any> = solver.parse(input).unwrap(),
			let part_result = solver.$part_fn_name(intermediate.as_ref()),
			assert_eq!(None, part_result)
		}
	};

	($test_fn_name:ident, $make_solver_expr:expr, file $input_fname:literal, $part_fn_name:ident, literal $expected_output:literal) => {
		$crate::part_test! {
			$test_fn_name,
			let mut solver: Solution = $make_solver_expr,
			let input: &str = include_str!($input_fname),
			let intermediate: Box<dyn core::any::Any> = solver.parse(input).unwrap(),
			let part_result = solver.$part_fn_name(intermediate.as_ref()),
			assert_eq!(Some($expected_output.to_string()), part_result)
		}
	};

	($test_fn_name:ident, $make_solver_expr:expr, file $input_fname:literal, $part_fn_name:ident, file $expected_output_file:literal) => {
		$crate::part_test! {
			$test_fn_name,
			let mut solver: Solution = $make_solver_expr,
			let input: &str = include_str!($input_fname),
			let intermediate: Box<dyn core::any::Any> = solver.parse(input).unwrap(),
			let part_result = solver.$part_fn_name(intermediate.as_ref()),
			assert_eq!(Some(include_str!($expected_output_file).trim().to_string()), part_result)
		}
	};

	($test_fn_name:ident, $make_solver:stmt, $load_input:stmt, $parse_input:stmt, $call_solver_part_fn:stmt, $assert_expected_output:stmt) => {
		#[test]
		fn $test_fn_name() {
			$make_solver
			$load_input
			$parse_input
			$call_solver_part_fn
			$assert_expected_output
		}
	}
}
