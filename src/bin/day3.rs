use anyhow::Result;
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug)]
enum Field {
    Number(u32),
    Symbol(char),
    Empty,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_schematic(lines: Vec<String>) -> BTreeMap<Position, Field> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().map(move |(y, c)| {
                (
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    match c {
                        '.' => Field::Empty,
                        c if c.is_ascii_digit() => {
                            Field::Number(c.to_digit(10).expect("Should be a digit"))
                        }
                        c => Field::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<Position, Field>>()
}

fn calc_numbers(schematic: &BTreeMap<Position, Field>) -> Vec<Vec<(Position, u32)>> {
    schematic
        .iter()
        .fold(vec![], |mut numbers, (position, field)| {
            if let Field::Number(num) = field {
                match numbers.iter().last() {
                    Some(v) => match v.iter().last() {
                        Some((last_num, _)) => {
                            if last_num.y + 1 == position.y {
                                numbers
                                    .iter_mut()
                                    .last()
                                    .expect("Should exist")
                                    .push(((position.clone()), *num))
                            } else {
                                numbers.push(vec![((position.clone()), *num)])
                            }
                        }
                        None => unimplemented!("Shouldn't happen"),
                    },
                    None => numbers.push(vec![((position.clone()), *num)]),
                }
            }
            numbers
        })
}

const POSITIONS: [Position; 8] = [
    Position { x: 1, y: 0 },
    Position { x: 1, y: -1 },
    Position { x: 0, y: -1 },
    Position { x: -1, y: -1 },
    Position { x: -1, y: 0 },
    Position { x: -1, y: 1 },
    Position { x: 0, y: 1 },
    Position { x: 1, y: 1 },
];

fn get_positions_to_check(list: &[(Position, u32)]) -> Vec<Position> {
    let num_positions: Vec<&Position> = list.iter().map(|(position, _)| position).collect();
    list.iter()
        .flat_map(|(pos, _)| get_position_to_check(pos))
        .unique()
        .filter(|num| !num_positions.contains(&num))
        .collect()
}

fn get_position_to_check(position: &Position) -> Vec<Position> {
    POSITIONS
        .iter()
        .map(|outer_pos| Position {
            x: outer_pos.x + position.x,
            y: outer_pos.y + position.y,
        })
        .collect()
}

fn part1(path: &str) -> Result<u32> {
    let schematic = parse_schematic(aoc23::read_one_per_line::<String>(path)?);
    let numbers = calc_numbers(&schematic);

    let parts_sum = numbers
        .into_iter()
        .filter_map(|num_list| {
            let pos_to_check = get_positions_to_check(&num_list);

            let is_part_number = pos_to_check
                .iter()
                .any(|pos| matches!(schematic.get(pos), Some(Field::Symbol(_))));

            if is_part_number {
                Some(
                    num_list
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .sum::<u32>();

    Ok(parts_sum)
}

fn part2(path: &str) -> Result<usize> {
    let schematic = parse_schematic(aoc23::read_one_per_line::<String>(path)?);
    let numbers = calc_numbers(&schematic);

    let gear_ratios = schematic
        .iter()
        .filter(|(_, field)| matches!(field, Field::Symbol('*')))
        .filter_map(|(position, _)| {
            let pos_to_check = get_position_to_check(position);

            let number_indexes = pos_to_check
                .iter()
                .fold(vec![], |mut number_indexes, pos| {
                    numbers.iter().enumerate().for_each(|(i, num_list)| {
                        if num_list.iter().any(|num_pos| num_pos.0 == *pos) {
                            number_indexes.push(i);
                        }
                    });
                    number_indexes
                })
                .into_iter()
                .unique()
                .collect_vec();

            if number_indexes.len() == 2 {
                Some(
                    number_indexes
                        .iter()
                        .map(|index| {
                            numbers[*index]
                                .iter()
                                .map(|(_, num)| num.to_string())
                                .collect::<String>()
                                .parse::<usize>()
                                .expect("Should be a number")
                        })
                        .product::<usize>(),
                )
            } else {
                None
            }
        })
        .sum::<usize>();

    Ok(gear_ratios)
}

fn main() {
    println!("Part 1: {}", part1("data/3.input").unwrap());
    println!("Part 2: {}", part2("data/3.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/3.sample").unwrap(), 4361);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/3.sample").unwrap(), 467835);
    }
}
