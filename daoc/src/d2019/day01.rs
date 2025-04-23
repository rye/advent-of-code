pub type Intermediate = Vec<u32>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let module_weights: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

	Ok(module_weights)
}

#[must_use]
pub fn part_one(module_weights: &Intermediate) -> Option<Output> {
	pub fn fuel_required_for_module(mass: u32) -> u32 {
		(mass / 3) - 2
	}

	let mut fuel_weight_sum = 0;

	for module_weight in module_weights {
		fuel_weight_sum += fuel_required_for_module(*module_weight);
	}

	Some(fuel_weight_sum)
}

crate::generate_example_tests!(
	parse, part_one,
	fuel_part_one_12 | "12" => Some(2),
	fuel_part_one_14 | "14" => Some(2),
	fuel_part_one_1969 | "1969" => Some(654),
	fuel_part_one_100756 | "100756" => Some(33583),
);

#[must_use]
pub fn part_two(module_weights: &Intermediate) -> Option<Output> {
	pub fn fuel_required_for_module(mass: u32) -> u32 {
		// Compute the required fuel (if any) for a given mass.
		let fuel_for_mass = |mass: u32| (mass / 3).checked_sub(2);

		// A running total.
		let mut total: u32 = 0;

		// Copy and mutify the mass variable.
		let mut mass = mass;

		// If we need some more fuel for the remaining unaccounted mass...
		while let Some(part) = fuel_for_mass(mass) {
			// ... add the required fuel to the total ...
			total += part;
			// ... and now our mass calculation should happen again for the fuel we just added.
			mass = part;
		}

		total
	}

	let mut fuel_weight_sum = 0;

	for module_weight in module_weights {
		fuel_weight_sum += fuel_required_for_module(*module_weight);
	}

	Some(fuel_weight_sum)
}

crate::generate_example_tests!(
	parse, part_two,
	fuel_part_two_14 | "14" => Some(2),
	fuel_part_two_1969 | "1969" => Some(966),
	fuel_part_two_100756 | "100756" => Some(50346)
);

crate::generate_solver!(solve, =>, self);
