use core::{error::Error, time::Duration};
use std::{path::PathBuf, time::Instant};

use aoc::{RunConstraints, Solver, gather_matching_solvers};

fn run_one_solver(data: &str, year: u16, day: u8, solver: Solver) -> Result<(), Box<dyn Error>> {
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
		// Future solver types could be added. In case the handling hasn't been added yet, print out a line to stderr and skip running the year/day.
		_ => {
			eprintln!("Unsupported solver type for year {year} day {day}. Skipped.");
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

		run_one_solver(&data, year, day, solver)?;
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
