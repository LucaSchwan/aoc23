use anyhow::Result;
use glam::{IVec2, IVec3, Vec3Swizzles};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    cubes: Vec<IVec3>,
}

#[derive(Debug, Clone)]
struct Supported {
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

fn brick_end(input: &str) -> IResult<&str, IVec3> {
    let (input, (x, _, y, _, z)) = tuple((
        complete::i32,
        tag(","),
        complete::i32,
        tag(","),
        complete::i32,
    ))(input)?;

    Ok((input, IVec3::new(x, y, z)))
}

fn brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start, end)) = separated_pair(brick_end, tag("~"), brick_end)(input)?;

    let cubes = [start.x..=end.x, start.y..=end.y, start.z..=end.z]
        .into_iter()
        .multi_cartesian_product()
        .map(|arr| IVec3::new(arr[0], arr[1], arr[2]))
        .collect();

    Ok((input, Brick { id: 0, cubes }))
}

fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    let (input, bricks) = separated_list1(newline, brick)(input)?;

    Ok((input, bricks))
}

fn part1(path: &str) -> Result<usize> {
    let input = aoc23::load_input(path)?;
    let (_, bricks) = parse_bricks(&input).expect("Should parse");
    let sorted_bricks = bricks
        .iter()
        .sorted_by(|a, b| {
            a.cubes
                .iter()
                .map(|cube| cube.z)
                .min()
                .unwrap()
                .cmp(&b.cubes.iter().map(|cube| cube.z).min().unwrap())
        })
        .collect_vec();

    let sorted_bricks = sorted_bricks
        .into_iter()
        .enumerate()
        .map(|(i, brick)| Brick {
            id: i,
            cubes: (*brick.cubes).to_vec(),
        })
        .collect_vec();

    let mut fallen_bricks: Vec<Brick> = vec![];

    let unsafe_bricks =
        sorted_bricks
            .into_iter()
            .fold(HashSet::<usize>::new(), |mut acc, brick| {
                let min_cubes = brick.cubes.iter().min_set_by_key(|cube| cube.z);
                let min_cubes_xy: Vec<IVec2> = min_cubes.iter().map(|cube| cube.xy()).collect();
                let touching_cubes = fallen_bricks
                    .iter()
                    .flat_map(|brick| brick.cubes.iter().map(|cube| (cube, brick.id)))
                    .filter_map(|(cube, brick_id)| {
                        min_cubes_xy
                            .contains(&cube.xy())
                            .then_some((cube.z, brick_id))
                    });

                let supporting_bricks = touching_cubes
                    .clone()
                    .max_set_by_key(|(z, _)| *z)
                    .into_iter()
                    .unique_by(|(_, id)| *id)
                    .map(|(_, id)| id)
                    .collect_vec();
                if supporting_bricks.len() <= 1 {
                    supporting_bricks.into_iter().for_each(|id| {
                        acc.insert(id);
                    });
                }

                let z = touching_cubes.map(|(z, _)| z).max().unwrap_or(0) + 1;

                let brick_z = min_cubes.first().unwrap().z;
                let diff = brick_z - z;
                let new_cubes = brick
                    .cubes
                    .iter()
                    .map(|cube| IVec3::new(cube.x, cube.y, cube.z - diff))
                    .collect();

                let new_brick = Brick {
                    id: brick.id,
                    cubes: new_cubes,
                };

                fallen_bricks.push(new_brick);
                acc
            });

    Ok(fallen_bricks.len() - unsafe_bricks.len())
}

fn part2(path: &str) -> Result<usize> {
    let input = aoc23::load_input(path)?;
    let (_, bricks) = parse_bricks(&input).expect("Should parse");
    let sorted_bricks = bricks
        .iter()
        .sorted_by(|a, b| {
            a.cubes
                .iter()
                .map(|cube| cube.z)
                .min()
                .unwrap()
                .cmp(&b.cubes.iter().map(|cube| cube.z).min().unwrap())
        })
        .collect_vec();

    let sorted_bricks = sorted_bricks
        .into_iter()
        .enumerate()
        .map(|(i, brick)| Brick {
            id: i,
            cubes: (*brick.cubes).to_vec(),
        })
        .collect_vec();

    let mut fallen_bricks: Vec<Brick> = vec![];
    let mut unsafe_bricks: HashSet<usize> = HashSet::new();

    let supports = sorted_bricks.clone().into_iter().fold(
        HashMap::<usize, Supported>::new(),
        |mut acc, brick| {
            let min_cubes = brick.cubes.iter().min_set_by_key(|cube| cube.z);
            let min_cubes_xy: Vec<IVec2> = min_cubes.iter().map(|cube| cube.xy()).collect();
            let touching_cubes = fallen_bricks
                .iter()
                .flat_map(|brick| brick.cubes.iter().map(|cube| (cube, brick.id)))
                .filter_map(|(cube, brick_id)| {
                    min_cubes_xy
                        .contains(&cube.xy())
                        .then_some((cube.z, brick_id))
                });

            let supporting_bricks = touching_cubes
                .clone()
                .max_set_by_key(|(z, _)| *z)
                .into_iter()
                .unique_by(|(_, id)| *id)
                .map(|(_, id)| id)
                .collect_vec();

            if supporting_bricks.len() <= 1 {
                supporting_bricks.clone().into_iter().for_each(|id| {
                    unsafe_bricks.insert(id);
                });
            }

            let supported = Supported {
                supported_by: supporting_bricks.clone(),
                supports: vec![],
            };

            acc.insert(brick.id, supported);

            supporting_bricks.into_iter().for_each(|id| {
                acc.entry(id)
                    .and_modify(|supported| supported.supports.push(brick.id));
            });

            let z = touching_cubes.map(|(z, _)| z).max().unwrap_or(0) + 1;

            let brick_z = min_cubes.first().unwrap().z;
            let diff = brick_z - z;
            let new_cubes = brick
                .cubes
                .iter()
                .map(|cube| IVec3::new(cube.x, cube.y, cube.z - diff))
                .collect();

            let new_brick = Brick {
                id: brick.id,
                cubes: new_cubes,
            };

            fallen_bricks.push(new_brick);
            acc
        },
    );

    let mut sum = 0;

    unsafe_bricks.into_iter().for_each(|id| {
        let mut ids = supports.get(&id).expect("Should exist").supports.clone();
        let mut falling_bricks = HashSet::new();

        while let Some(brick_id) = ids.pop() {
            if let Some(supported) = supports.get(&brick_id) {
                if supported
                    .supported_by
                    .iter()
                    .all(|support_id| falling_bricks.contains(support_id) || *support_id == id)
                {
                    falling_bricks.insert(brick_id);
                    ids.extend(supported.supports.clone());
                }
            }
        }

        sum += falling_bricks.len();
    });

    Ok(sum)
}

fn main() {
    println!("Part1: {}", part1("data/22.input").unwrap());
    println!("Part2: {}", part2("data/22.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/22.sample").unwrap(), 5);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/22.sample").unwrap(), 7);
    }
}
