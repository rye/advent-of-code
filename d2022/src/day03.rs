use {core::ops::Deref, std::collections::HashSet};

pub struct Rucksack<'a>(&'a str);

impl<'a> From<&'a str> for Rucksack<'a> {
	fn from(value: &'a str) -> Self {
		Self(value)
	}
}

impl<'a> Deref for Rucksack<'a> {
	type Target = &'a str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Rucksack<'_> {
	fn char_priority(char: char) -> Option<u32> {
		u8::try_from(char).ok().and_then(|byte| match byte {
			b'a'..=b'z' => Some(u32::from((byte - b'a') + 1)),
			b'A'..=b'Z' => Some(u32::from((byte - b'A') + 26 + 1)),
			_ => None,
		})
	}

	fn str_priorities(str: &str) -> HashSet<u32> {
		str.chars().filter_map(Self::char_priority).collect()
	}

	fn compartment_priorities(&self) -> (HashSet<u32>, HashSet<u32>) {
		let compartments = self.split_at(self.len() / 2);

		(
			Self::str_priorities(compartments.0),
			Self::str_priorities(compartments.1),
		)
	}

	fn priorities(&self) -> HashSet<u32> {
		Self::str_priorities(self.0)
	}
}

pub type Intermediate<'a> = Vec<Rucksack<'a>>;
pub type Output = u32;

/// # Errors
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	Ok(str.lines().map(Rucksack::from).collect())
}

// fn item_priority(item: char) -> Option<u32> {}

#[must_use]
pub fn part_one(rucksacks: &Intermediate) -> Option<Output> {
	let mut sum = 0_u32;

	for rucksack in rucksacks {
		let (priorities_a, priorities_b) = rucksack.compartment_priorities();
		sum += priorities_a.intersection(&priorities_b).sum::<u32>();
	}

	Some(sum)
}

#[must_use]
pub fn part_two(rucksacks: &Intermediate) -> Option<Output> {
	let mut sum = 0_u32;

	for sack_group in rucksacks.chunks_exact(3) {
		let common_priorities: HashSet<u32> =
			sack_group
				.iter()
				.fold(HashSet::new(), |shared_priorities, sack| {
					if shared_priorities.is_empty() {
						&shared_priorities | &sack.priorities()
					} else {
						&shared_priorities & &sack.priorities()
					}
				});

		sum += common_priorities.iter().sum::<u32>();
	}

	Some(sum)
}

#[cfg(test)]
mod rucksack {
	use super::Rucksack;

	#[test]
	fn char_priority_a_lc() {
		assert_eq!(Rucksack::char_priority('a'), Some(1));
	}

	#[test]
	fn char_priority_z_lc() {
		assert_eq!(Rucksack::char_priority('z'), Some(26));
	}

	#[test]
	fn char_priority_a_uc() {
		assert_eq!(Rucksack::char_priority('A'), Some(27));
	}

	#[test]
	fn char_priority_z_uc() {
		assert_eq!(Rucksack::char_priority('Z'), Some(52));
	}
}
