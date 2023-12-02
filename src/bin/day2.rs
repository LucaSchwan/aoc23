use anyhow::Result;
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut split = s.split(':');

        let id: u32 = split
            .next()
            .expect("Should be well formed")
            .split(' ')
            .nth(1)
            .expect("Should be Id")
            .parse()
            .expect("should be number");

        let mut game = Game {
            id,
            red: 0,
            green: 0,
            blue: 0,
        };

        split
            .next()
            .expect("Should be Well formed")
            .split(&[';', ','])
            .for_each(|color| {
                let mut color = color.trim_start().split(' ');
                let num = color
                    .next()
                    .expect("Color number should exist")
                    .parse::<u32>()
                    .expect("Should be a number");
                let color_name = color.next().expect("Color name should exist");

                match color_name {
                    "red" => {
                        if game.red < num {
                            game.red = num;
                        }
                    }
                    "green" => {
                        if game.green < num {
                            game.green = num;
                        }
                    }
                    "blue" => {
                        if game.blue < num {
                            game.blue = num;
                        }
                    }
                    _ => panic!("Undefined color"),
                }
            });

        Ok(game)
    }
}

fn part1(path: &str) -> Result<u32> {
    Ok(aoc23::read_one_per_line::<Game>(path)?
        .into_iter()
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|game| game.id)
        .sum())
}

fn part2(path: &str) -> Result<u32> {
    Ok(aoc23::read_one_per_line::<Game>(path)?
        .into_iter()
        .map(|game| game.red * game.green * game.blue)
        .sum())
}

fn main() {
    println!("Part1: {}", part1("data/2.input").unwrap());
    println!("Part2: {}", part2("data/2.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/2.sample").unwrap(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/2.sample").unwrap(), 2286);
    }
}
