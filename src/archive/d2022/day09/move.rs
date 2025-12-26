use core::{
	convert::Infallible,
	fmt::{self, Display, Formatter},
	str::FromStr,
};

use super::Direction;

pub struct Move {
	pub direction: Direction,
	pub distance: u16,
}

impl FromStr for Move {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let num_lines = s.lines().count();
		debug_assert_eq!(num_lines, 1);

		let mut split = s.split(' ');

		match (split.next(), split.next(), split.next()) {
			(Some(direction), Some(distance), None) => {
				let direction: Direction = direction.parse()?;
				let distance: u16 = distance.parse().expect("u16 parse err");

				Ok(Move {
					direction,
					distance,
				})
			}
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod parse {
	use core::mem::discriminant;

	use super::{Direction, Move};

	#[test]
	fn u_32() {
		let u_32 = "U 32".parse();
		assert!(u_32.is_ok());
		let u_32: Move = u_32.unwrap();
		assert_eq!(discriminant(&Direction::Up), discriminant(&u_32.direction));
		assert_eq!(32, u_32.distance);
	}

	#[test]
	fn r_17() {
		let r_17 = "R 17".parse();
		assert!(r_17.is_ok());
		let r_17: Move = r_17.unwrap();
		assert_eq!(
			discriminant(&Direction::Right),
			discriminant(&r_17.direction)
		);
		assert_eq!(17, r_17.distance);
	}
}

impl From<Move> for (i32, i32) {
	fn from(value: Move) -> Self {
		match value.direction {
			Direction::Up => (0, -i32::from(value.distance)),
			Direction::Down => (0, i32::from(value.distance)),
			Direction::Left => (-i32::from(value.distance), 0),
			Direction::Right => (i32::from(value.distance), 0),
		}
	}
}

#[cfg(test)]
mod to_i32_i32 {
	use super::{Direction, Move};

	#[test]
	fn conversion_to_i32_i32() {
		assert_eq!(
			(0, -32),
			Move {
				direction: Direction::Up,
				distance: 32
			}
			.into(),
		);

		assert_eq!(
			(0, 17),
			Move {
				direction: Direction::Down,
				distance: 17
			}
			.into(),
		);

		assert_eq!(
			(-7, 0),
			Move {
				direction: Direction::Left,
				distance: 7
			}
			.into(),
		);

		assert_eq!(
			(9, 0),
			Move {
				direction: Direction::Right,
				distance: 9
			}
			.into(),
		);
	}
}

impl Display for Move {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{} {}",
			match self.direction {
				Direction::Up => "U",
				Direction::Down => "D",
				Direction::Left => "L",
				Direction::Right => "R",
			},
			self.distance
		)
	}
}
