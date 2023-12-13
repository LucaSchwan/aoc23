use anyhow::Result;
use itertools::Itertools;
use std::ops::Range;

fn parse_patterns(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.to_string()).collect_vec())
        .collect_vec()
}

fn find_mirror_lines(pattern: &[String]) -> Vec<usize> {
    pattern.iter().enumerate().tuple_windows().fold(
        vec![],
        |mut mirror_lines, ((l_i, left), (_, right))| {
            if left == right {
                mirror_lines.push(l_i);
            }
            mirror_lines
        },
    )
}

fn get_range(pattern: &Vec<String>, index: &usize) -> (Range<usize>, usize, usize) {
    let len = pattern.len();
    let range_to_check = if *index >= len / 2 {
        1..(len - index) - 1
    } else {
        1..index + 1
    };

    (range_to_check, index + 1, *index)
}

fn index_is_mirror(index: &usize, pattern: &Vec<String>) -> bool {
    let (mut range_to_check, upper_index, lower_index) = get_range(pattern, index);

    range_to_check.all(|i| pattern[upper_index + i] == pattern[lower_index - i])
}

fn pattern_score(pattern: Vec<String>) -> usize {
    let mut score = None;

    let mirror_lines = find_mirror_lines(&pattern);

    mirror_lines.iter().for_each(|line| {
        if index_is_mirror(line, &pattern) {
            score = Some((line + 1) * 100);
        }
    });

    if let Some(score) = score {
        return score;
    }

    let vertical_pattern = (0..pattern[0].len())
        .map(|i| {
            pattern
                .clone()
                .into_iter()
                .map(|line| line.chars().nth(i).expect("Should exist"))
                .collect::<String>()
        })
        .collect_vec();

    let mirror_lines = find_mirror_lines(&vertical_pattern);

    mirror_lines.iter().for_each(|line| {
        if index_is_mirror(line, &vertical_pattern) {
            score = Some(line + 1);
        }
    });

    if let Some(score) = score {
        return score;
    }

    0
}

fn num_differences(left: String, right: String) -> u32 {
    left.chars()
        .zip(right.chars())
        .filter(|(left, right)| left != right)
        .count() as u32
}

fn mirror_differences(pattern: Vec<String>, index: &usize) -> u32 {
    let (range_to_check, upper_index, lower_index) = get_range(&pattern, index);

    range_to_check
        .map(|i| {
            num_differences(
                pattern[lower_index - i].clone(),
                pattern[upper_index + i].clone(),
            )
        })
        .sum()
}

fn find_mirror_lines_with_smudge(pattern: &[String]) -> Vec<usize> {
    pattern.iter().enumerate().tuple_windows().fold(
        vec![],
        |mut mirror_lines, ((l_i, left), (_, right))| {
            if num_differences(left.to_owned(), right.to_owned()) == 1 {
                mirror_lines.push(l_i);
            }
            mirror_lines
        },
    )
}

fn score_with_smudge(pattern: Vec<String>) -> usize {
    let mirror_lines_with_smudge = find_mirror_lines_with_smudge(&pattern);
    let mirror_lines = find_mirror_lines(&pattern);

    let mut smudged_lines = mirror_lines
        .iter()
        .map(|line| (line, mirror_differences(pattern.clone(), line)))
        .filter(|(_, differences)| *differences == 1)
        .map(|(line, _)| line)
        .collect_vec();

    mirror_lines_with_smudge.iter().for_each(|line| {
        if index_is_mirror(line, &pattern) {
            smudged_lines.push(line);
        }
    });

    if smudged_lines.len() == 1 {
        return (smudged_lines[0] + 1) * 100;
    }

    let vertical_pattern = (0..pattern[0].len())
        .map(|i| {
            pattern
                .clone()
                .into_iter()
                .map(|line| line.chars().nth(i).expect("Should exist"))
                .collect::<String>()
        })
        .collect_vec();

    let mirror_lines_with_smudge = find_mirror_lines_with_smudge(&vertical_pattern);
    let mirror_lines = find_mirror_lines(&vertical_pattern);

    let mut smudged_lines = mirror_lines
        .iter()
        .map(|line| (line, mirror_differences(vertical_pattern.clone(), line)))
        .filter(|(_, differences)| *differences == 1)
        .map(|(line, _)| line)
        .collect_vec();

    mirror_lines_with_smudge.iter().for_each(|line| {
        if index_is_mirror(line, &vertical_pattern) {
            smudged_lines.push(line);
        }
    });

    if smudged_lines.len() == 1 {
        return smudged_lines[0] + 1;
    }

    0
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;
    let patterns = parse_patterns(&input);

    Ok(patterns.into_iter().map(pattern_score).sum::<usize>() as u32)
}

fn part2(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;
    let patterns = parse_patterns(&input);

    Ok(patterns.into_iter().map(score_with_smudge).sum::<usize>() as u32)
}

fn main() {
    println!("Part1: {}", part1("data/13.input").unwrap());
    println!("Part2: {}", part2("data/13.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/13.sample").unwrap(), 405);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/13.sample").unwrap(), 400);
    }
}
