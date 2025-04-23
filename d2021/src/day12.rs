use core::str::FromStr;

mod node;
use node::Node;

mod edge;

mod graph;
use graph::Graph;

pub type Intermediate = Graph;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	input.lines().map(FromStr::from_str).collect()
}

type Solution = usize;

#[must_use]
pub fn part_one(graph: &Intermediate) -> Option<Solution> {
	println!("{}", graph.to_graphviz());

	None
}

#[must_use]
pub fn part_two(_graph: &Intermediate) -> Option<Solution> {
	None
}
