use {
	core::iter::empty,
	std::{
		collections::{HashMap, HashSet},
		rc::Rc,
	},
};

use super::{edge::Edge, node::Node};

#[derive(Clone, Debug)]
pub struct Graph {
	edges: HashMap<Rc<Node>, HashSet<Rc<Node>>>,
}

impl Graph {
	pub fn to_graphviz(&self) -> String {
		let mut string: String = String::new();

		for (left, right_set) in &self.edges {
			for right in right_set {
				use core::fmt::Write;
				writeln!(string, "{left} -- {right}").unwrap();
			}
		}

		string
	}

	pub fn nodes_from(&self, node: &Node) -> Box<dyn Iterator<Item = &Node> + '_> {
		match self.edges.get(node) {
			Some(set) => Box::new(set.iter().map(Rc::as_ref)),
			None => Box::new(empty()),
		}
	}
}

impl FromIterator<Edge> for Graph {
	fn from_iter<T: IntoIterator<Item = Edge>>(iter: T) -> Self {
		let mut edges = HashMap::new();

		for edge in iter {
			let (left, right) = edge.into();

			let left = Rc::new(left);
			let right = Rc::new(right);

			// Add edges (L -> R and R -> L).
			edges
				.entry(left.clone())
				.or_insert(HashSet::default())
				.insert(right.clone());

			edges
				.entry(right.clone())
				.or_insert(HashSet::default())
				.insert(left.clone());
		}

		Graph { edges }
	}
}
