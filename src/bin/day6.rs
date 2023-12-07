use anyhow::Result;
use nom::{
    character::complete,
    character::complete::{newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, races) = separated_pair(
        tag("Time:")
            .precedes(space1)
            .precedes(separated_list1(space1, complete::u64)),
        newline,
        tag("Distance:")
            .precedes(space1)
            .precedes(separated_list1(space1, complete::u64)),
    )
    .map(|(times, record_distances)| {
        zip(times, record_distances)
            .map(|(time, record_distance)| Race {
                time,
                record_distance,
            })
            .collect::<Vec<Race>>()
    })
    .parse(input)?;

    Ok((input, races))
}

fn parse_single_race(input: &str) -> IResult<&str, Race> {
    let (input, race) = separated_pair(
        tag("Time:")
            .precedes(space1)
            .precedes(separated_list1(space1, complete::u64)),
        newline,
        tag("Distance:")
            .precedes(space1)
            .precedes(separated_list1(space1, complete::u64)),
    )
    .map(|(time_digits, distance_digits)| {
        let time = time_digits
            .iter()
            .map(|digit| digit.to_string())
            .collect::<String>()
            .parse::<u64>()
            .expect("Should be a u64");
        let record_distance = distance_digits
            .iter()
            .map(|digit| digit.to_string())
            .collect::<String>()
            .parse::<u64>()
            .expect("Should be a u64");

        Race {
            time,
            record_distance,
        }
    })
    .parse(input)?;

    Ok((input, race))
}

fn part1(path: &str) -> Result<u64> {
    let input = aoc23::load_input(path)?;

    let (_, races) = parse_races(input.as_str()).expect("Should be well formed");

    let result = races
        .iter()
        .map(|race| {
            (1..(race.time - 1))
                .filter_map(|accelerated_time| {
                    let distance = (race.time - accelerated_time) * accelerated_time;

                    if distance > race.record_distance {
                        Some(distance)
                    } else {
                        None
                    }
                })
                .count() as u64
        })
        .product::<u64>();

    Ok(result)
}

fn part2(path: &str) -> Result<u64> {
    let input = aoc23::load_input(path)?;

    let (_, race) = parse_single_race(input.as_str()).expect("Should be well formed");
    let ways_to_win = (1..(race.time - 1))
        .filter_map(|accelerated_time| {
            let distance = (race.time - accelerated_time) * accelerated_time;

            if distance > race.record_distance {
                Some(distance)
            } else {
                None
            }
        })
        .count() as u64;

    Ok(ways_to_win)
}

fn main() {
    println!("Part 1: {}", part1("data/6.input").unwrap());
    println!("Part 2: {}", part2("data/6.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/6.sample").unwrap(), 288)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/6.sample").unwrap(), 71503)
    }
}
