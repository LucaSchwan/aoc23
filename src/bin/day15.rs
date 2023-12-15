use std::collections::BTreeMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    operation: Operation,
}

#[derive(Debug)]
enum Operation {
    Add(u32),
    Remove,
}

fn hash(text: &str) -> u32 {
    text.chars().fold(0, |mut curr, c| {
        curr += c as u32;
        curr *= 17;
        curr %= 256;
        curr
    })
}

fn lens(input: &str) -> IResult<&str, Lens> {
    let (input, lens) = tuple((
        alpha1,
        alt((
            map(tuple((tag("="), complete::u32)), |(_, number)| {
                Operation::Add(number)
            }),
            map(tag("-"), |_| Operation::Remove),
        )),
    ))
    .map(|(label, operation)| Lens { label, operation })
    .parse(input)?;

    Ok((input, lens))
}

fn parse_lenses(input: &str) -> IResult<&str, Vec<Lens>> {
    let (input, lenses) = separated_list1(tag(","), lens)(input)?;
    let (input, _) = tag(",")(input)?;

    debug_assert!(input.is_empty());

    Ok((input, lenses))
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;

    Ok(input.split([',', '\n']).map(hash).sum::<u32>())
}

fn arrange_lenses(lenses: Vec<Lens>) -> BTreeMap<u32, Vec<(&str, u32)>> {
    lenses
        .into_iter()
        .fold(BTreeMap::new(), |mut boxes, Lens { label, operation }| {
            let box_num = hash(label);
            match operation {
                Operation::Add(focal_length) => {
                    boxes
                        .entry(box_num)
                        .and_modify(|box_lenses| {
                            if let Some(index) = box_lenses
                                .iter()
                                .position(|(inner_label, _)| *inner_label == label)
                            {
                                box_lenses[index] = (label, focal_length);
                            } else {
                                box_lenses.push((label, focal_length));
                            }
                        })
                        .or_insert(vec![(label, focal_length)]);
                }
                Operation::Remove => {
                    boxes.entry(box_num).and_modify(|box_lenses| {
                        if let Some(index) = box_lenses
                            .iter()
                            .position(|(inner_label, _)| *inner_label == label)
                        {
                            box_lenses.remove(index);
                        }
                    });
                }
            };
            boxes
        })
}

fn part2(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;
    let input = input.replace('\n', ",");

    let (_, lenses) = parse_lenses(input.as_str()).expect("Should parse");
    let boxes = arrange_lenses(lenses);

    Ok(boxes
        .into_iter()
        .flat_map(|(box_num, box_lenses)| {
            box_lenses
                .into_iter()
                .enumerate()
                .map(move |(index, (_, focal_length))| {
                    (1 + box_num) * (index as u32 + 1) * focal_length
                })
        })
        .sum())
}

fn main() {
    println!("Part1: {}", part1("data/15.input").unwrap());
    println!("Part2: {}", part2("data/15.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("rn=1", 30 ; "1")]
    #[test_case("cm-", 253 ; "2")]
    #[test_case("qp=3", 97 ; "3")]
    #[test_case("cm=2", 47 ; "4")]
    #[test_case("qp-", 14 ; "5")]
    #[test_case("pc=4", 180 ; "6")]
    #[test_case("ot=9", 9 ; "7")]
    #[test_case("ab=5", 197 ; "8")]
    #[test_case("pc-", 48 ; "9")]
    #[test_case("pc=6", 214 ; "10")]
    #[test_case("ot=7", 231 ; "11")]
    fn hash_test(text: &str, hash_value: u32) {
        assert_eq!(hash(text), hash_value);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/15.sample").unwrap(), 1320);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/15.sample").unwrap(), 145);
    }
}
