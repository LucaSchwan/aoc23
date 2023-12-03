# AOC 2023

These are my solutions for Advent of Code 2023.

This project uses `just`. Look at the recipes by running `just --list`.

The `aoc23` binary can show a simple usage as well as download the input files.

```
cargo run --bin aoc23 usage
```

```
cargo run --bin aoc23 load [Number of day]
```

If you are running NixOS, the shell.nix includes extra dependencies for building
`openssl` used by the `reqwest` crate. Note: you still need rust installed in your
system, I haven't come around to having shells be purely every dependency you need.
