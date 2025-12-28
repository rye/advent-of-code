use core::{error::Error, time::Duration};
use std::{collections::BTreeSet, path::PathBuf, time::Instant};

mod archive;
#[cfg(feature = "y2024")]
mod d2024;
#[cfg(feature = "y2025")]
mod d2025;
mod dtemplate;

mod solver;
pub(crate) use solver::PartSolve;
mod util;

// Years -> Days -> Solvers

/// Types of solvers
pub enum Solver {
	/// A solver that is a function that takes a string and prints its results.
	Original(fn(&str) -> Result<(), Box<dyn Error>>),
	/// A solver that holds a struct implementing `PartSolve`.
	PartSolve(Box<dyn PartSolve>),
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
			let intermediate: Box<dyn core::any::Any> = solver.parse(input).unwrap(),
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

fn run_one_solver(data: &str, solver: Solver) -> Result<(), Box<dyn Error>> {
	match solver {
		// "Original"-class solvers simply take an &str, perform their operations, and print the output.
		Solver::Original(solver) => {
			solver(data)?;
		}
		// "PartSolve"-class solvers have a defined data structure which groups together the parsing & solving logic.
		Solver::PartSolve(mut part_solver) => {
			let t_start = Instant::now();

			let intermediate = part_solver.parse(data)?;

			let t_parsed = Instant::now();

			let mut t_solved_1: Option<Instant> = None;
			let mut t_solved_2: Option<Instant> = None;

			let t_solving_1 = Instant::now();
			if let Some(result) = part_solver.part_one(intermediate.as_ref()) {
				t_solved_1 = Some(Instant::now());
				println!("Part One: {result}");
			}

			let t_solving_2 = Instant::now();
			if let Some(result) = part_solver.part_two(intermediate.as_ref()) {
				t_solved_2 = Some(Instant::now());
				println!("Part Two: {result}");
			}

			if true {
				let parse_dur = t_parsed.duration_since(t_start);
				let part_one_dur: Option<Duration> = t_solved_1.map(|t| t.duration_since(t_solving_1));
				let part_two_dur: Option<Duration> = t_solved_2.map(|t| t.duration_since(t_solving_2));

				println!(
					"  (timings: {{parse: {parse_dur:.1?}, part_one: {}, part_two: {}}})",
					part_one_dur.map_or_else(|| "n/a".to_string(), |dur| format!("{dur:.1?}")),
					part_two_dur.map_or_else(|| "n/a".to_string(), |dur| format!("{dur:.1?}")),
				);
			}
		}
	}

	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let constraints = RunConstraints::parse_from_args(std::env::args());

	let solvers_to_run = gather_matching_solvers(&constraints);

	println!("Running {} solver(s).", solvers_to_run.len());

	for (year, day, solver) in solvers_to_run {
		println!("Running solver for year {year} day {day}:");

		let input_file_path = match find_input_file(year, day) {
			Ok(path) => path,
			Err(error) => {
				eprintln!("Error determining input file path for year {year} day {day}: {error}. Skipped.");
				continue;
			}
		};

		let data = match std::fs::read_to_string(&input_file_path) {
			Ok(data) => data,
			Err(error) => {
				eprintln!(
					"Error reading input file {}: {error}. Skipped.",
					input_file_path.display()
				);
				continue;
			}
		};

		run_one_solver(&data, solver)?;
	}

	Ok(())
}

#[derive(Debug, thiserror::Error)]
enum InputFindError {
	#[error("I/O error occurred while finding input file: {0}")]
	Io(#[from] std::io::Error),
	#[error("missing \"inputs\" directory")]
	MissingInputDir { cwd: PathBuf },
	#[error("missing input file (tried: {tried_paths:?})")]
	MissingInputFile { tried_paths: Vec<PathBuf> },
}

fn find_input_file(year: u16, day: u8) -> Result<PathBuf, InputFindError> {
	// Use relative paths directly without getting current directory
	let inputs_root = PathBuf::from("inputs");
	if !inputs_root.is_dir() {
		return Err(InputFindError::MissingInputDir {
			cwd: std::env::current_dir()?,
		});
	}

	// If the inputs root exists, we expect to find inputs from it under one of two subtrees:
	//
	// - inputs/archive/{year}/day{day:02}.txt (the "archive")
	// - inputs/{year}/day{day:02}.txt (the "current" inputs)
	let archive_dir = inputs_root.join("archive");

	let paths_to_try = [archive_dir, inputs_root]
		.into_iter()
		.filter(|d| d.is_dir())
		.map(|dir| dir.join(format!("{year}/day{day:02}.txt")));

	let mut tried_paths = Vec::new();

	for path in paths_to_try {
		tried_paths.push(path.clone());

		if path.is_file() {
			return Ok(path);
		}
	}

	Err(InputFindError::MissingInputFile { tried_paths })
}
