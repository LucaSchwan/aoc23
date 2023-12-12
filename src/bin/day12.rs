use anyhow::Result;
use cached::proc_macro::cached;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<Spring>,
    parity: Vec<u32>,
}

fn parse_springs(input: Vec<String>, repeat: usize) -> Vec<SpringRow> {
    input
        .into_iter()
        .map(|line| {
            let (springs, parity) = line.split_once(' ').expect("to split by space");
            let springs = &std::iter::repeat(springs).take(repeat).join("?");
            let parity = &std::iter::repeat(parity).take(repeat).join(",");
            let springs = springs
                .chars()
                .map(|spring| match spring {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    spring => panic!("Not a valid spring type: `{}`", spring),
                })
                .collect_vec();
            let parity = parity
                .split(',')
                .map(|num| num.parse::<u32>().expect("to parse as number"))
                .collect_vec();
            SpringRow { springs, parity }
        })
        .collect_vec()
}

#[cached]
fn solve_row(springs: Vec<Spring>, parity: Vec<u32>, num_done_in_group: u32) -> u64 {
    if springs.is_empty() {
        if parity.is_empty() && num_done_in_group == 0 {
            return 1;
        }
        return 0;
    }

    let mut ways = 0;

    let possibles = if springs[0] == Spring::Unknown {
        vec![Spring::Operational, Spring::Damaged]
    } else {
        vec![springs[0]]
    };

    possibles.into_iter().for_each(|possible| {
        if possible == Spring::Damaged {
            ways += solve_row(springs[1..].to_vec(), parity.clone(), num_done_in_group + 1)
        } else if num_done_in_group > 0 {
            if !parity.is_empty() && parity[0] == num_done_in_group {
                ways += solve_row(springs[1..].to_vec(), parity[1..].to_vec(), 0)
            }
        } else {
            ways += solve_row(springs[1..].to_vec(), parity.clone(), 0)
        }
    });

    ways
}

fn part1(path: &str) -> Result<u64> {
    let spring_rows = aoc23::read_one_per_line::<String>(path)?;
    let spring_rows = parse_springs(spring_rows, 1);

    Ok(spring_rows
        .into_iter()
        .map(|mut row| {
            row.springs.push(Spring::Operational);
            solve_row(row.springs, row.parity, 0)
        })
        .sum::<u64>())
}

fn part2(path: &str) -> Result<u64> {
    let spring_rows = aoc23::read_one_per_line::<String>(path)?;
    let spring_rows = parse_springs(spring_rows, 5);

    Ok(spring_rows
        .into_iter()
        .map(|mut row| {
            row.springs.push(Spring::Operational);
            solve_row(row.springs, row.parity, 0)
        })
        .sum::<u64>())
}

fn main() {
    println!("Part1: {}", part1("data/12.input").unwrap());
    println!("Part1: {}", part2("data/12.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/12.sample").unwrap(), 21);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/12.sample").unwrap(), 525152);
    }
}
