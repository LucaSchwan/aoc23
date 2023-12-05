use anyhow::{Context, Result};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, newline, space0, space1},
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;
use std::ops::Range;

#[derive(Clone)]
struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

#[derive(Clone)]
struct RangeAlmanac {
    seeds: Vec<Range<i64>>,
    maps: Vec<Map>,
}

#[derive(Clone)]
struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Clone)]
struct MapRange {
    source_range: Range<i64>,
    destination_range: Range<i64>,
}

impl Map {
    fn translate(&self, source: i64) -> i64 {
        let valid_mapping = self
            .ranges
            .iter()
            .find(|MapRange { source_range, .. }| source_range.contains(&source));

        let Some(MapRange {
            source_range,
            destination_range,
        }) = valid_mapping
        else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }

    fn rev_translate(&self, source: i64) -> i64 {
        let valid_mapping = self.ranges.iter().find(
            |MapRange {
                 destination_range, ..
             }| destination_range.contains(&source),
        );

        let Some(MapRange {
            source_range,
            destination_range,
        }) = valid_mapping
        else {
            return source;
        };

        let offset = source - destination_range.start;

        source_range.start + offset
    }
}

fn range(input: &str) -> IResult<&str, MapRange, ErrorTree<&str>> {
    let (input, (destination_start, _, source_start, _, count)) =
        tuple((complete::i64, space1, complete::i64, space1, complete::i64))(input)?;

    Ok((
        input,
        MapRange {
            source_range: source_start..(source_start + count),
            destination_range: destination_start..(destination_start + count),
        },
    ))
}

fn map(input: &str) -> IResult<&str, Vec<MapRange>, ErrorTree<&str>> {
    let (input, _) = pair(take_until("\n"), newline)(input)?;
    let (input, map) = separated_list1(newline, range).parse(input)?;

    Ok((input, map))
}

fn maps(input: &str) -> IResult<&str, Vec<Map>, ErrorTree<&str>> {
    let (input, maps) = separated_list1(pair(newline, newline), map)(input)?;

    Ok((
        input,
        maps.into_iter().map(|ranges| Map { ranges }).collect(),
    ))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac, ErrorTree<&str>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = many1(terminated(complete::i64, space0))(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    let (input, maps) = maps(input)?;

    Ok((input, Almanac { seeds, maps }))
}

fn seed_ranges(input: &str) -> IResult<&str, Vec<Range<i64>>, ErrorTree<&str>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seed_ranges) =
        separated_list1(space1, separated_pair(complete::i64, space1, complete::i64))
            .map(|ranges| {
                ranges
                    .into_iter()
                    .map(|(start, count)| start..(start + count))
                    .collect::<Vec<Range<i64>>>()
            })
            .parse(input)?;

    Ok((input, seed_ranges))
}

fn parse_range_almanac(input: &str) -> IResult<&str, RangeAlmanac, ErrorTree<&str>> {
    let (input, seeds) = seed_ranges(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    let (input, maps) = maps(input)?;

    Ok((input, RangeAlmanac { seeds, maps }))
}

fn part1(path: &str) -> Result<i64> {
    let input = aoc23::load_input(path)?;

    let (_, almanac) = parse_almanac(input.as_str()).expect("Should be well formed");

    let location = almanac
        .clone()
        .seeds
        .into_iter()
        .map(|seed| {
            almanac
                .maps
                .iter()
                .fold(seed, |num, map| map.translate(num))
        })
        .min();

    location.context("No min")
}

fn part2(path: &str) -> Result<i64> {
    let input = aoc23::load_input(path)?;

    let (_, range_almanac) = parse_range_almanac(input.as_str()).expect("Should be well formed");

    let mut location = 0;

    loop {
        let seed = range_almanac
            .maps
            .iter()
            .rev()
            .fold(location, |num, map| map.rev_translate(num));

        if range_almanac
            .seeds
            .iter()
            .any(|seed_range| seed_range.contains(&seed))
        {
            return Ok(location);
        }

        location += 1;
    }
}

fn main() {
    println!("Part 1: {}", part1("data/5.input").unwrap());
    println!("Part 2: {}", part2("data/5.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/5.sample").unwrap(), 35);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/5.sample").unwrap(), 46);
    }
}
