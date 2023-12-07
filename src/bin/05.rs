use std::collections::HashSet;

advent_of_code::solution!(5);

fn parse_seeds(section: &str, part: u8) -> HashSet<u64> {
    let mut hash_set = HashSet::new();

    let seeds = section
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|n| n.parse::<u64>().ok());

    match part {
        1 => {
            for seed in seeds {
                hash_set.insert(seed);
            }
        }
        2 => {
            for elements in seeds.collect::<Vec<u64>>().chunks(2) {
                for new_seed in elements[0]..(elements[0] + elements[1]) {
                    hash_set.insert(new_seed);
                }
            }
        }
        _ => unimplemented!(),
    };

    hash_set
}

struct Range {
    destination: u64,
    source: u64,
    size: u64,
}

fn parse_map(input: &str) -> Vec<Range> {
    let mut map = vec![];

    for line in input.split_terminator('\n').skip(1) {
        let mut elements = line.split(' ').filter_map(|n| n.parse::<u64>().ok());

        map.push(Range {
            destination: elements.next().unwrap(),
            source: elements.next().unwrap(),
            size: elements.next().unwrap(),
        })
    }

    map
}

fn solve(input: &str, part: u8) -> Option<u64> {
    let mut sections = input.split("\n\n");
    let mut seeds = parse_seeds(sections.next().unwrap(), part);
    let maps = sections.map(parse_map);

    maps.for_each(|map| {
        let mut new_seeds = HashSet::new();
        seeds.iter().for_each(|&seed| {
            let mut found = false;
            for range in &map {
                if seed >= range.source && seed < (range.source + range.size) {
                    new_seeds.insert(range.destination + (seed - range.source));
                    found = true;
                    break;
                }
            }
            if !found {
                new_seeds.insert(seed);
            }
        });
        seeds = new_seeds;
    });

    Some(*seeds.iter().min().unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 1)
}

// It should work, but runs out of memory on my machine
pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
