use core::{
	convert::Infallible,
	fmt::{self, Display, Formatter},
	str::FromStr,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub enum Node {
	Start,
	SmallCave(String),
	LargeCave(String),
	End,
}

impl Display for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		use Node::{End, LargeCave, SmallCave, Start};

		match self {
			Start => write!(f, "start"),
			SmallCave(cave) | LargeCave(cave) => write!(f, "{cave}"),
			End => write!(f, "end"),
		}
	}
}

impl FromStr for Node {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"start" => Ok(Self::Start),
			"end" => Ok(Self::End),
			s if s.chars().all(|c| c.is_ascii_uppercase()) => Ok(Self::LargeCave(s.to_string())),
			s if s.chars().all(|c| c.is_ascii_lowercase()) => Ok(Self::SmallCave(s.to_string())),
			_ => todo!(),
		}
	}
}
