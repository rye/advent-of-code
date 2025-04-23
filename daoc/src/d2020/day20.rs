use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const MONSTER: [&str; 3] = [
	"                  # ",
	"#    ##    ##    ###",
	" #  #  #  #  #  #   ",
];

type BorderMatches = HashMap<String, Vec<usize>>;
type Image<'a> = [&'a str; 10];

#[derive(Clone, Debug, Default)]
struct Tile {
	v: Vec<Vec<char>>,
	id: usize,
}

impl Tile {
	fn from_input(tile: &[&str], id: usize) -> Self {
		Self {
			v: tile.iter().map(|s| s.chars().collect()).collect(),
			id,
		}
	}

	fn get_edges(&self) -> (String, String) {
		let (mut b1, mut b2) = (String::new(), String::new());
		for i in 0..10 {
			b1.push(self.v[i][0]);
			b2.push(self.v[i][9]);
		}
		(b1, b2)
	}

	fn get_neighbor(&self, matches: &BorderMatches, n: usize) -> Option<usize> {
		let matches = match n {
			0 => &matches[&self.v[0].iter().copied().collect::<String>()],
			1 => &matches[&self.get_edges().1],
			2 => &matches[&self.v[9].iter().copied().collect::<String>()],
			3 => &matches[&self.get_edges().0],
			_ => unreachable!(),
		};
		matches.iter().find(|&&id| id != self.id).copied()
	}

	fn rotate(&mut self) {
		self.v = rotate(&self.v);
	}

	fn match_right(&self, matches: &BorderMatches, images: &HashMap<usize, Image>) -> Self {
		let id = self.get_neighbor(matches, 1).unwrap();
		let mut tile = Tile::from_input(&images[&id], id);

		while tile.get_neighbor(matches, 3) != Some(self.id) {
			tile.rotate();
		}

		if (0..10).any(|i| self.v[i][9] != tile.v[i][0]) {
			for i in 0..5 {
				tile.v.swap(i, 9 - i);
			}
		}
		tile
	}

	fn match_down(&self, matches: &BorderMatches, images: &HashMap<usize, Image>) -> Self {
		let id = self.get_neighbor(matches, 2).unwrap();
		let mut tile = Tile::from_input(&images[&id], id);

		while tile.get_neighbor(matches, 0) != Some(self.id) {
			tile.rotate();
		}

		if self.v[9] != tile.v[0] {
			for s in &mut tile.v {
				s.reverse();
			}
		}
		tile
	}
}

fn rotate(v: &[Vec<char>]) -> Vec<Vec<char>> {
	let (h, w) = (v.len(), v[0].len());
	let mut rot = vec![vec!['\0'; w]; h];
	for (i, j) in (0..h).cartesian_product(0..w) {
		rot[j][w - 1 - i] = v[i][j];
	}
	rot
}

fn build_image(
	images: &[(usize, Image)],
	matches: &BorderMatches,
	corner: usize,
) -> Vec<Vec<char>> {
	let images = images.iter().copied().collect::<HashMap<_, _>>();

	let mut starting_corner = Tile::from_input(&images[&corner], corner);
	while [0, 3]
		.iter()
		.any(|&d| starting_corner.get_neighbor(matches, d).is_some())
	{
		starting_corner.rotate();
	}

	let mut image = vec![vec![Tile::default(); 12]; 12];
	image[0][0] = starting_corner;

	for i in 1..12 {
		image[i][0] = image[i - 1][0].match_down(matches, &images);
	}

	for (i, j) in (0..12).cartesian_product(1..12) {
		image[i][j] = image[i][j - 1].match_right(matches, &images);
	}

	let mut actual_image = vec![Vec::new(); 8 * 12];

	for (i, j) in (0..12).cartesian_product(0..12) {
		let tile = &image[i][j];
		for k in 1..9 {
			actual_image[i * 8 + (k - 1)].extend(&tile.v[k][1..9]);
		}
	}

	actual_image
}

fn find_monsters(image: &[Vec<char>], monster_coords: &HashSet<(isize, isize)>) -> usize {
	let positions = image
		.iter()
		.enumerate()
		.flat_map(|(i, row)| {
			row
				.iter()
				.enumerate()
				.filter(|&(_, &c)| c == '#')
				.map(move |(j, _)| (isize::try_from(i).unwrap(), (isize::try_from(j).unwrap())))
		})
		.collect::<HashSet<_>>();

	positions
		.iter()
		.filter(|(i, j)| {
			monster_coords
				.iter()
				.map(|(di, dj)| (i + di, j + dj))
				.all(|pos| positions.contains(&pos))
		})
		.count()
}

pub type Intermediate<'a> = (Vec<(usize, [&'a str; 10])>, BorderMatches);
pub type Output = usize;

pub fn part_one((_images, matches): &Intermediate) -> Option<Output> {
	let mut count_map = HashMap::new();

	for ids in matches.values().filter(|ids| ids.len() == 1) {
		*count_map.entry(ids[0]).or_insert(0) += 1;
	}

	let corner_ids: Vec<usize> = count_map
		.iter()
		.filter(|&(_, &c)| c == 4)
		.map(|(id, _)| *id)
		.collect();

	Some(corner_ids.iter().product())
}

pub fn part_two((images, matches): &Intermediate) -> Option<Output> {
	const CORNER: usize = 3181;

	let monster_coords = MONSTER
		.iter()
		.enumerate()
		.flat_map(|(i, row)| {
			row
				.chars()
				.enumerate()
				.filter(|&(_, c)| c == '#')
				.map(move |(j, _)| (isize::try_from(i).unwrap() - 1, isize::try_from(j).unwrap()))
		})
		.collect::<HashSet<_>>();

	let mut image = build_image(images, matches, CORNER);

	let total = image.iter().flatten().filter(|c| **c == '#').count();

	loop {
		match find_monsters(&image, &monster_coords) {
			0 => image = rotate(&image),
			m => return Some(total - m * monster_coords.len()),
		}
	}
}

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let images: Vec<(usize, Image)> = input
		.split("\n\n")
		.map(|spec| {
			let lines: Vec<&str> = spec.lines().collect();

			let id: usize = lines[0]
				.split(' ')
				.nth(1)
				.unwrap()
				.split(':')
				.next()
				.unwrap()
				.parse()
				.unwrap();

			let body: Image = lines[1..=10].try_into().unwrap();

			(id, body)
		})
		.collect();

	let mut matches = HashMap::new();

	for &(id, tile) in &images {
		let (b1, b2) = Tile::from_input(&tile, id).get_edges();

		for edge in &[tile[0], tile[9], &b1, &b2] {
			matches
				.entry((*edge).to_string())
				.or_insert_with(Vec::new)
				.push(id);

			matches
				.entry(edge.chars().rev().collect())
				.or_insert_with(Vec::new)
				.push(id);
		}
	}

	Ok((images, matches))
}

daocutil::generate_solver!(solve, =>, self);
