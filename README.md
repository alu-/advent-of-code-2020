# Advent of Code 2020

> Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. People use them as a speed contest, interview prep, company training, university coursework, practice problems, or to challenge each other.
>
>[adventofcode.com](https://adventofcode.com/)

## Preamble
I've been writing and reading Rust code for about 24 hours, so expect some funkiness in the source.
As always the focus is to solve the issue at hand and not go back and refactor too much. 
Libraries and utilities might get refactored but shouldn't break old code.

### Running
```bash
cargo run
```

#### Tests
```bash
cargo test
```

##### Bootstrapper
I simple tool I quickly made to "get ready" for this year's advent of code. It simply downloads the question and input into two directories.
It also creates a solution file for me to start coding in.
The bootstrapper will not work without a configuration file (bootstrapper/config.toml) and refuses to work outside of an active advent of code.
```bash
cargo run -p bootstrapper
```