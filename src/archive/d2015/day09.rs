use std::collections::{HashMap, HashSet, hash_map::Entry};

use itertools::Itertools;
use regex::Regex;

pub type Intermediate = RouteDistances;
pub type Output = Distance;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let regex: Regex = Regex::new(LINE_PARSE_RE).unwrap();

	// First, process all the lines down to a collection of each of the pieces.
	let lines: Vec<(&str, &str, usize)> = input
		.lines()
		.filter_map(|line| parse_line(&regex, line))
		.collect();

	// Then, build the distance/place map.  Store:
	//
	// - the list of all seen places,
	// - the individual distances between A and B and B and A.
	let mut distances: DistanceMap = DistanceMap::default();
	let mut places: PlaceSet = PlaceSet::default();

	for line in lines {
		let start = line.0;
		let end = line.1;

		places.insert(start);
		places.insert(end);

		let distance = line.2;

		let normal_key = (start, end);
		let reverse_key = (end, start);

		if let Entry::Vacant(normal_entry) = distances.entry(normal_key) {
			normal_entry.insert(distance);
		}

		if let Entry::Vacant(reverse_entry) = distances.entry(reverse_key) {
			reverse_entry.insert(distance);
		}
	}

	// Finally, permute all the places together and produce a mapping of routes to their total distance

	Ok(
		all_routes(&places)
			.map(|route| total_distance(&distances, &route))
			.collect(),
	)
}

#[must_use]
pub fn part_one(route_distances: &Intermediate) -> Option<Output> {
	route_distances.iter().min().map(ToOwned::to_owned)
}

#[must_use]
pub fn part_two(route_distances: &Intermediate) -> Option<Output> {
	route_distances.iter().max().map(ToOwned::to_owned)
}

type Distance = usize;
type Place<'p> = &'p str;

type DistanceMap<'p> = HashMap<(Place<'p>, Place<'p>), Distance>;
type PlaceSet<'p> = HashSet<Place<'p>>;
type RouteDistances = Vec<Distance>;

const LINE_PARSE_RE: &str = r"^(?P<start>\w+) to (?P<end>\w+) = (?P<distance>\d+)$";

#[allow(clippy::needless_pass_by_value)]
fn extract_line_captures(
	captures: regex::Captures<'_>,
) -> (
	Option<regex::Match<'_>>,
	Option<regex::Match<'_>>,
	Option<regex::Match<'_>>,
) {
	(
		captures.name("start"),
		captures.name("end"),
		captures.name("distance"),
	)
}

fn process_line_captures<'input>(
	(start, end, distance): (
		Option<regex::Match<'input>>,
		Option<regex::Match<'input>>,
		Option<regex::Match<'input>>,
	),
) -> Option<(&'input str, &'input str, usize)> {
	match (start, end, distance) {
		(Some(start), Some(end), Some(distance)) => match distance.as_str().parse() {
			Ok(distance) => Some((start.as_str(), end.as_str(), distance)),
			_ => None,
		},
		_ => None,
	}
}

fn parse_line<'input>(
	regex: &Regex,
	line: &'input str,
) -> Option<(&'input str, &'input str, usize)> {
	regex
		.captures(line)
		.map(extract_line_captures)
		.and_then(process_line_captures)
}

fn all_routes<'places, 'input>(
	places: &'places HashSet<Place<'input>>,
) -> impl Iterator<Item = Vec<&'places &'input str>> {
	places.iter().permutations(places.len())
}

fn total_distance<'processing>(
	distances: &'processing DistanceMap,
	route: &[&'processing Place<'_>],
) -> usize {
	route
		.windows(2)
		.filter_map(|window| {
			let key = (*window[0], *window[1]);
			distances.get(&key)
		})
		.sum()
}

crate::generate_solver!(solve, =>, self);
