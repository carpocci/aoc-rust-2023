use itertools::Itertools;

advent_of_code::solution!(6);

fn solve((time, distance): (i64, i64)) -> i64 {
    let delta = ((time * time - 4 * distance) as f64).sqrt();
    let (x1, x2) = (
        ((-time as f64 - delta) / 2.0).floor() as i64,
        ((-time as f64 + delta) / 2.0).ceil() as i64,
    );
    x2 - x1 - 1
}

pub fn part_one(input: &str) -> Option<i64> {
    let (times, distances) = input
        .split_terminator('\n')
        .map(|line| line.split(' ').filter_map(|n| n.parse::<i64>().ok()))
        .next_tuple()?;

    Some(times.zip(distances).map(solve).product())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (time, distance) = input
        .split_terminator('\n')
        .map(|line| {
            line.split_whitespace()
                .collect::<String>()
                .split(':')
                .nth(1)
                .unwrap()
                .parse::<i64>()
                .unwrap()
        })
        .next_tuple()?;

    Some(solve((time, distance)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
