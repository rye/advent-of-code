use core::str::FromStr;
use std::collections::{BTreeMap, VecDeque};

pub type Intermediate = VecDeque<Connection>;
pub type Output = u16;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(core::str::FromStr::from_str)
			.filter_map(Result::ok)
			.collect(),
	)
}

#[must_use]
pub fn part_one(connections: &Intermediate) -> Option<Output> {
	let mut signal_tracker = process_connections(connections);

	signal_tracker.remove(&WireId::from("a"))
}

#[must_use]
pub fn part_two(connections: &Intermediate) -> Option<Output> {
	let mut connections: VecDeque<Connection> = connections.clone();

	let mut signal_tracker = process_connections(&connections);

	let result = signal_tracker.remove(&WireId::from("a")).unwrap();

	connections.push_back(Connection {
		input: Input::Source(Source::Signal(result)),
		output: WireId("b".to_string()),
	});

	let mut signal_tracker = process_connections(&connections);

	let result: Option<u16> = signal_tracker.remove(&WireId::from("a"));

	result
}

type Signal = u16;

#[derive(PartialEq, Debug, Eq, Hash, Clone, PartialOrd, Ord)]
struct WireId(String);

impl<T> From<T> for WireId
where
	T: AsRef<str>,
{
	fn from(id: T) -> Self {
		Self(id.as_ref().to_string())
	}
}

impl core::fmt::Display for WireId {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(PartialEq, Debug, Clone)]
enum Source {
	Wire(WireId),
	Signal(Signal),
}

impl<T> From<T> for Source
where
	T: AsRef<str>,
{
	fn from(s: T) -> Self {
		match s.as_ref().parse::<Signal>() {
			Ok(signal) => Source::Signal(signal),
			Err(_) => Source::Wire(s.into()),
		}
	}
}

impl core::fmt::Display for Source {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			Self::Wire(wire) => write!(f, "{wire}"),
			Self::Signal(signal) => write!(f, "{signal}"),
		}
	}
}

#[derive(PartialEq, Debug, Clone)]
enum Input {
	Source(Source),
	And(Source, Source),
	Or(Source, Source),
	LShift(Source, u16),
	RShift(Source, u16),
	Not(Source),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Connection {
	input: Input,
	output: WireId,
}

impl core::fmt::Display for Connection {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let out = &self.output;

		match &self.input {
			Input::Source(source) => write!(f, "{source} -> {out}"),
			Input::And(a, b) => write!(f, "{a} AND {b} -> {out}"),
			Input::Or(a, b) => write!(f, "{a} OR {b} -> {out}"),
			Input::LShift(a, b) => write!(f, "{a} LSHIFT {b} -> {out}"),
			Input::RShift(a, b) => write!(f, "{a} RSHIFT {b} -> {out}"),
			Input::Not(a) => write!(f, "NOT {a} -> {out}"),
		}
	}
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
	#[error("invalid instruction")]
	InvalidInstruction,
	#[error("failed to parse signal")]
	SignalParse(#[from] core::num::ParseIntError),
}

impl FromStr for Connection {
	type Err = Error;

