pub type Intermediate = (usize, usize);
type Output = usize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	let lines = data
		.lines()
		.map(|s: &str| s.parse::<usize>().unwrap())
		.collect::<Vec<_>>();

	Ok((lines[1], lines[0]))
}

pub fn part_one(&(card_pubkey, door_pubkey): &Intermediate) -> Option<Output> {
	Some(find_encryption_key(card_pubkey, door_pubkey))
}

pub fn part_two(_: &Intermediate) -> Option<Output> {
	None
}

fn transform_sub(subject: usize, loop_size: usize) -> usize {
	let mut value: usize = 1_usize;

	for _ in 0..loop_size {
		value *= subject;
		value %= 20_201_227_usize;
	}

	value
}

fn find_loop_size_for_key(pubkey: usize, subject: usize) -> usize {
	let mut value = 1_usize;
	let mut loop_size = 0;

	while value != pubkey {
		value = value * subject % 20_201_227;
		loop_size += 1;
	}

	loop_size
}

#[test]
fn transform_sub_s7_l8() {
	let subject = 7_usize;
	let loop_size: usize = 8_usize;

	assert_eq!(transform_sub(subject, loop_size), 5_764_801_usize);
}

#[test]
fn transform_sub_s7_l11() {
	let subject = 7_usize;
	let loop_size: usize = 11_usize;

	assert_eq!(transform_sub(subject, loop_size), 17_807_724_usize);
}

#[cfg(test)]
fn find_loop_size(target: usize, subject: usize) -> usize {
	let mut loop_size = 0_usize;

	loop {
		if transform_sub(subject, loop_size) == target {
			break loop_size;
		}

		loop_size += 1;
	}
}

fn find_subject(target: usize, loop_size: usize) -> usize {
	let mut subject = 1_usize;

	loop {
		if transform_sub(subject, loop_size) == target {
			break subject;
		}

		subject += 1_usize;
	}
}

#[test]
fn find_subject_t5764801_l8() {
	let target = 5_764_801_usize;
	let loop_size: usize = 8_usize;

	assert_eq!(find_subject(target, loop_size), 7_usize);
}

#[test]
fn find_subject_t17807724_l8() {
	let target = 17_807_724_usize;
	let loop_size: usize = 11_usize;

	assert_eq!(find_subject(target, loop_size), 7_usize);
}

#[test]
fn find_loop_size_s7_u5764801() {
	let target = 5_764_801_usize;
	let subject = 7_usize;

	assert_eq!(find_loop_size(target, subject), 8_usize);
}

#[test]
fn find_loop_size_s7_u17807724() {
	let target = 17_807_724_usize;
	let subject = 7_usize;

	assert_eq!(find_loop_size(target, subject), 11_usize);
}

fn find_encryption_key(pkey_card: usize, pkey_door: usize) -> usize {
	let shared_subject = 7_usize;

	let card_loop_size = find_loop_size_for_key(pkey_card, shared_subject);
	let door_loop_size = find_loop_size_for_key(pkey_door, shared_subject);

	let card_self_subject = find_subject(pkey_card, card_loop_size);
	assert_eq!(card_self_subject, 7_usize);

	let door_self_subject = find_subject(pkey_door, door_loop_size);
	assert_eq!(door_self_subject, 7_usize);

	let card_to_door_key = transform_sub(pkey_door, card_loop_size);
	let door_to_card_key = transform_sub(pkey_card, door_loop_size);

	assert_eq!(card_to_door_key, door_to_card_key);

	card_to_door_key
}

#[test]
fn find_encryption_key_d17807724_c5764801() {
	let card_pubkey = 5_764_801_usize;
	let door_pubkey = 17_807_724_usize;

	assert_eq!(
		find_encryption_key(card_pubkey, door_pubkey),
		14_897_079_usize
	);
}

crate::generate_solver!(solve, =>, self);
