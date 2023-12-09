use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> Option<(Vec<usize>, HashMap<&str, Vec<&str>>, Vec<&str>)> {
    let mut elements = input.split_terminator("\n\n");
    let directions: Vec<usize> = elements
        .next()?
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect();

    let mut ghosts = vec![];

    let map: HashMap<&str, Vec<&str>> =
        elements
            .next()?
            .split_terminator('\n')
            .fold(HashMap::new(), |mut map, line| {
                let regex = Regex::new(r"[A-Z0-9]{3}").unwrap();
                let (node, left, right) = regex
                    .find_iter(line)
                    .map(|mat| mat.as_str())
                    .collect_tuple()
                    .unwrap();

                if node.chars().nth(2).unwrap() == 'A' {
                    ghosts.push(node);
                }

                map.insert(node, vec![left, right]);
                map
            });

    Some((directions, map, ghosts))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, map, _) = parse_input(input)?;

    let mut jumps = 0;
    let mut current_node = "AAA";
    for direction in directions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }
        current_node = map.get(current_node).unwrap()[*direction];
        jumps += 1;
    }

    Some(jumps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, map, ghosts) = parse_input(input)?;

    Some(
        ghosts
            .iter()
            .map(|&ghost| {
                let mut jumps = 0;
                let mut node = ghost;
                for direction in directions.iter().cycle() {
                    if node.chars().nth(2).unwrap() == 'Z' {
                        break;
                    }

                    node = map.get(node).unwrap()[*direction];
                    jumps += 1;
                }
                jumps
            })
            .fold(1, |acc, n| lcm(acc, n)),
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
        assert_eq!(result, Some(2));

        let result = part_one("LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
