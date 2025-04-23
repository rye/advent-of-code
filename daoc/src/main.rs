use core::error::Error;
use std::env::Args;

use daoc::SolverClass;

pub type Solver = fn(&str) -> Result<(), Box<dyn Error>>;

fn main() -> Result<(), Box<dyn Error>> {
	// Ignore the first argument (the program name)
	let (year_constraints, day_constraints) = parse_year_day(std::env::args());

	// Build the plan from the SOLVER_GLOBAL_MAP.
	// This involves crawling the SOLVER_GLOBAL_MAP and finding all solvers that match the year and day constriants.

	let solvers_to_run: std::collections::BTreeSet<(u16, u8, &SolverClass)> = daoc::SOLVER_GLOBAL_MAP
		.entries()
		.flat_map(|(year, days)| days.entries().map(|(day, solver)| (*year, *day, solver)))
		.filter(|(year, day, _)| {
			(year_constraints.is_none() || year_constraints == Some(*year))
				&& (day_constraints.is_none() || day_constraints == Some(*day))
		})
		.collect();

	println!("Running {} solver(s).", solvers_to_run.len());

	for (year, day, solver) in solvers_to_run {
		println!("Running solver for year {year} day {day}.");

		let file_path = find_input_file(year, day);

		let data = match std::fs::read_to_string(&file_path) {
			Ok(data) => data,
			Err(err) => {
				println!("Error reading input file {file_path}: {err}. Skipped.");
				continue;
			}
		};

		match solver {
			// "Original"-class solvers simply take an &str, perform their operations, and print the output.
			SolverClass::Original(solver) => {
				solver(&data)?;
			}
		}
	}

	Ok(())
}

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

fn parse_year_day(args: Args) -> (Option<u16>, Option<u8>) {
	let mut year = None;
	let mut day = None;

	let inferred_parameters: Vec<SolverParam> = args
		.map(|arg| arg.parse())
		.collect::<Result<_, _>>()
		.expect("parameter inference failed");

	for param in inferred_parameters {
		match param {
			SolverParam::Year(y) => year = Some(y),
			SolverParam::Day(d) => day = Some(d),
			SolverParam::Unknown(_) => {}
		}
	}

	(year, day)
}

fn find_input_file(year: u16, day: u8) -> String {
	format!("inputs/{year}/day{day:02}.txt")
}
