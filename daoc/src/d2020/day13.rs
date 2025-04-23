use num_integer::Integer;

/// Compute the greatest common divisor of `a` and `b` using the Extended Euclidean Algorithm.
pub fn extended_gcd<T: Copy + Integer>(a: T, b: T) -> (T, T, T) {
	if a == T::zero() {
		(b, T::zero(), T::one())
	} else {
		let (gcd, x, y) = extended_gcd(b % a, a);
		(gcd, y - (b / a) * x, x)
	}
}

/// Compute the multiplicative inverse of `n` in Z_`modulus`, if one exists.
pub fn modular_multiplicative_inverse<T: Copy + Integer>(n: T, modulus: T) -> Option<T> {
	let (g, x, _) = extended_gcd(n, modulus);

	if g == T::one() {
		Some((x % modulus + modulus) % modulus)
	} else {
		None
	}
}

/// Compute the unique solution to a system of congruences using the Chinese Remainder Theorem.
///
/// If the `.1` subscripts of the given slice are pairwise coprime, and the `.0` subscripts of the
/// given slice are integers between 0 and their corresponding .1 subscript, then the Chinese
/// Remainder Theorem asserts that there is one and only one integer `x` between 0 and the product
/// of all of the `.1` subscripts for which the remainder of the Euclidean division of `x` and each
/// of the `.1` subscripts is the same as the value in the corresponding `.0` subscript.
///
/// See also the article on [Wikipedia](https://en.wikipedia.org/wiki/Chinese_remainder_theorem).
pub fn crt<T: Copy + Integer>(divisor_remainder_pairs: &[(T, T)]) -> T {
	// First, compute the product of all of the divisors.
	let product = divisor_remainder_pairs
		.iter()
		.fold(T::one(), |acc, (div, _rem)| acc * *div);

	let total = divisor_remainder_pairs
		.iter()
		.fold(T::zero(), |acc, (div, rem)| {
			let partial_product = product / *div;
			acc + *rem * partial_product * modular_multiplicative_inverse(partial_product, *div).unwrap()
		});

	total % product
}

pub type Intermediate = (i64, Vec<Option<i64>>);
type Output = i64;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	let timestamp = data.lines().next().unwrap().parse::<i64>().unwrap();

	let bus_intervals: Vec<Option<i64>> = data
		.lines()
		.nth(1)
		.unwrap()
		.split(',')
		.map(|n| match n {
			"x" => None,
			k => k.parse::<i64>().ok(),
		})
		.collect();

	Ok((timestamp, bus_intervals))
}

pub fn part_one((timestamp, bus_intervals): &Intermediate) -> Option<Output> {
	// Turn each bus ID (period) into ([time until next stop], [period])
	let mut values: Vec<(i64, i64)> = bus_intervals
		.iter()
		.filter_map(|o| *o)
		.map(|n| ((timestamp / n) * n + n - timestamp, n))
		.collect();

	// Sort. For tuples, the first subscript is sorted on first, so we'll
	// get the earliest next stop at the start of the list.
	values.sort_unstable();

	// Answer is the time until next stop * ID of the bus.
	let result = values[0].0 * values[0].1;

	Some(result)
}

pub fn part_two((_timestamp, bus_intervals): &Intermediate) -> Option<Output> {
	let divisors_and_remainders: Vec<(i64, i64)> = bus_intervals
		.iter()
		.enumerate()
		.filter(|(_, x)| x.is_some())
		.map(|(i, x)| (i, x.unwrap()))
		.map(|(idx, bus_id)| (bus_id, bus_id - i64::try_from(idx).unwrap()))
		.collect();

	Some(crt(&divisors_and_remainders))
}

crate::generate_solver!(solve, =>, self);
