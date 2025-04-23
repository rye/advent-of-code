use std::collections::BTreeSet;

pub type Answer = char;

pub fn people_in_group(group: &str) -> impl Iterator<Item = &str> {
	group.split_whitespace()
}

pub fn answers(person: &str) -> impl Iterator<Item = Answer> + '_ {
	person.chars().filter(|c| c.is_alphabetic())
}

pub fn intersect_all(items: impl Iterator<Item = BTreeSet<Answer>>) -> Option<BTreeSet<Answer>> {
	items.fold(
		None,
		|state: Option<BTreeSet<Answer>>, answers: BTreeSet<Answer>| {
			if let Some(state) = state {
				Some(state.intersection(&answers).copied().collect())
			} else {
				Some(answers)
			}
		},
	)
}

pub type Intermediate<'a> = Vec<&'a str>;
pub type Output = usize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(data.split("\n\n").collect())
}

pub fn part_one(groups: &Intermediate) -> Option<Output> {
	Some(
		groups
			.iter()
			.map(|group| answers(group).collect())
			.map(|answers: BTreeSet<Answer>| answers.len())
			.sum(),
	)
}

pub fn part_two(groups: &Intermediate) -> Option<Output> {
	Some(
		groups
			.iter()
			.map(|group| {
				let answers_by_person = people_in_group(group).map(|person| answers(person).collect());
				let answers_by_all = intersect_all(answers_by_person);

				answers_by_all.expect("no people in group").len()
			})
			.sum(),
	)
}

#[cfg(test)]
mod tests;

daocutil::generate_solver!(solve, =>, self);
