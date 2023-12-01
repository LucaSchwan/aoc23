use anyhow::Result;

fn sum_nums(nums: Vec<String>) -> Result<u32> {
    Ok(nums
        .into_iter()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect())
        .map(|line: String| {
            format!(
                "{}{}",
                line.clone().chars().next().unwrap(),
                line.clone().chars().last().unwrap()
            )
            .parse::<u32>()
        })
        .collect::<Result<Vec<u32>, _>>()?
        .into_iter()
        .sum())
}

fn part1(path: &str) -> Result<u32> {
    sum_nums(aoc23::read_one_per_line::<String>(path)?)
}

fn part2(path: &str) -> Result<u32> {
    let num_words = vec![
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];

    sum_nums(
        aoc23::read_one_per_line::<String>(path)?
            .into_iter()
            .map(|line| {
                let mut new_line = line.clone();
                num_words.clone().into_iter().for_each(|num| {
                    new_line = new_line.replace(num.0, num.1);
                });
                new_line
            })
            .collect(),
    )
}

fn main() {
    println!("Part 1: {}", part1("data/1.input").unwrap());
    println!("Part 2: {}", part2("data/1.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/1_1.sample").unwrap(), 142);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/1_2.sample").unwrap(), 281);
    }
}
