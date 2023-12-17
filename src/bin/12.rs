advent_of_code::solution!(12);

fn is_valid(line: &str) -> bool {
    let (springs, conditions) = line.split_once(' ').unwrap();
    let conditions: Vec<u32> = conditions
        .split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    let mut counts = vec![];
    let mut current: u32 = 0;
    for c in springs.chars() {
        match c {
            '#' => current += 1,
            _ => match current {
                0 => (),
                _ => {
                    counts.push(current);
                    current = 0
                }
            },
        }
    }

    if current != 0 {
        counts.push(current);
    }

    conditions == counts
}

fn parse_line(line: &str) -> u32 {
    if line.contains('?') {
        parse_line(line.replacen('?', ".", 1).as_str())
            + parse_line(line.replacen('?', "#", 1).as_str())
    } else {
        match is_valid(line) {
            true => 1,
            false => 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line).sum())
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
