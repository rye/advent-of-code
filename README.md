# advent-of-code

This repository houses my work on the annual [Advent of Code][aoc] challenge.
You're invited to give it a try!

[aoc]: https://adventofcode.com

## Usage

All years are part of a single crate.
Pass a year, a day, or both to choose which solvers to run:

```console
$ cargo run --release -- 2024 7     # 2024, day 7
$ cargo run --release -- 2024       # every solved day of 2024
$ cargo run --release -- 7          # day 7 of every year
```

Solvers read **your own** puzzle inputs from `inputs/YYYY/dayNN.txt` (or `inputs/archive/YYYY/dayNN.txt`), relative to the repository root.
That directory is gitignored. Advent of Code asks that inputs not be redistributed, so download your own from [adventofcode.com][aoc].

To run the tests, which use the public puzzle examples checked in under [`puzzles/`](puzzles/README.md):

```console
$ cargo test
```

## Puzzle inputs, examples, and descriptions

This repository includes only the small **example** inputs and answers from the puzzle descriptions, under `puzzles/YYYY/`.
The puzzle text itself and my personal inputs stay local and are gitignored.
See [`puzzles/README.md`](puzzles/README.md) for the layout and conventions.

## Spoilers Ahead!

This repository contains spoilers.
You've been warned!
If you have already solved the puzzle you're here to look at, congratulations!
If you haven't, I encourage you to give it your best effort first.

- My goal here is definitely _not_ to make the leaderboard.
  Rather, I hope to come up with simple, elegant, and performant solutions.

- **As much as possible**, I also aim to make solutions that share code as much as possible, too.
  It's pretty easy to write two entirely separate groups of functions per day, but being able to re-use intermediates, and do effectively the same calculations with only minor differences where part one and part two differ is (in my opinion) far more useful.

- I may make many revisions to my solutions as time goes on.
  Oftentimes I'll make a decision on the first pass and then decide there's a more elegant way to do things.

- You can expect to see solutions here after 6 hours _after_ the puzzle goes live.
  I am generally asleep hours before the puzzles go live, so sometimes it'll take me much longer than most to get to the solution.

## License

This project is licensed under the terms of the MIT license.
For more information, see the [LICENSE](LICENSE) file at the root of this repository.
Additional questions or requests for a license grant can be directed to <kristofer.rye@gmail.com>.
