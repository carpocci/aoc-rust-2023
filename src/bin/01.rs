use std::collections::HashMap;

advent_of_code::solution!(1);

fn solve(input: &str) -> Option<u32> {
    Some(
        input
            .split_terminator('\n')
            .map(|line| {
                let digits = line
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>();
                digits[0] * 10 + digits[digits.len() - 1]
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    solve(
        &input
            .split_terminator('\n')
            .map(|line| {
                let mut newline: String = String::from(line);
                for (&from, &to) in &numbers {
                    newline = newline.replace(from, to);
                }

                newline
            })
            .collect::<Vec<String>>()
            .join("\n"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