	fn from_str(specifier: &str) -> Result<Self, Self::Err> {
		let (lhs, rhs): (&str, &str) = {
			let split: Vec<_> = specifier.split(" -> ").collect();
			(
				split.first().ok_or(Error::InvalidInstruction)?,
				split.get(1).ok_or(Error::InvalidInstruction)?,
			)
		};

		let lhs_split: Vec<&str> = lhs.split(' ').collect();

		let input: Input = match lhs_split.len() {
			1 => match lhs_split.first().unwrap().parse::<Signal>() {
				Ok(signal) => Input::Source(Source::Signal(signal)),
				Err(_) => Input::Source(Source::Wire(lhs_split.first().unwrap().into())),
			},
			3 => {
				let a: Source = lhs_split.first().unwrap().into();
				let op = *lhs_split.get(1).unwrap();

				//let a: WireId = lhs_split.get(0).unwrap().into();

				match op {
					"AND" | "OR" => {
						let b: Source = lhs_split.get(2).unwrap().into();

						match op {
							"AND" => Input::And(a, b),
							"OR" => Input::Or(a, b),
							_ => unreachable!(),
						}
					}
					"LSHIFT" | "RSHIFT" => {
						let b: u16 = lhs_split.get(2).unwrap().parse().unwrap();

						match op {
							"LSHIFT" => Input::LShift(a, b),
							"RSHIFT" => Input::RShift(a, b),
							_ => unreachable!(),
						}
					}
					_ => unreachable!(),
				}
			}
			2 => match *lhs_split.first().unwrap() {
				"NOT" => Input::Not(lhs_split.get(1).unwrap().into()),
				_ => unreachable!(),
			},
			_ => unreachable!(),
		};

		let output: WireId = WireId(rhs.to_string());

		Ok(Connection { input, output })
	}
}

fn contains_source(signal_tracker: &BTreeMap<WireId, Signal>, source: &Source) -> bool {
	match source {
		Source::Signal(_) => true,
		Source::Wire(wire_id) => signal_tracker.contains_key(wire_id),
	}
}

fn eval_source(signal_tracker: &mut BTreeMap<WireId, Signal>, wire: &WireId, output: &WireId) {
	let signal: Signal = *signal_tracker
		.get(wire)
		.expect("called eval_source without valid entry for wire");
	signal_tracker.insert(output.clone(), signal);
}

fn eval_and(
	signal_tracker: &mut BTreeMap<WireId, Signal>,
	a: &Source,
	b: &Source,
	output: &WireId,
) {
	let signal_a: Signal = match a {
		Source::Signal(signal) => *signal,
		Source::Wire(wire) => *signal_tracker
			.get(wire)
			.expect("called eval_and without valid entry for input a"),
	};
	let signal_b: Signal = match b {
		Source::Signal(signal) => *signal,
		Source::Wire(wire) => *signal_tracker
			.get(wire)
			.expect("called eval_and without valid entry for input a"),
	};

	let output_signal: Signal = signal_a & signal_b;
	signal_tracker.insert(output.clone(), output_signal);
}

fn eval_or(signal_tracker: &mut BTreeMap<WireId, Signal>, a: &Source, b: &Source, output: &WireId) {
	let signal_a: Signal = match a {
		Source::Signal(signal) => *signal,
		Source::Wire(wire) => *signal_tracker
			.get(wire)
			.expect("called eval_and without valid entry for input a"),
	};
	let signal_b: Signal = match b {
		Source::Signal(signal) => *signal,
		Source::Wire(wire) => *signal_tracker
			.get(wire)
			.expect("called eval_and without valid entry for input a"),
	};

	let output_signal: Signal = signal_a | signal_b;
	signal_tracker.insert(output.clone(), output_signal);
}

fn eval_lshift(
	signal_tracker: &mut BTreeMap<WireId, Signal>,
	input: &Source,
	value: u16,
	output: &WireId,
) {
	let signal: Signal = match input {
		Source::Signal(signal) => *signal,
		Source::Wire(wire) => *signal_tracker
			.get(wire)
			.expect("called eval_and without valid entry for input a"),
	};

	let output_signal: Signal = signal << value;
	signal_tracker.insert(output.clone(), output_signal);
}

fn eval_rshift(
	signal_tracker: &mut BTreeMap<WireId, Signal>,
	input: &Source,
	value: u16,
	output: &WireId,
) {
	let signal: Signal = match input {
		Source::Signal(signal) => *signal,
		Source::Wire(wire) => *signal_tracker
			.get(wire)
			.expect("called eval_and without valid entry for input a"),
	};

	let output_signal: Signal = signal >> value;
	signal_tracker.insert(output.clone(), output_signal);
}

fn eval_not(signal_tracker: &mut BTreeMap<WireId, Signal>, input: &Source, output: &WireId) {
	let signal: Signal = match input {
		Source::Signal(signal) => *signal,
		Source::Wire(wire) => *signal_tracker
			.get(wire)
			.expect("called eval_and without valid entry for input a"),
	};

	let output_signal: Signal = !signal;
	signal_tracker.insert(output.clone(), output_signal);
}

fn process_connections(connections: &VecDeque<Connection>) -> BTreeMap<WireId, Signal> {
	// Isolate the connections that are supplying a direct input to a wire.
	let (signal_sources, mut connections): (VecDeque<&Connection>, VecDeque<&Connection>) =
		connections
			.iter()
			.partition(|&connection| matches!(connection.input, Input::Source(Source::Signal(_))));

	// Build up the initial state of the signal tracker, cloning the output so the tracker has ownership
	// of its own wire identifiers.
	//
	// TODO: Intern the strings somehow?
	let signal_sources: BTreeMap<WireId, Signal> = signal_sources
		.iter()
		.filter_map(|&connection| match connection.input {
			Input::Source(Source::Signal(signal)) => Some((connection.output.clone(), signal)),
			_ => unreachable!(),
		})
		.collect();

	let mut signal_tracker: BTreeMap<WireId, Signal> = signal_sources;

	while let Some(connection) = connections.pop_front() {
		if match &connection.input {
			Input::Source(Source::Wire(a)) => signal_tracker.contains_key(a),
			Input::Source(_) => unreachable!(),
			Input::And(a, b) | Input::Or(a, b) => {
				contains_source(&signal_tracker, a) && contains_source(&signal_tracker, b)
			}
			Input::LShift(a, _) | Input::RShift(a, _) | Input::Not(a) => {
				contains_source(&signal_tracker, a)
			}
		} {
			// Evaluate and place the result.
			match (&connection.input, &connection.output) {
				(Input::Source(Source::Wire(wire)), output) => {
					eval_source(&mut signal_tracker, wire, output);
				}
				(Input::Source(_), _) => unreachable!(),
				(Input::And(a, b), output) => eval_and(&mut signal_tracker, a, b, output),
				(Input::Or(a, b), output) => eval_or(&mut signal_tracker, a, b, output),
				(Input::LShift(input, value), output) => {
					eval_lshift(&mut signal_tracker, input, *value, output);
				}
				(Input::RShift(input, value), output) => {
					eval_rshift(&mut signal_tracker, input, *value, output);
				}
				(Input::Not(input), output) => eval_not(&mut signal_tracker, input, output),
			}
		} else {
			// Restore the connection back onto the stack.
			connections.push_back(connection);
		}
	}

	signal_tracker
}

#[cfg(test)]
mod connection {
	use core::str::FromStr;

