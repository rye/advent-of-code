#![allow(clippy::unnecessary_wraps, clippy::trivially_copy_pass_by_ref)]

use core::{any::Any, error::Error};

mod d2015;
mod d2016;
mod d2017;
mod d2018;
mod d2019;
mod d2020;
mod d2021;
mod d2022;
mod d2023;
mod d2024;

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

pub trait PartSolve {
	/// Parse the provided `input` to an intermediate type.
	/// The resulting data will be passed in to `part_one` and `part_two`.
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn Any>>;

	/// Solve the first part of the puzzle.
	fn part_one(&self, intermediate: &Box<dyn Any>) -> Option<String>;

	/// Solve the second part of the puzzle.
	fn part_two(&self, intermediate: &Box<dyn Any>) -> Option<String>;
}

/// Execution constraints controlling which solvers will be selected.
#[derive(Debug, Clone, Copy)]
pub struct RunConstraints {
	year: Option<u16>,
	day: Option<u8>,
	/// If true and both year/day are None, run all solvers.
	run_all_if_unconstrained: bool,
}

impl RunConstraints {
	pub fn new(year: Option<u16>, day: Option<u8>, run_all_if_unconstrained: bool) -> Self {
		Self {
			year,
			day,
			run_all_if_unconstrained,
		}
	}

	/// Returns true if a solver for a particular year and day is allowed under this set of constraints.
	fn allows(&self, year: u16, day: u8) -> bool {
		match (self.run_all_if_unconstrained, self.year, self.day) {
			(true, None, None) => true,
			(false, None, None) => false,
			(_, Some(req_year), None) => year == req_year,
			(_, None, Some(req_day)) => day == req_day,
			(_, Some(req_year), Some(req_day)) => year == req_year && day == req_day,
		}
	}

	/// Conditionally offer a solver; if constraints allow, push onto output list.
	pub fn offer(&self, year: u16, day: u8, solver: Solver, out: &mut Vec<(u16, u8, Solver)>) {
		if self.allows(year, day) {
			out.push((year, day, solver));
		}
	}

	/// Accessor for optional year (for external logic if ever needed).
	pub fn year(&self) -> Option<u16> {
		self.year
	}
	/// Accessor for optional day.
	pub fn day(&self) -> Option<u8> {
		self.day
	}
}

/// Gather all solvers matching the provided constraints.
pub fn gather_matching_solvers(constraints: &RunConstraints) -> Vec<(u16, u8, Solver)> {
	let mut solvers: Vec<(u16, u8, Solver)> = Vec::new();

	d2015::gather_matching_solvers(constraints, &mut solvers);
	d2016::gather_matching_solvers(constraints, &mut solvers);
	d2017::gather_matching_solvers(constraints, &mut solvers);
	d2018::gather_matching_solvers(constraints, &mut solvers);
	d2019::gather_matching_solvers(constraints, &mut solvers);
	d2020::gather_matching_solvers(constraints, &mut solvers);
	d2021::gather_matching_solvers(constraints, &mut solvers);
	d2022::gather_matching_solvers(constraints, &mut solvers);
	d2023::gather_matching_solvers(constraints, &mut solvers);
	d2024::gather_matching_solvers(constraints, &mut solvers);
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

		// Generate gatherer function
		pub(crate) fn gather_matching_solvers(
			constraints: &$crate::RunConstraints,
			out: &mut Vec<(u16, u8, $crate::Solver)>,
		) {
			let year = $year;
			$(
				constraints.offer(year, $day, $make_day_solver, out);
			)+
		}
	};
}
