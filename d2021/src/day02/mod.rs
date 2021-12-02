pub enum Command {
	Forward(usize),
	Down(usize),
	Up(usize),
}

#[derive(Debug, thiserror::Error)]
pub enum CommandParseError {
	#[error("empty line")]
	EmptyLine,
	#[error("invalid units")]
	ParseUnits(#[from] std::num::ParseIntError),
	#[error("invalid command")]
	InvalidCommand,
}

impl core::str::FromStr for Command {
	type Err = CommandParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// TODO: DRY this up a bit, maybe collect components?
		let mut split = s.split(' ');
		match (split.next(), split.next(), split.next()) {
			(Some("forward"), Some(units), None) => units
				.parse()
				.map(Command::Forward)
				.map_err(CommandParseError::from),
			(Some("down"), Some(units), None) => units
				.parse()
				.map(Command::Down)
				.map_err(CommandParseError::from),
			(Some("up"), Some(units), None) => units
				.parse()
				.map(Command::Up)
				.map_err(CommandParseError::from),
			(Some(_), _, _) => Err(CommandParseError::InvalidCommand),
			(None, _, _) => Err(CommandParseError::EmptyLine),
		}
	}
}

type Intermediate = Vec<Command>;

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.map(str::parse)
		.collect::<Result<Vec<_>, CommandParseError>>()
		.unwrap()
}

type Solution = usize;

struct State {
	position: usize,
	aim: Option<usize>,
	depth: usize,
}

pub fn part_one(commands: &Intermediate) -> Option<Solution> {
	fn apply_state_transition(state: State, command: &Command) -> State {
		let State {
			position,
			depth,
			aim,
		} = state;

		match command {
			Command::Down(units) => State {
				position,
				aim,
				depth: depth + units,
			},
			Command::Up(units) => State {
				position,
				aim,
				depth: depth - units,
			},
			Command::Forward(units) => State {
				position: position + units,
				aim,
				depth,
			},
		}
	}

	let final_state = commands.iter().fold(
		State {
			position: 0,
			aim: None,
			depth: 0,
		},
		apply_state_transition,
	);

	Some(final_state.position * final_state.depth)
}

pub fn part_two(commands: &Intermediate) -> Option<Solution> {
	struct State {
		aim: usize,
		depth: usize,
		position: usize,
	}

	let final_state = commands.iter().fold(
		State {
			aim: 0,
			depth: 0,
			position: 0,
		},
		|State {
		   position,
		   depth,
		   aim,
		 },
		 command| match command {
			Command::Forward(units) => State {
				position: position + units,
				aim,
				depth: depth + aim * units,
			},
			Command::Down(units) => State {
				position,
				aim: aim + units,
				depth,
			},
			Command::Up(units) => State {
				position,
				aim: aim - units,
				depth,
			},
		},
	);

	Some(final_state.position * final_state.depth)
}
