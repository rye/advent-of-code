pub type Intermediate = State<10>;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	input.parse()
}

mod neighbors;
mod state;
use state::State;

type Output = usize;

#[must_use]
pub fn part_one(state: &Intermediate) -> Option<Output> {
	let mut state: State<10> = state.clone();

	let mut total_flashes: usize = 0;

	for _ in 0..100 {
		total_flashes += state.tick();
	}

	Some(total_flashes)
}

#[must_use]
pub fn part_two(state: &Intermediate) -> Option<Output> {
	let mut state: State<10> = state.clone();
	let mut step_counter: usize = 0;

	loop {
		let flashes = state.tick();
		step_counter += 1;

		if flashes == 10_usize * 10_usize {
			break;
		}
	}

	Some(step_counter)
}

daocutil::generate_solver!(solve, =>, self);
