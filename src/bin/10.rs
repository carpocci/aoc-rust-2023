use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(10);

#[derive(EnumIter, Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_map(input: &str) -> (Vec<Vec<char>>, usize, (usize, usize)) {
    let mut map = input
        .split_terminator('\n')
        .map(|line| format!(".{}.", line).chars().collect_vec())
        .collect_vec();

    let size = map[0].len();

    let dotted_line = (0..size).map(|_| '.').collect_vec();
    map.insert(0, dotted_line.clone());
    map.push(dotted_line.clone());

    let mut start = (0, 0);
    'main: for (i, line) in map.iter().enumerate() {
        for (j, &cell) in line.iter().enumerate() {
            if cell == 'S' {
                start = (i, j);
                break 'main;
            }
        }
    }

    (map, size, start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, _size, start) = parse_map(input);

    let mut max = 0;

    'main: for mut direction in Direction::iter() {
        let (mut i, mut j) = start;
        let mut jumps = 0;
        loop {
            (i, j) = match direction {
                Direction::North => (i - 1, j),
                Direction::East => (i, j + 1),
                Direction::South => (i + 1, j),
                Direction::West => (i, j - 1),
            };

            jumps += 1;

            match (direction, map[i][j]) {
                (_, 'S') => {
                    max = jumps.max(max);
                    continue 'main;
                }
                (Direction::North, '7') => direction = Direction::West,
                (Direction::North, 'F') => direction = Direction::East,
                (Direction::South, 'J') => direction = Direction::West,
                (Direction::South, 'L') => direction = Direction::East,
                (Direction::North | Direction::South, '|') => (),
                (Direction::East, '7') => direction = Direction::South,
                (Direction::East, 'J') => direction = Direction::North,
                (Direction::West, 'L') => direction = Direction::North,
                (Direction::West, 'F') => direction = Direction::South,
                (Direction::East | Direction::West, '-') => (),
                _ => continue 'main,
            }
        }
    }

    Some(max / 2)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));

        let tests: Vec<(&str, u32)> = vec![("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...", 8)];

        for (input, expected) in &tests {
            assert_eq!(part_one(input), Some(*expected))
        }
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
