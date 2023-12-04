use std::collections::HashSet;

advent_of_code::solution!(4);

fn parse_card(line: &str) -> usize {
    let numbers = &mut line.split(": ").nth(1).unwrap().split(" | ").map(|list| {
        list.split(' ')
            .filter_map(|d| d.parse::<u32>().ok())
            .collect::<HashSet<u32>>()
    });

    numbers
        .next()
        .unwrap()
        .intersection(&numbers.next().unwrap())
        .count()
}

fn parse_cards(input: &str) -> Vec<usize> {
    input
        .split_terminator('\n')
        .map(parse_card)
        .collect::<Vec<usize>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_cards(input)
            .iter()
            .filter(|&count| count > &0)
            .map(|&count| 2_usize.pow(count as u32 - 1))
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let cards = parse_cards(input);
    let len = cards.len();

    let mut card_counts = vec![1; len];

    cards.iter().enumerate().for_each(|(i, &count)| {
        (1..=count).for_each(|j| card_counts[i + j] += card_counts[i]);
    });

    Some(card_counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
