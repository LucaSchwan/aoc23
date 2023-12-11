run DAY:
  cargo watch -x "run --bin day{{DAY}}"

test DAY:
  cargo watch -x "test --bin day{{DAY}}"

clippy_watch:
  cargo watch -s "./clippy_dirty"

clippy:
  ./clippy_dirty

build_aoc:
  cargo build --bin aoc23

run_aoc: build_aoc
  ./target/debug/aoc23

load DAY: build_aoc
  ./target/debug/aoc23 load {{DAY}}

docs:
  rustup docs --std
