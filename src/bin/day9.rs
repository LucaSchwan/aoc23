use anyhow::Result;
use itertools::Itertools;
use std::vec::Vec;

fn build_tree(history: Vec<i32>) -> Vec<Vec<i32>> {
    let mut values_tree = vec![history];

    while !values_tree
        .last()
        .expect("Last should exist")
        .iter()
        .all(|value| *value == 0)
    {
        let new_values = values_tree
            .iter()
            .last()
            .expect("Should have a last")
            .iter()
            .tuple_windows()
            .map(|(left, right)| right - left)
            .collect();

        values_tree.push(new_values);
    }

    values_tree
}

fn calc_extrapolate_forwards(history: Vec<i32>) -> i32 {
    let mut values_tree = build_tree(history);

    values_tree.iter_mut().for_each(|values| values.reverse());
    values_tree.reverse();

    values_tree.into_iter().skip(1).fold(0, |value, history| {
        value + history.first().expect("Should have first")
    })
}

fn calc_extrapolate_backwards(history: Vec<i32>) -> i32 {
    let mut values_tree = build_tree(history);

    values_tree.reverse();

    values_tree.into_iter().skip(1).fold(0, |value, values| {
        values.first().expect("Should have first") - value
    })
}

fn part1(path: &str) -> Result<i32> {
    let histories = aoc23::read_lines_of_num::<i32>(path)?;

    Ok(histories
        .into_iter()
        .map(|history| calc_extrapolate_forwards(history))
        .sum())
}

fn part2(path: &str) -> Result<i32> {
    let histories = aoc23::read_lines_of_num::<i32>(path)?;

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
