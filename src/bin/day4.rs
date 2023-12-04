use anyhow::Result;
use std::str::FromStr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn calc_score(&self) -> u32 {
        let matches = self.matches();

        // 1 Point for the first match, double for every other
        // so all over one are 2^(match - 1)
        match matches {
            0 => 0,
            1 => 1,
            x => u32::pow(2, x - 1),
        }
    }

    fn matches(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count() as u32
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut card_numbers = s
            .split(':')
            .nth(1)
            .expect("Should be well formed")
            .split('|');

        let winning_numbers: Vec<u32> = card_numbers
            .next()
            .expect("Should be well formed")
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().expect("Should be a number"))
            .collect();

        let numbers: Vec<u32> = card_numbers
            .next()
            .expect("Should be well formed")
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().expect("Should be a number"))
            .collect();

        Ok(Card {
            winning_numbers,
            numbers,
        })
    }
}

fn part1(path: &str) -> Result<u32> {
    Ok(aoc23::read_one_per_line::<Card>(path)?
        .into_iter()
        .map(|card| card.calc_score())
        .sum::<u32>())
}

fn part2(path: &str) -> Result<u32> {
    let cards = aoc23::read_one_per_line::<Card>(path)?
        .into_iter()
        .enumerate();

    let mut total_cards = cards.clone().map(|_| 1).collect::<Vec<u32>>();

    cards.clone().for_each(|(i, card)| {
        let matches = card.matches();
        let card_total = total_cards[i];
        let next_index = i + 1;

        (next_index..usize::min(cards.len(), next_index + (matches as usize))).for_each(|index| {
            total_cards[index] += card_total;
        });
    });

    Ok(total_cards.into_iter().sum::<u32>())
}

fn main() {
    println!("Part 1: {}", part1("data/4.input").unwrap());
    println!("Part 2: {}", part2("data/4.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/4.sample").unwrap(), 13);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/4.sample").unwrap(), 30);
    }
}
