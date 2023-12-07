use anyhow::Result;
use nom::{
    character::complete,
    character::complete::{newline, one_of, space0},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::error::ErrorTree;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    value: HandValue,
    bid: u32,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct HandWithJoker {
    value: HandValueWithJoker,
    bid: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandValue {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandValueWithJoker {
    HighCard([CardWithJoker; 5]),
    OnePair([CardWithJoker; 5]),
    TwoPair([CardWithJoker; 5]),
    ThreeOfAKind([CardWithJoker; 5]),
    FullHouse([CardWithJoker; 5]),
    FourOfAKind([CardWithJoker; 5]),
    FiveOfAKind([CardWithJoker; 5]),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum CardWithJoker {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

fn hand_value(input: &str) -> IResult<&str, HandValue, ErrorTree<&str>> {
    many1(one_of("AKQJT998765432").map(|card_value| match card_value {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Shouldn't happen"),
    }))
    .map(|cards| {
        if cards.len() != 5 {
            panic!("There is a hand with more than 5 cards!");
        }

        let grouped_cards = cards
            .clone()
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, card| {
                acc.entry(card).and_modify(|num| *num += 1).or_insert(1);
                acc
            });

        let hand = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        match grouped_cards.len() {
            1 => HandValue::FiveOfAKind(hand),
            2 => match grouped_cards.into_values().max().expect("Should have max") {
                4 => HandValue::FourOfAKind(hand),
                3 => HandValue::FullHouse(hand),
                _ => panic!("Shouldn't happen!"),
            },
            3 => match grouped_cards.into_values().max().expect("Should have max") {
                3 => HandValue::ThreeOfAKind(hand),
                2 => HandValue::TwoPair(hand),
                _ => panic!("Shouldn't happen!"),
            },
            4 => HandValue::OnePair(hand),
            5 => HandValue::HighCard(hand),
            _ => panic!("Shouldn't happen!"),
        }
    })
    .parse(input)
}

fn hand(input: &str) -> IResult<&str, Hand, ErrorTree<&str>> {
    separated_pair(hand_value, space0, complete::u32)
        .map(|(value, bid)| Hand { value, bid })
        .parse(input)
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>, ErrorTree<&str>> {
    separated_list1(newline, hand).parse(input)
}

fn hand_value_with_jokers(input: &str) -> IResult<&str, HandValueWithJoker, ErrorTree<&str>> {
    many1(one_of("AKQJT998765432").map(|card_value| match card_value {
        'A' => CardWithJoker::A,
        'K' => CardWithJoker::K,
        'Q' => CardWithJoker::Q,
        'J' => CardWithJoker::J,
        'T' => CardWithJoker::T,
        '9' => CardWithJoker::Nine,
        '8' => CardWithJoker::Eight,
        '7' => CardWithJoker::Seven,
        '6' => CardWithJoker::Six,
        '5' => CardWithJoker::Five,
        '4' => CardWithJoker::Four,
        '3' => CardWithJoker::Three,
        '2' => CardWithJoker::Two,
        _ => panic!("Shouldn't happen"),
    }))
    .map(|cards| {
        if cards.len() != 5 {
            panic!("There is a hand with more than 5 cards!");
        }

        let mut grouped_cards = cards
            .clone()
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, card| {
                acc.entry(card).and_modify(|num| *num += 1).or_insert(1);
                acc
            });

        if let Some(num_jokers) = grouped_cards.clone().get(&CardWithJoker::J) {
            if *num_jokers != 5 {
                grouped_cards.remove(&CardWithJoker::J);

                let (max_key, _) = grouped_cards.clone().into_iter().fold(
                    (CardWithJoker::Two, 0),
                    |mut acc, card| {
                        if card.1 > acc.1 {
                            acc = card
                        }
                        acc
                    },
                );

                grouped_cards
                    .entry(max_key)
                    .and_modify(|num| *num += num_jokers);
            }
        }

        let hand = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        match grouped_cards.len() {
            1 => HandValueWithJoker::FiveOfAKind(hand),
            2 => match grouped_cards.into_values().max().expect("Should have max") {
                4 => HandValueWithJoker::FourOfAKind(hand),
                3 => HandValueWithJoker::FullHouse(hand),
                _ => panic!("Shouldn't happen!"),
            },
            3 => match grouped_cards.into_values().max().expect("Should have max") {
                3 => HandValueWithJoker::ThreeOfAKind(hand),
                2 => HandValueWithJoker::TwoPair(hand),
                _ => panic!("Shouldn't happen!"),
            },
            4 => HandValueWithJoker::OnePair(hand),
            5 => HandValueWithJoker::HighCard(hand),
            _ => {
                dbg!(grouped_cards);
                panic!("Shouldn't happen!")
            }
        }
    })
    .parse(input)
}

fn hand_with_jokers(input: &str) -> IResult<&str, HandWithJoker, ErrorTree<&str>> {
    separated_pair(hand_value_with_jokers, space0, complete::u32)
        .map(|(value, bid)| HandWithJoker { value, bid })
        .parse(input)
}

fn parse_hands_with_jokers(input: &str) -> IResult<&str, Vec<HandWithJoker>, ErrorTree<&str>> {
    separated_list1(newline, hand_with_jokers).parse(input)
}

fn part1(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;

    let (_, mut hands) = parse_hands(input.as_str()).expect("Should be well formed");

    hands.sort();

    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum())
}

fn part2(path: &str) -> Result<u32> {
    let input = aoc23::load_input(path)?;

    let (_, mut hands) = parse_hands_with_jokers(input.as_str()).expect("Should be well formed");

    hands.sort();

    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum())
}

fn main() {
    println!("Part 1: {}", part1("data/7.input").unwrap());
    println!("Part 2: {}", part2("data/7.input").unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    use super::{Card::*, HandValue::*};
    use test_case::test_case;

    #[test_case(FiveOfAKind([A, A, A, A, A]),
                FourOfAKind([A, A, A, A, K])
                ; "Simple ordering by type")]
    #[test_case(FiveOfAKind([A, A, A, A, A]),
                FiveOfAKind([A, A, A, A, K])
                ; "Order by highest first value")]
    #[test_case(ThreeOfAKind([Two, Four, Four, Four, A]),
                ThreeOfAKind([Two, Two, Two, Three, K])
                ; "Order by highest first value, sanity check")]
    fn hands_order_test(a: HandValue, b: HandValue) {
        assert!(a > b);
    }

    #[test_case(A, K ; "Ace larger than King")]
    #[test_case(J, Five ; "Jack larger than ten")]
    fn cards_order_test(a: Card, b: Card) {
        assert!(a > b);
    }

    #[test]
    fn hand_parser_test() {
        let test_hand = hand("AAAAA  123").unwrap();
        assert_eq!(test_hand.0, "");
        assert_eq!(
            test_hand.1,
            Hand {
                value: FiveOfAKind((A, A, A, A, A)),
                bid: 123
            }
        );
    }

    #[test]
    fn hand_value_parser_test() {
        let test_hand = hand_value("AAAAA").unwrap();
        assert_eq!(test_hand.0, "");
        assert_eq!(test_hand.1, FiveOfAKind((A, A, A, A, A)),);
    }

    #[test]
    fn part1_test() {
        assert_eq!(part1("data/7.sample").unwrap(), 6440)
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2("data/7.sample").unwrap(), 5905)
    }
}
