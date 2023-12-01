run DAY:
  nix-shell --run "cargo run --bin day{{DAY}}"

test DAY:
  nix-shell --run "cargo test --bin day{{DAY}}"

build_aoc:
  nix-shell --run "cargo build --bin aoc23"

run_aoc: build_aoc
  ./target/debug/aoc23

docs:
  rustup docs --std
