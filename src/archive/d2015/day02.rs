use core::num::ParseIntError;

pub type Intermediate = Vec<Gift>;
pub type Output = usize;

pub struct Gift {
	length: usize,
	width: usize,
	height: usize,
}

impl Gift {
	fn side_areas(&self) -> [usize; 3] {
		[
			self.length * self.width,
			self.width * self.height,
			self.height * self.length,
		]
	}

	fn side_perimeters(&self) -> [usize; 3] {
		[
			2 * (self.length + self.width),
			2 * (self.width + self.height),
			2 * (self.height + self.length),
		]
	}

	fn surface_area(&self) -> usize {
		let three_side_sum: usize = self.side_areas().into_iter().sum();
		2_usize * three_side_sum
	}

	fn area_of_smallest_side(&self) -> usize {
		self.side_areas().into_iter().min().unwrap()
	}

	fn wrapping_paper_required(&self) -> usize {
		self.surface_area() + self.area_of_smallest_side()
	}

	fn volume(&self) -> usize {
		self.length * self.width * self.height
	}

	fn perimeter_of_smallest_side(&self) -> usize {
		self.side_perimeters().into_iter().min().unwrap()
	}

	fn ribbon_required(&self) -> usize {
		self.perimeter_of_smallest_side() + self.volume()
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	fn line_to_gift(line: &str) -> Gift {
		line.parse().unwrap()
	}

	Ok(input.lines().map(line_to_gift).collect())
}

#[must_use]
pub fn part_one(gifts: &Intermediate) -> Option<Output> {
	Some(gifts.iter().map(Gift::wrapping_paper_required).sum())
}

#[must_use]
pub fn part_two(gifts: &Intermediate) -> Option<Output> {
	Some(gifts.iter().map(Gift::ribbon_required).sum())
}

crate::generate_solver!(solve, =>, self);

impl core::str::FromStr for Gift {
	type Err = ParseIntError;
	fn from_str(line: &str) -> Result<Self, Self::Err> {
		let dimensions: Vec<usize> = line
			.split('x')
			.map(usize::from_str)
			.filter_map(Result::ok)
			.collect();

		let (length, width, height) = (dimensions[0], dimensions[1], dimensions[2]);

		Ok(Self {
			length,
			width,
			height,
		})
	}
}

#[cfg(test)]
mod ribbon {
	use super::Gift;

	#[test]
	fn dimensions_2_3_4_correct() {
		let gift: Gift = Gift {
			length: 2,
			width: 3,
			height: 4,
		};
		assert_eq!(gift.side_perimeters(), [10, 14, 12]);
		assert_eq!(gift.volume(), 24);
		assert_eq!(gift.perimeter_of_smallest_side(), 10);
		assert_eq!(gift.ribbon_required(), 34);
	}

	#[test]
	fn dimensions_1_1_10_correct() {
		let gift: Gift = Gift {
			length: 1,
			width: 1,
			height: 10,
		};
		assert_eq!(gift.side_perimeters(), [4, 22, 22]);
		assert_eq!(gift.volume(), 10);
		assert_eq!(gift.perimeter_of_smallest_side(), 4);
		assert_eq!(gift.ribbon_required(), 14);
	}
}
