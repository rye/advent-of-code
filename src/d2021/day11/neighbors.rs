pub fn neighbors<const N: usize>(y_0: u8, x_0: u8) -> impl Iterator<Item = (u8, u8)> {
	assert!(u8::try_from(N).is_ok());

	let upper_bound = u8::try_from(N).unwrap();

	let y_min = y_0.saturating_sub(1);
	let y_max = y_0.saturating_add(1).clamp(0, upper_bound - 1);

	let x_min = x_0.saturating_sub(1);
	let x_max = x_0.saturating_add(1).clamp(0, upper_bound - 1);

	(y_min..=y_max)
		.flat_map(move |y| (x_min..=x_max).map(move |x| (y, x)))
		.filter(move |(y, x)| y != &y_0 || x != &x_0)
}
#[cfg(test)]
mod _5 {
	use super::neighbors;

	#[test]
	fn corners() {
		let mut neighbors = neighbors::<5>(0, 0);

		assert_eq!(neighbors.next(), Some((0, 1)));
		assert_eq!(neighbors.next(), Some((1, 0)));
		assert_eq!(neighbors.next(), Some((1, 1)));
		assert_eq!(neighbors.next(), None);
	}

	#[test]
	fn inner_center() {
		let mut neighbors = neighbors::<5>(2, 2);

		assert_eq!(neighbors.next(), Some((1, 1)));
		assert_eq!(neighbors.next(), Some((1, 2)));
		assert_eq!(neighbors.next(), Some((1, 3)));
		assert_eq!(neighbors.next(), Some((2, 1)));
		// assert_eq!(neighbors.next(), Some((2, 2)));
		assert_eq!(neighbors.next(), Some((2, 3)));
		assert_eq!(neighbors.next(), Some((3, 1)));
		assert_eq!(neighbors.next(), Some((3, 2)));
		assert_eq!(neighbors.next(), Some((3, 3)));
		assert_eq!(neighbors.next(), None);
	}

	#[test]
	fn inner_off_center() {
		let mut neighbors = neighbors::<5>(3, 3);

		assert_eq!(neighbors.next(), Some((2, 2)));
		assert_eq!(neighbors.next(), Some((2, 3)));
		assert_eq!(neighbors.next(), Some((2, 4)));
		assert_eq!(neighbors.next(), Some((3, 2)));
		// assert_eq!(neighbors.next(), Some((3, 3)));
		assert_eq!(neighbors.next(), Some((3, 4)));
		assert_eq!(neighbors.next(), Some((4, 2)));
		assert_eq!(neighbors.next(), Some((4, 3)));
		assert_eq!(neighbors.next(), Some((4, 4)));
		assert_eq!(neighbors.next(), None);
	}

	#[test]
	fn edge_off_center() {
		let mut neighbors = neighbors::<5>(3, 4);

		assert_eq!(neighbors.next(), Some((2, 3)));
		assert_eq!(neighbors.next(), Some((2, 4)));
		assert_eq!(neighbors.next(), Some((3, 3)));
		// assert_eq!(neighbors.next(), Some((3, 4)));
		assert_eq!(neighbors.next(), Some((4, 3)));
		assert_eq!(neighbors.next(), Some((4, 4)));
		assert_eq!(neighbors.next(), None);
	}
}
