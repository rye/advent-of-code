use core::error::Error;

use aoc::{RunConstraints, Solver, gather_matching_solvers};

fn main() -> Result<(), Box<dyn Error>> {
	let constraints = RunConstraints::parse_from_args(std::env::args());
	let solvers_to_run = gather_matching_solvers(&constraints);

	println!("Running {} solver(s).", solvers_to_run.len());

	for (year, day, solver) in solvers_to_run {
		println!("Running solver for year {year} day {day}:");

		let file_path = find_input_file(year, day);

		let data = match std::fs::read_to_string(&file_path) {
			Ok(data) => data,
			Err(error) => {
				println!("Error reading input file {file_path}: {error}. Skipped.");
				continue;
			}
		};

		match solver {
			// "Original"-class solvers simply take an &str, perform their operations, and print the output.
			Solver::Original(solver) => {
				solver(&data)?;
			}
			// "PartSolve"-class solvers have a defined data structure which groups together the parsing & solving logic.
			Solver::PartSolve(mut part_solver) => {
				let intermediate = part_solver.parse(&data)?;

				if let Some(result) = part_solver.part_one(&intermediate) {
					println!("Part One: {result}");
				}

				if let Some(result) = part_solver.part_two(&intermediate) {
					println!("Part Two: {result}");
				}
			}
			// Future solver types could be added. In case the handling hasn't been added yet, print out a line to stderr and skip running the year/day.
			_ => {
				eprintln!("Unsupported solver type for year {year} day {day}. Skipped.");
			}
		}
	}

	Ok(())
}

fn find_input_file(year: u16, day: u8) -> String {
	format!("inputs/{year}/day{day:02}.txt")
}
