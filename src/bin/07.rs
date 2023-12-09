use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Hand {
    strengths: Vec<u32>,
    rank: u32,
    bid: u32,
}

enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn get_type(strengths: Vec<u32>) -> Type {
    match strengths
        .iter()
        .fold(HashMap::new(), |mut map, s| {
            *map.entry(s).or_insert(0) += 1;
            map
        })
        .values()
        .sorted()
        .collect_vec()[..]
    {
        [5] => Type::FiveOfAKind,
        [1, 4] => Type::FourOfAKind,
        [2, 3] => Type::FullHouse,
        [1, 1, 3] => Type::ThreeOfAKind,
        [1, 2, 2] => Type::TwoPair,
        [1, 1, 1, 2] => Type::OnePair,
        [1, 1, 1, 1, 1] => Type::HighCard,
        _ => unimplemented!(),
    }
}

fn parse_hand(line: &str) -> Option<Hand> {
    let mut elements = line.split(' ');
    let cards = elements.next()?;
    let bid = elements.next()?.parse::<u32>().ok()?;

    let strengths = cards
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        })
        .collect::<Vec<u32>>();

    let rank = get_type(strengths.clone()) as u32;

    Some(Hand {
        strengths,
        rank,
        bid,
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .split_terminator('\n')
        .filter_map(parse_hand)
        .collect();

    hands.sort_unstable_by_key(|hand| {
        Reverse((
            hand.rank,
            hand.strengths[0],
            hand.strengths[1],
            hand.strengths[2],
            hand.strengths[3],
            hand.strengths[4],
        ))
    });

    Some(
        hands
            .iter()
            .zip((1..=hands.len()).rev())
            .map(|(hand, i)| hand.bid * i as u32)
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
