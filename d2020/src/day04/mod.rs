use std::collections::HashMap;

pub fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
	matches!(
		(
			passport.get("byr"),
			passport.get("iyr"),
			passport.get("eyr"),
			passport.get("hgt"),
			passport.get("hcl"),
			passport.get("ecl"),
			passport.get("pid"),
			passport.get("cid"),
		),
		(
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			_
		)
	)
}

pub fn valid_birth_year(s: &str) -> bool {
	if let Ok(year) = s.parse::<usize>() {
		(1920..=2002).contains(&year)
	} else {
		false
	}
}

pub fn valid_issue_year(s: &str) -> bool {
	if let Ok(iyr) = s.parse::<usize>() {
		(2010..=2020).contains(&iyr)
	} else {
		false
	}
}

pub fn valid_expiry_year(s: &str) -> bool {
	if let Ok(eyr) = s.parse::<usize>() {
		(2020..=2030).contains(&eyr)
	} else {
		false
	}
}

pub fn valid_height(s: &str) -> bool {
	if let Some(first_letter_offset) = s.chars().position(|c| c.is_alphabetic()) {
		let maybe_number = &s[0..first_letter_offset];
		if let Ok(number) = maybe_number.parse::<usize>() {
			let rest = &s[first_letter_offset..];

			match rest {
				"cm" => (150..=193).contains(&number),
				"in" => (59..=76).contains(&number),
				_ => false,
			}
		} else {
			false
		}
	} else {
		false
	}
}

pub fn valid_hair_color(s: &str) -> bool {
	if let Some(rest) = s.strip_prefix('#') {
		let is_digit_and_lowercase = rest.chars().all(|ch| {
			if ch.is_digit(16) && !ch.is_digit(10) {
				ch.is_lowercase()
			} else {
				ch.is_digit(10)
			}
		});

		is_digit_and_lowercase
	} else {
		false
	}
}

pub type Intermediate<'a> = Vec<HashMap<&'a str, &'a str>>;
pub type Solution = usize;

pub fn parse(data: &str) -> Intermediate {
	// To start with, passports are separated by \n\n
	let passports: Vec<&str> = data.split("\n\n").collect();

	// Next, we split each passport on all whitespace to get a collection of key:value groups per passport
	let passports: Vec<Vec<&str>> = passports
		.iter()
		.map(|line| line.split_whitespace().collect())
		.collect();

	// Now, turn those collections of key:value groups into hashmaps
	let passports: Vec<HashMap<&str, &str>> = passports
		.iter()
		.map(|pairs| {
			pairs
				.iter()
				.map(|pair| {
					let results: Vec<&str> = pair.split(':').collect();
					(results[0], results[1])
				})
				.collect()
		})
		.collect();

	passports
}

pub fn part_one(passports: &Intermediate) -> Option<Solution> {
	Some(
		passports
			.iter()
			.filter(|passport| has_required_fields(passport))
			.count(),
	)
}

pub fn part_two(passports: &Intermediate) -> Option<Solution> {
	use std::collections::HashSet;

	let valid_eye_colors: HashSet<&str> = {
		let mut set = HashSet::new();
		set.insert("amb");
		set.insert("blu");
		set.insert("brn");
		set.insert("gry");
		set.insert("grn");
		set.insert("hzl");
		set.insert("oth");
		set
	};

	Some(
		passports
			.iter()
			.filter(|passport| {
				match (
					passport.get("byr"),
					passport.get("iyr"),
					passport.get("eyr"),
					passport.get("hgt"),
					passport.get("hcl"),
					passport.get("ecl"),
					passport.get("pid"),
					passport.get("cid"),
				) {
					(Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid), _) => {
						let byr_ok: bool = valid_birth_year(byr);
						let iyr_ok: bool = valid_issue_year(iyr);
						let eyr_ok: bool = valid_expiry_year(eyr);
						let hgt_ok: bool = valid_height(hgt);
						let hcl_ok: bool = valid_hair_color(hcl);
						let ecl_ok: bool = valid_eye_colors.contains(ecl);
						let pid_ok: bool = { pid.len() == 9 && pid.chars().all(|c| c.is_digit(10)) };

						byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
					}
					_ => false,
				}
			})
			.count(),
	)
}

#[cfg(test)]
mod tests;
