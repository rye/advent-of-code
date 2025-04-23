#![allow(
	dead_code,
	clippy::trivially_copy_pass_by_ref,
	clippy::unnecessary_wraps
)]

use core::error::Error;

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

// Years -> Days -> Solvers

/// Types of solvers
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SolverClass {
	/// A solver that is a function that takes a string and prints its results.
	Original(fn(&str) -> Result<(), Box<dyn Error>>),
}

// pub type Solver = fn(&str) -> Result<(), Box<dyn Error>>;

pub const SOLVER_GLOBAL_MAP: phf::Map<u16, phf::Map<u8, SolverClass>> = phf::phf_map! {
	2015u16 => d2015::SOLVER_MAP,
	2016u16 => d2016::SOLVER_MAP,
	2017u16 => d2017::SOLVER_MAP,
	2018u16 => d2018::SOLVER_MAP,
	2019u16 => d2019::SOLVER_MAP,
	2020u16 => d2020::SOLVER_MAP,
	2021u16 => d2021::SOLVER_MAP,
	2022u16 => d2022::SOLVER_MAP,
	2023u16 => d2023::SOLVER_MAP,
	2024u16 => d2024::SOLVER_MAP,
	// 2025u16 => d2025::SOLVER_MAP,
};
