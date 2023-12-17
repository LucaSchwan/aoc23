use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use anyhow::Result;
use aoc23::Vec2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: isize,
    position: Vec2D,
    offset: Vec2D,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type BlockMap = Vec<Vec<isize>>;

fn find_least_heat_loss(input: BlockMap, min_steps: isize, max_steps: isize) -> isize {
    let mut dists = HashMap::new();

    let mut heap = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

    while let Some((cost, (y, x, d))) = heap.pop() {
        if (x, y) == (input[0].len() - 1, input.len() - 1) {
            return -cost;
        }

        if dists.get(&(y, x, d)).is_some_and(|&c| -cost > c) {
            continue;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if d == (dx, dy) || d == (-dx, -dy) {
                continue;
            }

            let mut next_cost = -cost;

            for dist in 1..=max_steps {
                let xx = (x as isize + dx * dist) as usize;
                let yy = (y as isize + dy * dist) as usize;

                if xx >= input[0].len() || yy >= input.len() {
                    continue;
                }
                next_cost += input[yy][xx];

                if dist < min_steps {
                    continue;
                }

                let key = (yy, xx, (dx, dy));

                if next_cost < *dists.get(&key).unwrap_or(&isize::MAX) {
                    dists.insert(key, next_cost);
                    heap.push((-next_cost, key));
                }
            }
        }
    }
    unreachable!()
}

fn parse_blocks(input: Vec<String>) -> BlockMap {
    input
        .into_iter()
        .map(|row| {
            row.chars()
                .map(|cost| cost.to_digit(10).expect("Should be a digit") as isize)
                .collect()
        })
        .collect()
}

fn part1(path: &str) -> Result<isize> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let blocks = parse_blocks(input);

    Ok(find_least_heat_loss(blocks, 1, 3))
}

fn part2(path: &str) -> Result<isize> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let blocks = parse_blocks(input);

    Ok(find_least_heat_loss(blocks, 4, 10))
}

fn main() {
    println!("Part1: {}", part1("data/17.input").unwrap());
    println!("Part2: {}", part2("data/17.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/17_1.sample").unwrap(), 102);
    }

    #[test_case("data/17_1.sample", 94; "Inital sample")]
    #[test_case("data/17_2.sample", 71; "Extra sample")]
    fn part2_test(path: &str, result: isize) {
        assert_eq!(part2(path).unwrap(), result);
    }
}
