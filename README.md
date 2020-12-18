My solutions for the "advent of code" challenge of 2020.
===

[Advent of code 2020](https://adventofcode.com/2020)

This crate uses cargo-aoc for automating the boilerplate.

For each challenge, at least 2 unit tests exist asserting that parts 1 and 2 are correct (after manually verifying the answers on the website). These serve as regression tests in case I touch up some previous days (for instance, to reduce duplication with later days)

# Running existing solutions

To run the current day, use
```
cargo aoc
```
To benchmark the current day, use
```
cargo aoc bench
```

To run a specific day (e.g day 1), use
```
cargo aoc -d1
```
To run a specific day and part (e.g day 1, part 2), use
```
cargo aoc -d1 -p2
```

To run all solutions, use
```
cargo run --release
```
To run all unit tests, use
```
cargo test --release
```

# Preparing a new solution
To download the input for today, run
```
cargo aoc input
```

To download the input for a previous day X, run
```
cargo aoc input -dX
```

Code the generator to parse this day's input and the solutions to each part in `src/dayX.rs`.
Make your solution visible at the top level by adding `pub mod dayX` in `src/lib.rs`.

# Updating the session id 
If the session id expire, log in to the advent of code website, and obtain the cookie id (In Chrome: Shift+F9, Cookies tab, and copy the "Value" for the "session" field).
Then run
```
cargo aoc credentials -s <session id>
```

# Performance so far

Note: this is annecdotal. Benchmarks can be run on the faster ones for more precise data.
These were run on a 5 years old chromebook with a processor clocked at 2.4 GHz.

```
Day 1 - Part 1: 719796
	generator: 36.446µs,
	runner: 568ns

Day 1 - Part 2: 144554112
	generator: 14.457µs,
	runner: 2.606µs

Day 2 - Part 1: 517
	generator: 270.42µs,
	runner: 25.687µs

Day 2 - Part 2: 284
	generator: 226.291µs,
	runner: 37.349µs

Day 3 - Part 1: 195
	generator: 76.209µs,
	runner: 4.545µs

Day 3 - Part 2: 3772314000
	generator: 65.81µs,
	runner: 15.273µs

Day 4 - Part 1: 206
	generator: 558.66µs,
	runner: 4.783µs

Day 4 - Part 2: 123
	generator: 440.516µs,
	runner: 47.062µs

Day 5 - Part 1: 813
	generator: 157.209µs,
	runner: 583ns

Day 5 - Part 2: 612
	generator: 126.053µs,
	runner: 32.704µs

Day 6 - Part 1: 6686
	generator: 226.232µs,
	runner: 673.443µs

Day 6 - Part 2: 3476
	generator: 184.485µs,
	runner: 624.339µs

Day 7 - Part 1: 222
	generator: 837.783µs,
	runner: 529.499µs

Day 7 - Part 2: 13264
	generator: 831.036µs,
	runner: 1.404µs

Day 8 - Part 1: 1487
	generator: 57.969µs,
	runner: 1.766µs

Day 8 - Part 2: 1607
	generator: 62.443µs,
	runner: 109.154µs

Day 9 - Part 1: 373803594
	generator: 46.005µs,
	runner: 116.334µs

Day 9 - Part 2: 51152360
	generator: 36.322µs,
	runner: 110.968µs

Day 10 - Part 1: 3034
	generator: 4.678µs,
	runner: 4.554µs

Day 10 - Part 2: 259172170858496
	generator: 3.505µs,
	runner: 8.15µs

Day 11 - Part 1: 2427
	generator: 143ns,
	runner: 10.147305ms

Day 11 - Part 2: 2199
	generator: 312ns,
	runner: 14.080139ms

Day 12 - Part 1: 1457
	generator: 350ns,
	runner: 52.444µs

Day 12 - Part 2: 106860
	generator: 229ns,
	runner: 49.864µs

Day 13 - Part 1: 119
	generator: 12.57µs,
	runner: 768ns

Day 13 - Part 2: 1106724616194525
	generator: 6.57µs,
	runner: 43.471µs

Day 14 - Part 1: 7440382076205
	generator: 363.45µs,
	runner: 23.939µs

Day 14 - Part 2: 4200656704538
	generator: 359.083µs,
	runner: 10.843888ms

Day 15 - Part 1: 1238
	generator: 453ns,
	runner: 48.763µs

Day 15 - Part 2: 3745954
	generator: 175ns,
	runner: 2.376411589s

Day 16 - Part 1: 27850
	generator: 405ns,
	runner: 382.461µs

Day 16 - Part 2: 491924517533
	generator: 274ns,
	runner: 26.654402ms

Day 17 - Part 1: 273
	generator: 653ns,
	runner: 8.655878ms

Day 17 - Part 2: 1504
	generator: 626ns,
	runner: 214.516751ms
```
