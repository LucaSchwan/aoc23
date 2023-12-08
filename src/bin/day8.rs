use anyhow::{Error, Result};
use num::integer::lcm;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct NodeChildren {
    left: String,
    right: String,
}

fn parse_map(input: &str) -> Result<BTreeMap<String, NodeChildren>> {
    Ok(input.lines().fold(BTreeMap::new(), |mut map, line| {
        let (node, children) = line
            .split_once(" = ")
            .expect("Children should be well formed");
        let (left, right) = children
            .split_once(", ")
            .expect("Children should be well formed");
        map.entry(node.to_string()).or_insert(NodeChildren {
            left: left.replace('(', ""),
            right: right.replace(')', ""),
        });
        map
    }))
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;

    let (directions, map) = input
        .split_once("\n\n")
        .ok_or(Error::msg("Error parsing input"))?;

    let map = parse_map(map)?;

    let mut directions = directions.chars().cycle();
    let mut steps = 0;
    let mut current = "AAA".to_string();

    while current != "ZZZ" {
        steps += 1;
        let current_children = map.get(&current).expect("Should exist");
        let next_direction = directions.next().expect("Should exist");
        current = match next_direction {
            'L' => current_children.left.clone(),
            'R' => current_children.right.clone(),
            _ => panic!("Wrong direction"),
        };
    }

    Ok(steps)
}

fn part2(path: &str) -> Result<u64> {
    let input = aoc23::load_input(path)?;

    let (directions, map) = input
        .split_once("\n\n")
        .ok_or(Error::msg("Error parsing input"))?;

    let map = parse_map(map)?;

    let mut directions = directions.chars().cycle();

    let currents = map
        .clone()
        .into_keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<String>>();

    let steps = currents
        .clone()
        .into_iter()
        .map(|current| {
            let mut current = current;
            let mut steps = 0;
            while !current.ends_with("Z") {
                steps += 1;
                let current_children = map.get(&current).expect("Should exist");
                let next_direction = directions.next().expect("Should exist");
                current = match next_direction {
                    'L' => current_children.left.clone(),
                    'R' => current_children.right.clone(),
                    _ => panic!("Wrong direction"),
                };
            }
            steps
        })
        .collect::<Vec<u64>>();

    let steps = steps.iter().fold(1, |acc, steps| lcm(acc, *steps));

    Ok(steps)
}

fn main() {
    println!("Part 1: {}", part1("data/8.input").unwrap());
    println!("Part 2: {}", part2("data/8.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("data/8_1.sample", 2 ; "First Sample")]
    #[test_case("data/8_2.sample", 6 ; "Second Sample")]
    fn part1_test(path: &str, result: u32) {
        assert_eq!(part1(path).unwrap(), result);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/8_3.sample").unwrap(), 6);
    }
}
