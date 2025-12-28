use core::str::FromStr;
use std::collections::BTreeSet;

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

struct IngredientDatabase {
	ranges: BTreeSet<(u64, u64)>,
	ingredients: Vec<u64>,
}

impl FromStr for IngredientDatabase {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// Two parts: \n\n-separated, first part is ranges (-separated), second part is list of numbers (ingredients)
		let sections: Vec<&str> = s.split("\n\n").collect();

		let range_lines = sections
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("Missing ranges section"))?
			.lines();

		let ranges = range_lines
			.filter_map(|line| {
				let (start, end) = line
					.split_once('-')
					.and_then(|(s, e)| Some((s.parse::<u64>().ok()?, e.parse::<u64>().ok()?)))?;
				Some((start, end))
			})
			.collect();

		let ingredients = sections
			.get(1)
			.ok_or_else(|| anyhow::anyhow!("Missing ingredients section"))?
			.lines()
			.map(|line| line.parse::<u64>())
			.collect::<Result<Vec<u64>, _>>()?;

		Ok(IngredientDatabase {
			ranges,
			ingredients,
		})
	}
}

impl IngredientDatabase {
	fn is_fresh_ingredient(&self, ingredient: u64) -> bool {
		for &(_lo, hi) in self.ranges.range(..=(ingredient, u64::MAX)).rev() {
			if hi >= ingredient {
				return true;
			}
		}

		false
	}

	fn count_fresh_listed_ingredients(&self) -> usize {
		self
			.ingredients
			.iter()
			.filter(|&&ingredient| self.is_fresh_ingredient(ingredient))
			.count()
	}

	fn combine_overlaps(&self) -> Vec<(u64, u64)> {
		// Copy the BTreeSet into a Vec so we can sort and mutate.
		let mut ranges = self.ranges.iter().cloned().collect::<Vec<(u64, u64)>>();

		// Sort by start so we can sweep from start to finish once, merging as we go.
		ranges.sort_by_key(|&(lo, _)| lo);

		let mut combined: Vec<(u64, u64)> = Vec::new();

		for (lo, hi) in ranges {
			// If there is an existing combined range, see whether the current one
			// overlaps or touches it (ranges are inclusive, so lo == last_hi + 1 counts).
			if let Some((_last_lo, last_hi)) = combined.last_mut() {
				if lo <= *last_hi + 1 {
					// Extend the existing range if the new one reaches further.
					*last_hi = (*last_hi).max(hi);
					continue;
				}
			}

			// Otherwise start a new disjoint range.
			combined.push((lo, hi));
		}

		combined
	}

	fn count_combined_overlap_ranges(&self) -> usize {
		let combined_ranges = self.combine_overlaps();
		combined_ranges
			.iter()
			.map(|&(range_lo, range_hi)| (range_lo..=range_hi).count())
			.sum()
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		Ok(Box::new(IngredientDatabase::from_str(input)?))
	}

	fn part_one(&self, ingredient_database: &dyn core::any::Any) -> Option<String> {
		let db = ingredient_database.downcast_ref::<IngredientDatabase>()?;
		Some(db.count_fresh_listed_ingredients().to_string())
	}

	fn part_two(&self, ingredient_database: &dyn core::any::Any) -> Option<String> {
		let db = ingredient_database.downcast_ref::<IngredientDatabase>()?;
		Some(db.count_combined_overlap_ranges().to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day05.example.in.txt", part_one, literal "3");
part_test!(part_two, Solution, file "day05.example.in.txt", part_two, literal "14");
