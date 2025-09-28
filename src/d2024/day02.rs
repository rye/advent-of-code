use itertools::Itertools;

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution {
	reports: Option<Vec<Vec<i32>>>,
}

impl Solution {
	fn report_is_safe_as_is(report: &[i32]) -> bool {
		let level_jumps_within_bounds = report
			.iter()
			.tuple_windows()
			.all(|(a, b)| (1..=3).contains(&b.abs_diff(*a)));

		let all_increasing_or_decreasing = report.iter().is_sorted() || report.iter().rev().is_sorted();

		level_jumps_within_bounds && all_increasing_or_decreasing
	}

	fn report_is_safe_after_removing_at_most_one_element(report: &[i32]) -> bool {
		Self::report_is_safe_as_is(report) || {
			for attempt_idx in 0..report.len() {
				let left = &report[..attempt_idx];
				let right = &report[attempt_idx + 1..];
				if Self::report_is_safe_as_is(&[left, right].concat()) {
					return true;
				}
			}
			false
		}
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, reports: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		self.reports = Some(
			reports
				.lines()
				.map(|line| {
					line
						.split_whitespace()
						.map(|num| num.parse::<i32>().unwrap())
						.collect()
				})
				.collect(),
		);

		Ok(Box::new(()))
	}

	fn part_one(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		let Some(reports) = &self.reports else {
			return None;
		};

		Some(
			reports
				.iter()
				.filter(|report| Solution::report_is_safe_as_is(report))
				.count()
				.to_string(),
		)
	}

	fn part_two(&self, _intermediate: &Box<dyn core::any::Any>) -> Option<String> {
		let Some(reports) = &self.reports else {
			return None;
		};

		Some(
			reports
				.iter()
				.filter(|report| Solution::report_is_safe_after_removing_at_most_one_element(report))
				.count()
				.to_string(),
		)
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution::default())));

part_test!(part_one, Solution::default(), file "day02.example.in.txt", part_one, literal "2");
part_test!(part_two, Solution::default(), file "day02.example.in.txt", part_two, literal "4");
