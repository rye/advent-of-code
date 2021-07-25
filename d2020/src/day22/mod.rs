use std::collections::{HashSet, VecDeque};

pub type Intermediate<'input> = (&'input str, &'input str);
pub type Solution = usize;

pub fn parse(input: &str) -> Intermediate {
	input.split_once("\n\n").unwrap()
}

pub fn part_one(chunks: &Intermediate) -> Option<Solution> {
	let (p1, p2) = (chunks.0, chunks.1);

	let mut p1: VecDeque<usize> = p1.lines().skip(1).map(|n| n.parse().unwrap()).collect();
	let mut p2: VecDeque<usize> = p2.lines().skip(1).map(|n| n.parse().unwrap()).collect();

	while !p1.is_empty() && !p2.is_empty() {
		let (c1, c2) = p1.pop_front().zip(p2.pop_front()).unwrap();
		if c1 > c2 {
			p1.push_back(c1);
			p1.push_back(c2);
		} else {
			p2.push_back(c2);
			p2.push_back(c1);
		}
	}

	Some(
		p1.iter()
			.chain(p2.iter())
			.rev()
			.enumerate()
			.map(|(i, c)| (i + 1) * c)
			.sum::<usize>(),
	)
}

const ALLOC_SIZE: usize = 16000;

pub fn part_two(chunks: &Intermediate) -> Option<Solution> {
	let (p1, p2) = (chunks.0, chunks.1);
	let p1 = Deck::from(
		&p1
			.lines()
			.skip(1)
			.map(|n| n.parse().unwrap())
			.collect::<Vec<_>>(),
	);
	let p2 = Deck::from(
		&p2
			.lines()
			.skip(1)
			.map(|n| n.parse().unwrap())
			.collect::<Vec<_>>(),
	);

	Some(
		if play(&p1, &p2) { p1 } else { p2 }
			.as_slice()
			.iter()
			.rev()
			.enumerate()
			.map(|(i, c)| (i + 1) * c)
			.sum::<usize>(),
	)
}

fn play(p1: &Deck, p2: &Deck) -> bool {
	let mut turns = HashSet::new();
	while turns.insert((p1.as_slice(), p2.as_slice())) {
		if p1.len() == 0 || p2.len() == 0 {
			return p1.len() > 0;
		}

		let (c1, c2) = (p1.pop(), p2.pop());
		if if c1 <= p1.len() && c2 <= p2.len() {
			let sp1 = Deck::from(&p1.as_slice()[..c1]);
			let sp2 = Deck::from(&p2.as_slice()[..c2]);
			sp1.as_slice().iter().max() >= sp2.as_slice().iter().max() || play(&sp1, &sp2)
		} else {
			c1 > c2
		} {
			p1.push(c1);
			p1.push(c2);
		} else {
			p2.push(c2);
			p2.push(c1);
		}
	}

	true
}

struct Deck {
	start: usize,
	end: usize,
	cards: [usize; ALLOC_SIZE],
}

// struct Deck(usize, usize, [usize; ALLOC_SIZE]);

impl Deck {
	fn from(slice: &[usize]) -> Self {
		let mut cards = [0; ALLOC_SIZE];
		cards[..slice.len()].copy_from_slice(&slice);
		Deck {
			start: 0,
			end: slice.len(),
			cards,
		}
	}

	fn as_slice(&self) -> &[usize] {
		&self.cards[self.start..self.end]
	}

	fn len(&self) -> usize {
		self.end - self.start
	}

	fn push(&self, card: usize) {
		unsafe {
			(*(&self.cards as *const _ as *mut [_; ALLOC_SIZE]))[self.end] = card;
			*(&self.end as *const _ as *mut usize) += 1;
		}
	}

	fn pop(&self) -> usize {
		unsafe {
			*(&self.start as *const _ as *mut usize) += 1;
		}
		self.cards[self.start - 1]
	}
}
