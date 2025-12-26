use core::ops::RangeInclusive;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::{PartSolve, Solver, export_solver, part_test};

#[derive(Default)]
struct Solution;

#[derive(Clone, Copy, Debug)]
enum DiskMapDescriptor {
	File { size: u8, id: u16 },
	Gap { size: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DiskEntry(Option<u16>);

#[derive(Clone, Debug)]
struct Disk {
	entries: VecDeque<DiskEntry>,
}

type GapSet = BTreeSet<(usize, usize)>;
type FileRangeMap = BTreeMap<u16, (usize, usize)>;

impl Disk {
	fn draw(&self) {
		for entry in &self.entries {
			match entry.0 {
				Some(id) if id < 10 => print!("{id}"),
				Some(_id) => print!("#"),
				None => print!("."),
			}
		}

		println!();
	}

	fn next_file_block_back(&mut self) -> Option<DiskEntry> {
		while let DiskEntry(None) = self.entries.back()? {
			// Skip gaps.
			self.entries.pop_back();
		}

		self.entries.pop_back()
	}

	fn checksum(&self) -> u64 {
		self
			.entries
			.iter()
			.enumerate()
			.fold(0_u64, |acc, (pos, entry)| {
				acc
					+ match entry.0 {
						Some(id) => u64::try_from(pos).unwrap() * u64::from(id),
						None => 0_u64,
					}
			})
	}

	fn compactify_bad(&mut self, print: bool) {
		let mut cursor: usize = 0_usize;
		let mut still_has_gaps: bool = true;

		if self.entries.is_empty() {
			return;
		}

		while still_has_gaps {
			if print {
				self.draw();
			}

			// If current spot is a gap, fill it in.
			if self
				.entries
				.get(cursor)
				.is_some_and(|entry| entry.0.is_none())
			{
				if let Some(file_block) = self.next_file_block_back() {
					self.entries[cursor] = file_block;
				} else {
					still_has_gaps = false;
				}
			}

			cursor += 1;

			if cursor >= self.entries.len() {
				still_has_gaps = false;
			}
		}
	}

	fn find_gaps_and_files(&self) -> (GapSet, FileRangeMap) {
		let mut gap_ranges: GapSet = BTreeSet::new();
		let mut file_ranges: FileRangeMap = BTreeMap::new();

		let mut cursor: usize = 0_usize;

		while cursor < self.entries.len() {
			if let Some(id) = self.entries[cursor].0 {
				// Start of a file.
				let start = cursor;

				// Move cursor rightwards until we find the end of the file.
				while cursor + 1 < self.entries.len() && self.entries[cursor + 1].0 == Some(id) {
					cursor += 1;
				}

				let end = cursor;

				file_ranges.insert(id, (start, end));

				cursor += 1;
			} else {
				// Start of a gap.
				let start = cursor;

				// Move cursor rightwards until we find the end of the gap.
				while cursor + 1 < self.entries.len() && self.entries[cursor + 1].0.is_none() {
					cursor += 1;
				}

				let end = cursor;

				gap_ranges.insert((start, end));

				cursor += 1;
			}
		}

		(gap_ranges, file_ranges)
	}

	fn move_block(&mut self, from_range: RangeInclusive<usize>, to_range: RangeInclusive<usize>) {
		for (from, to) in from_range.zip(to_range) {
			self.entries.swap(from, to);
		}
	}

	fn pick_gap(
		gap_ranges: &BTreeSet<(usize, usize)>,
		file_range: RangeInclusive<usize>,
	) -> Option<RangeInclusive<usize>> {
		let file_size = file_range.clone().count();

		for (gap_start, gap_end) in gap_ranges {
			let gap_range = *gap_start..=*gap_end;

			let gap_size = gap_range.clone().count();

			if gap_start >= file_range.start() {
				return None;
			}

			if gap_size >= file_size {
				return Some(gap_range);
			}
		}

		None
	}

	// What about compacting by moving whole files instead of just one block at a time?
	// That way, files are contiguous on disk in the end, which is far more efficient for reading.
	fn compactify_better(&mut self, print: bool) {
		// First, we'll find all the gaps and files on the disk.
		let (mut gap_ranges, mut file_ranges) = self.find_gaps_and_files();

		let file_ids: Vec<u16> = file_ranges.keys().rev().copied().collect();

		// Loop over each file in reverse order (largest id to smallest id).
		for file_id in file_ids {
			// TODO: For each file, find the range and size.
			let file_range: RangeInclusive<usize> = file_ranges
				.get(&file_id)
				.map(|(start, end)| *start..=*end)
				.unwrap();

			let file_size: usize = file_range.clone().count();

			// Find the first gap that is equal to or larger than the size of the file.
			// If found, move the entire file to the start of that gap.
			if let Some(gap) = Self::pick_gap(&gap_ranges, file_range.clone()) {
				// We found a gap that can fit the file.
				self.move_block(file_range.clone(), gap.clone());

				if print {
					self.draw();
				}

				// Remove the stale gap from the tracker.
				// Remove the stale file from the tracker.
				gap_ranges.remove(&(*gap.start(), *gap.end()));
				file_ranges.remove(&file_id);

				// If the gap was larger than the file, create a new smaller gap.
				let gap_size = gap.clone().count();

				if gap_size > file_size {
					let new_gap_start = gap.start() + file_size;
					let new_gap_end = gap.end();

					gap_ranges.insert((new_gap_start, *new_gap_end));
				}
			}
		}
	}
}

#[cfg(test)]
mod disk {
	use super::{BTreeSet, Disk, DiskEntry, VecDeque};

	#[test]
	fn find_gaps_and_files() {
		let disk = Disk {
			entries: VecDeque::from(vec![
				DiskEntry(Some(0)),
				DiskEntry(Some(0)),
				DiskEntry(None),
				DiskEntry(None),
				DiskEntry(Some(1)),
				DiskEntry(Some(1)),
				DiskEntry(Some(1)),
				DiskEntry(None),
				DiskEntry(Some(2)),
			]),
		};

		let (gaps, files) = disk.find_gaps_and_files();

		assert_eq!(gaps, BTreeSet::from([(2, 3), (7, 7)]));

		assert_eq!(files.len(), 3);
		assert_eq!(files.get(&0), Some(&(0, 1)));
		assert_eq!(files.get(&1), Some(&(4, 6)));
		assert_eq!(files.get(&2), Some(&(8, 8)));
	}

	#[test]
	fn move_block() {
		let mut disk = Disk {
			entries: VecDeque::from(vec![
				DiskEntry(Some(0)),
				DiskEntry(Some(0)),
				DiskEntry(None),
				DiskEntry(None),
				DiskEntry(Some(1)),
				DiskEntry(Some(1)),
				DiskEntry(Some(1)),
				DiskEntry(None),
				DiskEntry(Some(2)),
			]),
		};

		disk.move_block(8..=8, 2..=2);

		assert_eq!(
			disk.entries,
			VecDeque::from(vec![
				DiskEntry(Some(0)),
				DiskEntry(Some(0)),
				DiskEntry(Some(2)),
				DiskEntry(None),
				DiskEntry(Some(1)),
				DiskEntry(Some(1)),
				DiskEntry(Some(1)),
				DiskEntry(None),
				DiskEntry(None),
			])
		);
	}
}

impl From<Vec<DiskMapDescriptor>> for Disk {
	fn from(disk_map: Vec<DiskMapDescriptor>) -> Self {
		let size_hint = disk_map.iter().fold(0usize, |acc, entry| {
			acc
				+ match entry {
					DiskMapDescriptor::File { size, .. } | DiskMapDescriptor::Gap { size } => {
						usize::from(*size)
					}
				}
		});

		let entries = {
			let mut entries = vec![DiskEntry(None); size_hint];

			let mut pos = 0usize;

			for entry in disk_map {
				match entry {
					DiskMapDescriptor::File { size, id } => {
						for _ in 0..size {
							entries[pos] = DiskEntry(Some(id));
							pos += 1;
						}
					}
					DiskMapDescriptor::Gap { size } => {
						for _ in 0..size {
							entries[pos] = DiskEntry(None);
							pos += 1;
						}
					}
				}
			}

			entries.into()
		};

		Self { entries }
	}
}

impl PartSolve for Solution {
	fn parse(&mut self, disk_map_str: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
		let disk_map: Vec<u8> = disk_map_str
			.trim()
			.chars()
			.map(|c| {
				u8::try_from(c.to_digit(10).unwrap())
					.expect("expected single digit which should be able to easily fit in u8")
			})
			.collect::<Vec<_>>();

		let disk_map: Vec<DiskMapDescriptor> = disk_map
			.chunks(2)
			.enumerate()
			.flat_map(|(id, chunk)| {
				let id = u16::try_from(id).expect("expected not to exceed u16::MAX files");

				let file_size = chunk.first().map(|file_size| DiskMapDescriptor::File {
					size: *file_size,
					id,
				});

				let free_size = chunk
					.get(1)
					.map(|free_size| DiskMapDescriptor::Gap { size: *free_size });

				[file_size, free_size].into_iter().flatten()
			})
			.collect();

		Ok(Box::new(disk_map))
	}

	fn part_one(&self, disk_map: &dyn core::any::Any) -> Option<String> {
		let disk_map: &Vec<DiskMapDescriptor> = disk_map.downcast_ref::<Vec<DiskMapDescriptor>>()?;
		let disk_map: Vec<DiskMapDescriptor> = disk_map.clone();

		let _total_size = disk_map.iter().fold(0usize, |acc, entry| {
			acc
				+ match entry {
					DiskMapDescriptor::File { size, .. } | DiskMapDescriptor::Gap { size } => {
						usize::from(*size)
					}
				}
		});

		let mut disk = Disk::from(disk_map);
		disk.compactify_bad(false);

		Some(disk.checksum().to_string())
	}

	fn part_two(&self, disk_map: &dyn core::any::Any) -> Option<String> {
		let disk_map: &Vec<DiskMapDescriptor> = disk_map.downcast_ref::<Vec<DiskMapDescriptor>>()?;
		let disk_map: Vec<DiskMapDescriptor> = disk_map.clone();

		let mut disk = Disk::from(disk_map);
		disk.compactify_better(false);

		Some(disk.checksum().to_string())
	}
}

export_solver!(solver, Solver::PartSolve(Box::new(Solution)));

part_test!(part_one, Solution, file "day09.example.in.txt", part_one, literal "1928");

part_test!(part_two, Solution, file "day09.example.in.txt", part_two, literal "2858");
