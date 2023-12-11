use anyhow::{Error, Result};
use aoc23::Vec2D;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    EmptySpace,
    Galaxy,
}

impl FromStr for Space {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::EmptySpace),
            "#" => Ok(Self::Galaxy),
            _ => Err(Error::msg("Wrong type of space")),
        }
    }
}

fn parse_image(input: Vec<String>) -> Vec<Vec<Space>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<Space>()
                        .expect("Should be well formed")
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<Vec<Space>>>()
}

fn calc_expanded(image: &[Vec<Space>]) -> (Vec<i64>, Vec<i64>) {
    let rows = image
        .iter()
        .enumerate()
        .fold(vec![], |mut rows, (y, line)| {
            if !line.contains(&Space::Galaxy) {
                rows.push((y + 1) as i64);
            }
            rows
        });

    let columns = (0..image[0].len()).fold(vec![], |mut columns, x| {
        if !image.iter().any(|line| line[x] == Space::Galaxy) {
            columns.push((x + 1) as i64);
        }
        columns
    });

    (rows, columns)
}

fn find_galaxies(image: &[Vec<Space>]) -> HashSet<Vec2D> {
    image
        .iter()
        .enumerate()
        .fold(HashSet::new(), |mut galaxies, (y, line)| {
            line.iter().enumerate().for_each(|(x, space)| {
                if space == &Space::Galaxy {
                    galaxies.insert(Vec2D::new((x + 1) as i32, (y + 1) as i32));
                }
            });
            galaxies
        })
}

fn _print_image(image: &[Vec<Space>]) {
    image.iter().for_each(|line| {
        line.iter().for_each(|space| match space {
            Space::EmptySpace => print!("\u{2022}"),
            Space::Galaxy => print!("#"),
        });
        println!();
    })
}

fn solve(path: &str, expanded_by: i64) -> Result<i64> {
    let input = aoc23::read_one_per_line::<String>(path)?;
    let image = parse_image(input);
    let (expanded_rows, expanded_cols) = calc_expanded(&image);
    let galaxies = find_galaxies(&image);

    let mut already_checked = vec![];
    Ok(galaxies
        .clone()
        .into_iter()
        .map(|galaxy1| {
            let sum = galaxies
                .clone()
                .into_iter()
                .map(|galaxy2| {
                    if galaxy1 != galaxy2 && !already_checked.contains(&galaxy2) {
                        let y_range = if galaxy1.y < galaxy2.y {
                            galaxy1.y as i64..galaxy2.y as i64
                        } else {
                            galaxy2.y as i64..galaxy1.y as i64
                        };

                        let x_range = if galaxy1.x < galaxy2.x {
                            galaxy1.x as i64..galaxy2.x as i64
                        } else {
                            galaxy2.x as i64..galaxy1.x as i64
                        };

                        let contained_expanded_rows = expanded_rows
                            .iter()
                            .filter(|row| y_range.contains(row))
                            .count() as i64;
                        let contained_expanded_cols = expanded_cols
                            .iter()
                            .filter(|col| x_range.contains(col))
                            .count() as i64;

                        let x = (galaxy2.x - galaxy1.x).abs() as i64
                            + (contained_expanded_cols * expanded_by - contained_expanded_cols);
                        let y = (galaxy2.y - galaxy1.y).abs() as i64
                            + (contained_expanded_rows * expanded_by - contained_expanded_rows);
                        return x + y;
                    }
                    0
                })
                .sum::<i64>();
            already_checked.push(galaxy1);
            sum
        })
        .sum::<i64>())
}

fn part1(path: &str) -> Result<i64> {
    solve(path, 2)
}

fn part2(path: &str) -> Result<i64> {
    solve(path, 1_000_000)
}

fn main() {
    println!("Part1: {}", part1("data/11.input").unwrap());
    println!("Part2: {}", part2("data/11.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(2, 374 ; "Once expanded(faktor x2)")]
    #[test_case(10, 1030 ; "10 times expanded")]
    #[test_case(100, 8410 ; "100 times expanded")]
    fn solve_test(times_expanded: i64, result: i64) {
        assert_eq!(solve("data/11.sample", times_expanded).unwrap(), result);
    }
}
