use {core::convert::Infallible, std::collections::BTreeMap};

#[derive(Debug)]
enum Line<'a> {
	Command(&'a str),
	Dir(&'a str),
	File(usize, &'a str),
}

impl<'a> TryFrom<&'a str> for Line<'a> {
	type Error = Infallible;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		// Lines come in three forms:
		//
		// - $ <cmd> <args...>
		// - dir <directory to explore>
		// - <size> filename

		if &value[0..2] == "$ " {
			Ok(Self::Command(&value[2..]))
		} else if &value[0..4] == "dir " {
			Ok(Self::Dir(&value[4..]))
		} else {
			let mut split = value.split(' ');

			match (split.next().map(str::parse), split.next()) {
				(Some(Ok(sz)), Some(name)) => Ok(Self::File(sz, name)),
				_ => unreachable!(),
			}
		}
	}
}

pub struct DirectoryTree(BTreeMap<Vec<String>, Option<usize>>);

pub type Intermediate = DirectoryTree;
pub type Output = usize;

fn canonicalize(slice: &[String]) -> Vec<String> {
	// Convert the slice into a "scratchpad", allowing to "blank out" parts of the string.
	let mut working: Vec<Option<String>> = slice.iter().cloned().map(Some).collect();

	// As long as we have a ".." in the path, "blank out" the ".." and its preceding piece.
	while let Some(idx) = working.iter().position(|c| match c {
		Some(reldir) if reldir == ".." => true,
		Some(_reldir) => false,
		None => false,
	}) {
		working[idx - 1] = None;
		working[idx] = None;
	}

	// Take all the non-blanked parts and return an array containing just the string.
	working.into_iter().flatten().collect()
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let lines: Vec<Line> = input
		.lines()
		.map(TryFrom::try_from)
		.collect::<Result<Vec<Line>, _>>()?;

	let mut cwd: Option<Vec<String>> = None;

	let mut tree = BTreeMap::default();

	for line in lines {
		match line {
			Line::Command(command) => {
				let parts: Vec<&str> = command.split(' ').collect();

				match parts.first() {
					Some(&"cd") => match (&mut cwd, parts[1]) {
						(None, "/") => {
							cwd.replace(vec![String::new()]);

							tree.insert(vec![String::new()], None);
						}
						(Some(cwd), reldir) => {
							let mut tmp = cwd.clone();
							tmp.push(reldir.to_string());
							*cwd = canonicalize(&tmp);
						}
						(None, _) => unreachable!(),
					},
					Some(&"ls") => {}
					_ => unreachable!(),
				}
			}
			Line::Dir(dirname) => match &cwd {
				Some(cwd) => {
					let mut key = cwd.clone();
					key.push(dirname.to_string());
					tree.insert(canonicalize(&key), None);
				}
				None => todo!(),
			},
			Line::File(size, filename) => match &cwd {
				Some(cwd) => {
					let mut key = cwd.clone();
					key.push(filename.to_string());
					tree.insert(canonicalize(&key), Some(size));
				}
				None => todo!(),
			},
		}
	}

	Ok(DirectoryTree(tree))
}

fn convert_tree_to_directory_sizes(
	DirectoryTree(tree): &DirectoryTree,
) -> BTreeMap<Vec<String>, usize> {
	let mut directory_sizes: BTreeMap<Vec<String>, usize> = BTreeMap::new();

	for (name, size) in tree {
		if size.is_none() {
			directory_sizes.insert(name.clone(), 0_usize);
		}
	}

	for (directory, directory_size) in &mut directory_sizes {
		for (entry, size) in tree {
			if let Some(size) = size {
				if directory.len() <= entry.len() && &entry[0..directory.len()] == directory.as_slice() {
					*directory_size += size;
				}
			}
		}
	}

	directory_sizes
}

#[test]
fn parse_ok() {
	macro_rules! p {
		($($component:literal)|*) => {
			vec![$($component.to_string()),*]
		};
	}

	let example = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

	let result = parse(example).expect("failed to parse");

	// - / (dir)
	//   - a (dir)
	//     - e (dir)
	//       - i (file, size=584)
	//     - f (file, size=29116)
	//     - g (file, size=2557)
	//     - h.lst (file, size=62596)
	//   - b.txt (file, size=14848514)
	//   - c.dat (file, size=8504156)
	//   - d (dir)
	//     - j (file, size=4060174)
	//     - d.log (file, size=8033020)
	//     - d.ext (file, size=5626152)
	//     - k (file, size=7214296)

	assert_eq!(
		vec![
			(p![""], None),
			(p!["" | "a"], None),
			(p!["" | "a" | "e"], None),
			(p!["" | "a" | "e" | "i"], Some(584)),
			(p!["" | "a" | "f"], Some(29116)),
			(p!["" | "a" | "g"], Some(2557)),
			(p!["" | "a" | "h.lst"], Some(62596)),
			(p!["" | "b.txt"], Some(14_848_514)),
			(p!["" | "c.dat"], Some(8_504_156)),
			(p!["" | "d"], None),
			(p!["" | "d" | "j"], Some(4_060_174)),
			(p!["" | "d" | "d.log"], Some(8_033_020)),
			(p!["" | "d" | "d.ext"], Some(5_626_152)),
			(p!["" | "d" | "k"], Some(7_214_296)),
		]
		.into_iter()
		.collect::<BTreeMap<Vec<String>, Option<usize>>>(),
		result.0
	);
}

#[must_use]
pub fn part_one(tree: &Intermediate) -> Option<Output> {
	let directory_sizes: BTreeMap<Vec<String>, usize> = convert_tree_to_directory_sizes(tree);

	Some(
		directory_sizes
			.iter()
			.filter_map(|(_dir, sz)| if *sz <= 100_000 { Some(sz) } else { None })
			.sum(),
	)
}

#[must_use]
pub fn part_two(tree: &Intermediate) -> Option<Output> {
	const FILE_SYSTEM_SIZE: usize = 70_000_000;
	const MIN_UNUSED_SIZE: usize = 30_000_000;

	let directory_sizes: BTreeMap<Vec<String>, usize> = convert_tree_to_directory_sizes(tree);

	let outer_directory_size: usize = *directory_sizes
		.get(&vec![String::new()])
		.expect("no size for root directory?!");

	let mut sizes_to_directories: BTreeMap<usize, Vec<Vec<String>>> = BTreeMap::new();

	for (directory, size) in directory_sizes {
		sizes_to_directories
			.entry(size)
			.or_default()
			.push(directory);
	}

	let current_unused = FILE_SYSTEM_SIZE - outer_directory_size;

	if current_unused < MIN_UNUSED_SIZE {
		let size_to_free_up = MIN_UNUSED_SIZE - current_unused;

		let sizes: Vec<usize> = sizes_to_directories.keys().copied().collect();

		sizes.iter().find(|size| **size > size_to_free_up).copied()
	} else {
		None
	}
}

daocutil::generate_solver!(solve, =>, self);
