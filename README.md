# Advent of Code 2023

This repository contains the solution for the [Advent of Code of 2023](https://adventofcode.com/2023) using the Rust language exclusively.

Because this challenge is part of a learning process of the Rust language, the solution may certainly not be the perfect solution and you can still find room for improvement on most of them.

## Get started

### Run solution code

The cargo CLI is what you'll need to run the code. But only one puzzle at a time, and only one part of a single day. Example: you can run the part 1 of the day 7. You can run the code using the input file or even the sample file provided by the Advent of Code.

For that, you need to set the Env Variables that live in the `.cargo/config.toml` file:

```toml
[env]
DAY = "7"
PART = "1"
USE_SAMPLE = "false"
```

`DAY` and `PART` are the day/part you want to run and `USE_SAMPLE` detect whether to run the `input.txt` or the `sample.txt` file inside the `src/day7` folder.

When done, you can run the CLI using the following command:

```
cargo run
```

### Script generation

This project contains a script generation feature that provides the ability to easily create files from a template using a single command line:

```
cargo run -- generate
```

It will prompt you to give the day of the puzzle. Once done, it will generate a new folder inside `src/`.

The template you can find in the `template/` folder consists of multiple files:

- sample.txt - the sample text provided by Advent of Code website
- input.txt - the input text provided by Advent of Code website
- mod.rs - a dummy module used to redirect to the part1 or part2 `run` function
- common.rs - a module used to export functions used by both the `part1` and `part2` modules, as long as a function to parse input
- part1.rs - the basic template which contains the part 1 `run` function
- part2.rs - the basic template which contains the part 2 `run` function

## Language

Rust 1.74.0

## Dependencies

| Name      | Version | Usage                                                                                                                                    |
| --------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| chrono    | 0.4.31  | Performance benchmark (time performance of the algorithm)                                                                                |
| clap      | 4.4.8   | Simplify creation of CLI (run algorithm, script generation, etc...)                                                                      |
| colored   | 2.0.4   | Apply colors and style on the CLI output                                                                                                 |
| itertools | 0.12.0  | Used to get access to specific iterator methods. <br />Like the immutable `sorted_by` function instead of the mutable `sort` function... |
| nom       | 7.1.3   | Used to parse input.                                                                                                                     |
| num       | 0.4.1   | Used to execute the `lcm` function for the day 8 part 2.                                                                                 |
