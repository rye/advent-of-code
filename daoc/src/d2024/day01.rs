pub type Intermediate = ();
pub type Output = u32;

/// Parses the input data and returns an `Intermediate` type.
///
/// # Errors
pub fn parse(_data: &str) -> anyhow::Result<Intermediate> {
	Ok(())
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

/// Processes the parsed intermediate data and determines the solution for part one.
#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}

daocutil::generate_solver!(solve, =>, self);
