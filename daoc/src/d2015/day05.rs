pub type Intermediate<'input> = Vec<&'input str>;
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.lines().collect())
}

#[must_use]
pub fn part_one(strings: &Intermediate) -> Option<Output> {
	Some(strings.iter().filter(is_nice_old).count())
}

#[must_use]
pub fn part_two(strings: &Intermediate) -> Option<Output> {
	Some(strings.iter().filter(is_nice_new).count())
}

fn contains_at_least_three_vowels(string: &str) -> bool {
	string
		.chars()
		.filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
		.count()
		>= 3
}

fn has_at_least_one_duplicated_letter(string: &str) -> bool {
	let chars: Vec<char> = string.chars().collect();
	for window in chars.windows(2) {
		if window.first() == window.get(1) {
			return true;
		}
	}
	false
}

fn does_not_contain_banned_strings(string: &str) -> bool {
	const STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

	for banned_str in STRINGS {
		if string.contains(banned_str) {
			return false;
		}
	}

	true
}

fn is_nice_old(string: &impl AsRef<str>) -> bool {
	let string: &str = string.as_ref();

	contains_at_least_three_vowels(string)
		&& has_at_least_one_duplicated_letter(string)
		&& does_not_contain_banned_strings(string)
}

fn contains_nonoverlapping_pair(string: &str) -> bool {
	let chars: Vec<char> = string.chars().collect();
	for window in chars.windows(2) {
		let pat = format!("{}{}", window[0], window[1]);
		if string.split(&pat).count() >= 3 {
			return true;
		}
	}

	false
}

#[test]
fn aaa_cnp() {
	assert!(!contains_nonoverlapping_pair("aaa"));
}

#[test]
fn aaaa_cnp() {
	assert!(contains_nonoverlapping_pair("aaaa"));
}

#[test]
fn xy0xy_cnp() {
	assert!(contains_nonoverlapping_pair("xy0xy"));
}

fn contains_one_wrapped_letter(string: &str) -> bool {
	let chars: Vec<char> = string.chars().collect();

	for window in chars.windows(3) {
		if window.first() == window.get(2) {
			return true;
		}
	}

	false
}

fn is_nice_new(string: &impl AsRef<str>) -> bool {
	let string: &str = string.as_ref();

	contains_nonoverlapping_pair(string) && contains_one_wrapped_letter(string)
}

#[cfg(test)]
mod is_nice_new {
	use super::is_nice_new;

	#[test]
	fn qjhvhtzxzqqjkmpb_is_nice() {
		assert!(is_nice_new(&"qjhvhtzxzqqjkmpb"));
	}

	#[test]
	fn xxyxx_is_nice() {
		assert!(is_nice_new(&"xxyxx"));
	}

	#[test]
	fn uurcxstgmygtbstg_is_naughty() {
		assert!(!is_nice_new(&"uurcxstgmygtbstg"));
	}

	#[test]
	fn ieodomkazucvgmuy_is_nice() {
		assert!(!is_nice_new(&"ieodomkazucvgmuy"));
	}
}

crate::generate_solver!(solve, =>, self);
