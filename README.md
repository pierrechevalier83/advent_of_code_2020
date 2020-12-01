My solutions for the "advent of code" challenge of 2020.
===

[Advent of code 2020](https://adventofcode.com/2020)

For each day, there is a binary which outputs the answers to parts 1 and 2 of the question.

For each challenge, at least 2 unit tests exist asserting that parts 1 and 2 are correct (after manually verifying the answers on the website). These serve as regression tests in case I touch up some previous days (for instance, to reduce duplication with later days)

To run a specific day (e.g day 1), use
```
cargo run --release -p 01
```

To run all days at once, use
```
cargo run --release
```

To run all unit tests, use
```
cargo test --release --workspace
```
