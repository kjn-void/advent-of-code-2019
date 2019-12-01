# Advent of Code 2019
Use advent of code as a way to learn the basics of the Rust programming language.

Every day is implemented as a separate module named _dayX_. Each
module contains a implementation of the trait _Solutions_.

## Running program

All solutions are grouped into a single executable. That executable
take two arguments; date of the day to solve and an input file.

```
$ cargo run 1 src/day1/input.txt
```

## Running tests

Each solution use the examples from the problem description as
unit-tests. All unit-tests are run through Cargo

```
$ cargo test
```
