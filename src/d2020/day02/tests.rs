use super::{validate_password, Rule};

#[test]
fn validate_password_correct_0() {
	let rule: Rule = Rule {
		count_range: (1, 3),
		character: 'a',
	};

	let password = "abcde";

	assert!(validate_password(&rule, password));
}

#[test]
fn validate_password_correct_1() {
	let rule: Rule = Rule {
		count_range: (1, 3),
		character: 'b',
	};

	let password = "cdefg";

	assert!(!validate_password(&rule, password));
}

#[test]
fn validate_password_correct_2() {
	let rule: Rule = Rule {
		count_range: (2, 9),
		character: 'c',
	};

	let password = "ccccccccc";

	assert!(validate_password(&rule, password));
}
