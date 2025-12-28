use core::any::Any;

/// Describes the behavior of common Advent of Code solvers
///
/// This trait arose out of the observation that Advent of Code challenges typically ask the player (that's you) to:
///
/// 1. Parse the input into some data structure.
/// 2. Use the data in that data structure to compute the answer for part one.
/// 3. Use the data in that data structure to compute the answer for part two.
///
/// This trait has several design aims:
///
/// - Parsing is fully separated from solving. The input can be deallocated before any solving begins.
///
/// - Part Solutions are fully independent from each other. They cannot mutate any shared state through the trait.
///
/// - Part Solutions can return `None` if they do not have an answer to provide (yet).
///
/// - Part Solutions return their answer as a heap-allocated `String`. This is flexible enough to handle days where
///   alphanumeric output is required to solve the puzzle (e.g. password challenges).
///
/// - In theory, one could run both parts in parallel, since they ought to be independent.
///
/// - The intermediate data structure is type-erased to `Box<dyn Any>`, allowing the implementer to choose any type
///   they wish to use as their "Intermediate".
pub trait PartSolve {
	/// Parse the provided `input` to an intermediate type.
	/// The resulting data will be passed in to `part_one` and `part_two`.
	///
	/// # Errors
	///
	/// If parsing fails for an unrecoverable reason, implementations can/should
	/// return an `Err` value.
	fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn Any>>;

	/// Solve the first part of the puzzle.
	///
	/// # Examples
	///
	/// Without accessing the intermediate data, you can simply return a value:
	///
	/// ```
	/// struct Solution;
	///
	/// impl aoc::PartSolve for Solution {
	///#    fn parse(&mut self, _input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
	///#        Ok(Box::new(42_u32))
	///#    }
	///#
	///     // parse, part_two omitted for brevity
	///
	///     fn part_one(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
	///         Some("42".to_string())
	///     }
	///#
	///#    fn part_two(&self, _intermediate: &dyn core::any::Any) -> Option<String> {
	///#        Some("42".to_string())
	///#    }
	/// }
	/// ```
	///
	/// To access the intermediate data, use [`(dyn Any)::downcast_ref`]:
	///
	/// ```
	/// # use aoc::PartSolve;
	/// struct Solution;
	///
	/// impl aoc::PartSolve for Solution {
	///     // Split a comma-separated list of numbers...
	///     fn parse(&mut self, input: &str) -> anyhow::Result<Box<dyn core::any::Any>> {
	///         let numbers: Vec<u32> = input.split(',').map(|num| num.parse().unwrap()).collect();
	///         Ok(Box::new(numbers))
	///     }
	///
	///     // ... sum them for part one ...
	///     fn part_one(&self, numbers: &dyn core::any::Any) -> Option<String> {
	///         let numbers: &Vec<u32> = numbers.downcast_ref()?;
	///         let sum: u32 = numbers.iter().sum();
	///         Some(sum.to_string())
	///     }
	///
	///     // ... and product them for part two.
	///     fn part_two(&self, numbers: &dyn core::any::Any) -> Option<String> {
	///         let numbers: &Vec<u32> = numbers.downcast_ref()?;
	///         let sum: u32 = numbers.iter().product();
	///         Some(sum.to_string())
	///     }
	/// }
	///
	/// let mut solver = Solution;
	/// let intermediate = solver.parse("1,2,3,4").unwrap();
	/// assert_eq!(Some("10".to_string()), solver.part_one(intermediate.as_ref()));
	/// assert_eq!(Some("24".to_string()), solver.part_two(intermediate.as_ref()));
	/// ```
	fn part_one(&self, intermediate: &dyn Any) -> Option<String>;

	/// Solve the second part of the puzzle.
	fn part_two(&self, intermediate: &dyn Any) -> Option<String>;
}
