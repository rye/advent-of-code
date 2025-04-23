use core::{convert::Infallible, str::FromStr};

use super::Node;

#[derive(Clone)]
pub struct Edge(Node, Node);

impl From<Edge> for (Node, Node) {
	fn from(edge: Edge) -> Self {
		(edge.0, edge.1)
	}
}

impl FromStr for Edge {
	type Err = Infallible;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let split: Vec<&str> = str.split('-').collect();

		match split[..] {
			[left, right] => Ok(Self(left.parse()?, right.parse()?)),
			_ => unreachable!(),
		}
	}
}
