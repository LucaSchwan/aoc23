use std::str::FromStr;
use std::vec::Vec;

use anyhow::{Error, Result};

#[derive(Debug, Clone)]
struct ValueHistory {
    history: Vec<i32>,
}

impl FromStr for ValueHistory {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(ValueHistory {
            history: s
                .split(" ")
                .map(|value| value.parse::<i32>().expect("Should be a number"))
                .collect(),
        })
    }
}

impl FromIterator<i32> for ValueHistory {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        ValueHistory {
            history: Vec::from_iter(iter),
        }
    }
}

fn build_tree(history: ValueHistory) -> Vec<ValueHistory> {
    let mut values_tree = vec![history];

    while !values_tree
        .last()
        .expect("Last should exist")
        .history
        .iter()
        .all(|value| *value == 0)
    {
        let new_values = values_tree
            .clone()
            .iter()
            .last()
            .expect("Should have a last")
            .history
            .as_slice()
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        values_tree.push(new_values);
    }

    values_tree
}

fn calc_extrapolate_forwards(history: ValueHistory) -> i32 {
    let mut values_tree = build_tree(history);

    values_tree
        .iter_mut()
        .for_each(|values| values.history.reverse());
    values_tree.reverse();

    values_tree.iter().skip(1).fold(0, |value, history| {
        value + *history.history.first().expect("Should have first")
    })
}

fn calc_extrapolate_backwards(history: ValueHistory) -> i32 {
    let mut values_tree = build_tree(history);

    values_tree.reverse();

    values_tree.iter().skip(1).fold(0, |value, values| {
        *values.history.first().expect("Should have first") - value
    })
}

fn part1(path: &str) -> Result<i32> {
    let histories = aoc23::read_one_per_line::<ValueHistory>(path)?;

    Ok(histories
        .into_iter()
        .map(|history| calc_extrapolate_forwards(history))
        .sum())
}

fn part2(path: &str) -> Result<i32> {
    let histories = aoc23::read_one_per_line::<ValueHistory>(path)?;

    Ok(histories
        .into_iter()
        .map(|history| calc_extrapolate_backwards(history))
        .sum())
}

fn main() {
    println!("Part 1: {}", part1("data/9.input").unwrap());
    println!("Part 2: {}", part2("data/9.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/9.sample").unwrap(), 114);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2("data/9.sample").unwrap(), 2);
    }
}
