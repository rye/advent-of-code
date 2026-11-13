# `puzzles/`

This directory holds the **example** inputs and answers from Advent of Code puzzle descriptions, which the unit tests use.
It is also where I keep local copies of the puzzle text, which are **not** committed.

## What's committed, and what isn't

Advent of Code's [policy](https://adventofcode.com/about#faq_copying) is that the puzzles are free to *use*, not free to *copy*:
repositories shouldn't include the puzzle text or personal puzzle inputs.
This repository follows that:

| Path | In git? | What it is |
| --- | --- | --- |
| `puzzles/YYYY/dayNN.exampleNN.in.txt` | ✅ yes | An example input shown in the puzzle description |
| `puzzles/YYYY/dayNN.exampleNN.out-1.txt` | ✅ yes | The expected part 1 answer for that example |
| `puzzles/YYYY/dayNN.exampleNN.out-2.txt` | ✅ yes | The expected part 2 answer for that example |
| `puzzles/YYYY/dayNN.md` | ❌ ignored | Local copy of the puzzle description (handy for prompting AI agents) |
| `inputs/` | ❌ ignored | My personal puzzle inputs (see below) |

The ignore rules live in the top-level [`.gitignore`](../.gitignore).
If you clone this repository, you won't have the `.md` files or `inputs/`.
Bring your own from [adventofcode.com](https://adventofcode.com).

## Example file conventions

- `YYYY` is the event year and `NN` is the zero-padded day, e.g. `2024/day01.example00.in.txt`.
- `exampleNN` numbers each example within a day, starting at `00`, in the order they appear in the puzzle.
  Days with several examples (different inputs for part 1 and 2, or many one-line cases) have `example00`, `example01`, ….
- `.in.txt` files hold the example input **verbatim** as the puzzle shows it, ending with exactly one newline, like a real input.
  (When the puzzle shows an example wrapped for display or as a list in prose, it's reshaped to match real-input format.)
- `.out-1.txt` / `.out-2.txt` hold just the answer (no prose), on one line, or several lines for answers drawn as ASCII art.
  An example that applies to only one part has only that part's output file.
- **Every example input has at least one answer file.**
  Answers are ones the puzzle states (or spells out with arithmetic, e.g. "7 … and 5 …" for a product), never ones computed by a solver.
  Snippets the puzzle uses only to illustrate a rule, and inputs it never gives a result for, aren't kept; a test that needs one inlines it.
- Some examples use different parameters than the real puzzle (a smaller grid, fewer steps, a different target row).
  Their answer files hold the answer the puzzle states for that example, so a solver with the real puzzle's parameters hard-coded won't reproduce it.

## Using examples in tests

The `example!` macro (defined in `src/main.rs`) includes a file from this directory at compile time.
The path is relative to `puzzles/`:

```rust
let input: &str = crate::example!("2022/day10.example00.in.txt");
```

For `PartSolve` solvers, `part_test!` accepts `example` paths for the input and, optionally, the expected output:

```rust
part_test!(part_one, Solution, example "2024/day01.example00.in.txt", part_one, example "2024/day01.example00.out-1.txt");
part_test!(part_two, Solution, example "2024/day07.example00.in.txt", part_two, literal "11387");
```

Prefer an example file over an inline string whenever the input comes from the puzzle text.
Inline strings are still fine for hand-made edge cases that aren't from a puzzle.

## Personal inputs

`cargo run` reads real inputs from `inputs/` relative to the current directory. It looks in:

1. `inputs/archive/YYYY/dayNN.txt`, then
2. `inputs/YYYY/dayNN.txt`.

Keep them there locally. The whole directory is gitignored.
