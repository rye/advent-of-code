use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

#[derive(Clone, PartialEq)]
struct PageNumber(u8);

struct OrderingRule {
	page: PageNumber,
	must_precede: PageNumber,
}

struct UpdatePageOrder(Vec<PageNumber>);

impl UpdatePageOrder {
	// If the rule is `A|B` (short for `A` must be printed before `B`), then only if:
	// - self.0 contains both `A` and `B`, AND
	// - self.0 lists `A` after `B`
	// do we have a violation of the rule.
	fn check_one_rule(&self, rule: &OrderingRule) -> Option<bool> {
		// If either page is missing, the rule is not violated.
		if !self.0.contains(&rule.page) || !self.0.contains(&rule.must_precede) {
			return None;
		}

		// If both pages are present, check their order
		Some(
			self.0.iter().position(|p| p == &rule.must_precede)
				> self.0.iter().position(|p| p == &rule.page),
		)
	}

	fn passes_rule(&self, rule: &OrderingRule) -> bool {
		self.check_one_rule(rule).unwrap_or(true)
	}

	fn violates_rule(&self, rule: &OrderingRule) -> bool {
		self.check_one_rule(rule) == Some(false)
	}

	fn update_yo_self(&self, rules: &[OrderingRule]) -> Self {
		let mut new_order = self.0.clone();

		while let Some(OrderingRule {
			page: a,
			must_precede: b,
		}) = rules
			.iter()
			.find(|rule| UpdatePageOrder(new_order.clone()).violates_rule(rule))
		{
			let pos_a = new_order.iter().position(|p| p == a).unwrap();
			let pos_b = new_order.iter().position(|p| p == b).unwrap();
			new_order.swap(pos_a, pos_b);
		}

		UpdatePageOrder(new_order)
	}
}

#[test]
fn check_one_rule_positive() {
	let update = UpdatePageOrder(vec![PageNumber(3), PageNumber(1), PageNumber(4)]);
	let rule = OrderingRule {
		page: PageNumber(1),
		must_precede: PageNumber(4),
	};

	assert_eq!(Some(true), update.check_one_rule(&rule));
}

#[test]
fn check_one_rule_negative() {
	let update = UpdatePageOrder(vec![PageNumber(3), PageNumber(1), PageNumber(4)]);
	let rule = OrderingRule {
		page: PageNumber(4),
		must_precede: PageNumber(1),
	};

	assert_eq!(Some(false), update.check_one_rule(&rule));
}

struct Input {
	ordering_rules: Vec<OrderingRule>,
	updates: Vec<UpdatePageOrder>,
}

impl PartSolve for Solution {
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let sections = input.split("\n\n").collect::<Vec<_>>();

		if sections.len() != 2 {
			return Err(anyhow::anyhow!("Expected exactly two sections"));
		}

		let rules_section = sections[0];
		let updates_section = sections[1];

		let ordering_rules = rules_section
			.lines()
			.filter_map(|line| {
				let mut parts = line.split('|');
				let left = parts.next();
				let right = parts.next();

				let Ok(left) = left?.trim().parse() else {
					panic!("Failed to parse left page number in rule: {line}");
				};

				let Ok(right) = right?.trim().parse() else {
					panic!("Failed to parse right page number in rule: {line}");
				};

				Some(OrderingRule {
					page: PageNumber(left),
					must_precede: PageNumber(right),
				})
			})
			.collect();

		let updates: Vec<UpdatePageOrder> = updates_section
			.lines()
			.map(|line| {
				let pages: Result<Vec<PageNumber>, _> = line
					.split(',')
					.map(|part| part.trim().parse().map(PageNumber))
					.collect();

				let Ok(pages) = pages else {
					panic!("Failed to parse page numbers in update line: {line}");
				};

				UpdatePageOrder(pages)
			})
			.collect();

		Ok(Box::new(Input {
			ordering_rules,
			updates,
		}))
	}

	fn part_one(&self, input: &Box<dyn core::any::Any>) -> Option<String> {
		let Input {
			ordering_rules,
			updates,
		} = input.downcast_ref::<Input>()?;

		let sum = updates
			.iter()
			.filter(|update| ordering_rules.iter().all(|rule| update.passes_rule(rule)))
			.map(|update| update.0[update.0.len() / 2].clone())
			.map(|page_number| u32::from(page_number.0))
			.sum::<u32>();

		Some(sum.to_string())
	}

	fn part_two(&self, input: &Box<dyn core::any::Any>) -> Option<String> {
		let Input {
			ordering_rules,
			updates,
		} = input.downcast_ref::<Input>()?;

		let sum = updates
			.iter()
			.filter(|update| ordering_rules.iter().any(|rule| update.violates_rule(rule)))
			.map(|update| update.update_yo_self(ordering_rules))
			.map(|update| update.0[update.0.len() / 2].clone())
			.map(|page_number| u32::from(page_number.0))
			.sum::<u32>();

		Some(sum.to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution::default(), file "day05.example.in.txt", part_one, literal "143");

part_test!(part_two, Solution::default(), file "day05.example.in.txt", part_two, literal "123");
