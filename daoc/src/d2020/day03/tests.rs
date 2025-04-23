use super::*;

#[cfg(test)]
mod slope {
	use super::slope;

	#[cfg(test)]
	fn test_slope() -> Vec<Vec<char>> {
		let data = include_str!("test-data");
		let data: Vec<Vec<char>> = data
			.split('\n')
			.filter(|s| !s.is_empty())
			.map(|line| line.chars().collect())
			.collect();

		data
	}

	#[test]
	fn slope_3_1_correct() {
		assert_eq!(slope(&test_slope(), (3, 1)), 7);
	}

	#[test]
	fn slope_1_1_correct() {
		assert_eq!(slope(&test_slope(), (1, 1)), 2);
	}

	#[test]
	fn slope_5_1_correct() {
		assert_eq!(slope(&test_slope(), (5, 1)), 3);
	}

	#[test]
	fn slope_7_1_correct() {
		assert_eq!(slope(&test_slope(), (7, 1)), 4);
	}

	#[test]
	fn slope_1_2_correct() {
		assert_eq!(slope(&test_slope(), (1, 2)), 2);
	}
}
