use std::{
	collections::{BTreeSet, HashSet},
	fmt::Debug,
	hash::BuildHasher,
	io::StdinLock,
};
use std::{io::BufRead, str::FromStr};

pub struct Lines<T> {
	inner: std::io::Lines<T>,
}

impl<T> Debug for Lines<T>
where
	T: Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.inner.fmt(f)
	}
}

impl<T> Iterator for Lines<T>
where
	T: BufRead,
{
	type Item = std::io::Result<String>;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}
}

impl<T> From<std::io::Lines<T>> for Lines<T> {
	fn from(inner: std::io::Lines<T>) -> Self {
		Self { inner }
	}
}

impl<'stdin, T> From<Lines<StdinLock<'stdin>>> for BTreeSet<T>
where
	T: FromStr,
	<T as FromStr>::Err: Debug,
	T: Ord,
{
	fn from(lines: Lines<StdinLock<'stdin>>) -> Self {
		lines
			.map(|line| line.unwrap().parse::<T>().unwrap())
			.collect()
	}
}

impl<'stdin, T, S: BuildHasher + Default> From<Lines<StdinLock<'stdin>>> for HashSet<T, S>
where
	T: FromStr,
	<T as FromStr>::Err: Debug,
	T: core::hash::Hash,
	T: Eq,
{
	fn from(lines: Lines<StdinLock<'stdin>>) -> Self {
		lines
			.map(|line| line.unwrap().parse::<T>().unwrap())
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::Lines;
	use static_assertions::assert_impl_all;

	#[cfg(test)]
	mod debug {
		use super::{assert_impl_all, Lines};

		#[derive(Debug)]
		#[allow(dead_code)]
		struct DebugDummy {}

		#[test]
		fn is_implemented() {
			assert_impl_all!(Lines<DebugDummy>: core::fmt::Debug);
		}

		#[test]
		fn transparent() {
			use std::io::BufRead;

			let buffer = std::io::Cursor::new("1\n2\n3\n");
			let std_lines = buffer.clone().lines();
			let cus_lines: Lines<std::io::Cursor<&str>> = buffer.clone().lines().into();

			assert_eq!(format!("{std_lines:?}"), format!("{:?}", cus_lines));
		}
	}
}
