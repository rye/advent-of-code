use crate::SolverClass;

mod day01;
mod day02;

pub(crate) const SOLVER_MAP: phf::Map<u8, SolverClass> = phf::phf_map! {
	1u8 => SolverClass::Original(day01::solve),
	2u8 => SolverClass::Original(day02::solve),
	// 3u8 => SolverClass::Original(day03::solve),
	// 4u8 => SolverClass::Original(day04::solve),
	// 5u8 => SolverClass::Original(day05::solve),
	// 6u8 => SolverClass::Original(day06::solve),
	// 7u8 => SolverClass::Original(day07::solve),
	// 8u8 => SolverClass::Original(day08::solve),
	// 9u8 => SolverClass::Original(day09::solve),
	// 10u8 => SolverClass::Original(day10::solve),
	// 11u8 => SolverClass::Original(day11::solve),
	// 12u8 => SolverClass::Original(day12::solve),
	// 13u8 => SolverClass::Original(day13::solve),
	// 14u8 => SolverClass::Original(day14::solve),
	// 15u8 => SolverClass::Original(day15::solve),
	// 16u8 => SolverClass::Original(day16::solve),
	// 17u8 => SolverClass::Original(day17::solve),
	// 18u8 => SolverClass::Original(day18::solve),
	// 19u8 => SolverClass::Original(day19::solve),
	// 20u8 => SolverClass::Original(day20::solve),
	// 21u8 => SolverClass::Original(day21::solve),
	// 22u8 => SolverClass::Original(day22::solve),
	// 23u8 => SolverClass::Original(day23::solve),
	// 24u8 => SolverClass::Original(day24::solve),
	// 25u8 => SolverClass::Original(day25::solve),
};
