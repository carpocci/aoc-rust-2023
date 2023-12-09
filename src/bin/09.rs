use itertools::Itertools;

advent_of_code::solution!(9);

fn solve(input: &str, backwards: bool) -> Option<i32> {
    Some(
        input
            .split_terminator('\n')
            .filter_map(|line| {
                let mut sequences = vec![];
                sequences.push({
                    let mut sequence = line
                        .split(' ')
                        .filter_map(|n| n.parse::<i32>().ok())
                        .collect_vec();
                    if backwards {
                        sequence.reverse();
                    }
                    sequence
                });

                while sequences.last()?.iter().any(|&n| n != 0) {
                    sequences.push(
                        (0..sequences.last()?.len() - 1)
                            .filter_map(|i| Some(sequences.last()?[i + 1] - sequences.last()?[i]))
                            .collect_vec(),
                    )
                }

                (0..sequences.len() - 2).rev().for_each(|i| {
                    let new_item = sequences[i + 1].last().unwrap() + sequences[i].last().unwrap();
                    sequences[i].push(new_item);
                });

                Some(*sequences[0].last()?)
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<i32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<i32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