	use super::{Connection, Input, Source};

	#[test]
	fn connect_123_x() {
		assert_eq!(
			Connection::from_str("123 -> x"),
			Ok(Connection {
				input: Input::Source(Source::Signal(123)),
				output: "x".into()
			})
		);
	}

	#[test]
	fn connect_456_y() {
		let connection = "456 -> y";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Source(Source::Signal(456)),
				output: "y".into(),
			})
		);
	}

	#[test]
	fn connect_xandy_d() {
		let connection = "x AND y -> d";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::And("x".into(), "y".into()),
				output: "d".into(),
			})
		);
	}
	#[test]
	fn connect_xory_e() {
		let connection = "x OR y -> e";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Or("x".into(), "y".into()),
				output: "e".into(),
			})
		);
	}

	#[test]
	fn connect_xlshift2_f() {
		let connection = "x LSHIFT 2 -> f";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::LShift("x".into(), 2),
				output: "f".into(),
			})
		);
	}

	#[test]
	fn connect_yrshift2_g() {
		let connection = "y RSHIFT 2 -> g";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::RShift("y".into(), 2),
				output: "g".into(),
			})
		);
	}

	#[test]
	fn connect_notx_h() {
		let connection = "NOT x -> h";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Not("x".into()),
				output: "h".into(),
			})
		);
	}

	#[test]
	fn connect_noty_i() {
		let connection = "NOT y -> i";
		assert_eq!(
			Connection::from_str(connection),
			Ok(Connection {
				input: Input::Not("y".into()),
				output: "i".into(),
			})
		);
	}
}

daocutil::generate_solver!(solve, =>, self);
