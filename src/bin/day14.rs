use std::{collections::BTreeMap, fmt::Display};

use anyhow::Result;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rock {
    Round,
    Cube,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Platform {
    platform: Vec<Vec<Rock>>,
}

impl Platform {
    fn spin(self) -> Self {
        let mut platform_cols = self
            .platform
            .into_iter()
            .map(|mut col| {
                col.reverse();
                col.into_iter()
            })
            .collect_vec();
        let new_platform = std::iter::from_fn(move || {
            let mut new_col = vec![];
            for col in &mut platform_cols {
                match col.next() {
                    Some(rock) => {
                        new_col.push(rock);
                    }
                    None => return None,
                }
            }
            Some(new_col)
        })
        .collect_vec();

        Self {
            platform: new_platform,
        }
    }

    fn tilt(self) -> Self {
        let platform = self
            .platform
            .into_iter()
            .map(|col| {
                col.into_iter()
                    .fold(vec![(vec![], vec![], vec![])], |mut acc, rock| {
                        let (_, ref mut round_rocks, ref mut none_rocks) =
                            acc.last_mut().expect("Should have last");
                        match rock {
                            Rock::Round => round_rocks.push(rock),
                            Rock::Cube => acc.push((vec![rock], vec![], vec![])),
                            Rock::None => none_rocks.push(rock),
                        };
                        acc
                    })
                    .into_iter()
                    .flat_map(|(mut cube_rocks, mut round_rocks, mut none_rocks)| {
                        cube_rocks.append(&mut round_rocks);
                        cube_rocks.append(&mut none_rocks);
                        cube_rocks
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self { platform }
    }

    fn cycle(mut self) -> Self {
        for _ in 0..4 {
            self = self.tilt().spin();
        }
        self
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.platform[0].len())
                .map(|index| {
                    self.platform
                        .iter()
                        .map(|col| match col[index] {
                            Rock::Round => "O",
                            Rock::Cube => "#",
                            Rock::None => ".",
                        })
                        .collect_vec()
                        .join("")
                })
                .join("\n")
        )
    }
}

fn parse_platform(input: Vec<String>) -> Platform {
    let mut platform_rows = input.iter().map(|line| line.chars()).collect_vec();
    let platform = std::iter::from_fn(move || {
        let mut col = vec![];
        for iter in &mut platform_rows {
            match iter.next() {
                Some(rock) => {
                    let rock = match rock {
                        'O' => Rock::Round,
                        '#' => Rock::Cube,
                        '.' => Rock::None,
                        _ => panic!("Not a rock"),
                    };
                    col.push(rock);
                }
                None => return None,
            }
        }
        Some(col)
    })
    .collect_vec();

    Platform { platform }
}

fn calc_load(platform: Platform) -> u32 {
    let num_rows = platform.platform[0].len();

    platform
        .platform
        .iter()
        .map(|col| {
            col.iter()
                .enumerate()
                .fold(0, |load, (index, rock)| match rock {
                    Rock::Round => load + (num_rows - index) as u32,
                    _ => load,
                })
        })
        .sum::<u32>()
}

fn find_cycle(mut platform: Platform) -> (Platform, u32, u32) {
    let mut num_cycles = 0;
    let mut cache = BTreeMap::new();
    loop {
        platform = platform.cycle();

        num_cycles += 1;
        if let Some(prev) = cache.insert(platform.clone(), num_cycles) {
            return (platform, prev, num_cycles - prev);
        }
    }
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let platform = parse_platform(input);

    Ok(calc_load(platform.tilt()))
}

fn part2(path: &str) -> Result<u32> {
    let input = aoc23::read_one_per_line::<String>(path)?;

    let platform = parse_platform(input);

    let (mut platform, cycle_start, cycle_len) = find_cycle(platform.clone());

    let left_cycles = (1_000_000_000 - cycle_start) % cycle_len;

    for _ in 0..left_cycles {
        platform = platform.cycle();
    }

    Ok(calc_load(platform))
}

fn main() {
    println!("Part1: {}", part1("data/14.input").unwrap());
    println!("Part2: {}", part2("data/14.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/14.sample").unwrap(), 136);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/14.sample").unwrap(), 64);
    }
}
