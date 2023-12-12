use itertools::Itertools;

advent_of_code::solution!(11);

fn find_empty_space(map: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = map
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|c| *c == '.'))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let empty_columns = (0..map[0].len())
        .filter(|&i| map.iter().all(|line| line[i] == '.'))
        .collect::<Vec<_>>();

    (empty_rows, empty_columns)
}

struct Galaxy {
    x: usize,
    y: usize,
}

fn get_distance(
    galaxy: &Galaxy,
    other_galaxy: &Galaxy,
    empty_rows: &[usize],
    empty_columns: &[usize],
    empty_space_lenght: u64,
) -> u64 {
    let mut dist = ((galaxy.x as i32 - other_galaxy.x as i32).abs()
        + (galaxy.y as i32 - other_galaxy.y as i32).abs()) as u64;

    dist += empty_rows
        .iter()
        .filter(|&&row| {
            (row > galaxy.x && row < other_galaxy.x) || (row > other_galaxy.x && row < galaxy.x)
        })
        .count() as u64
        * empty_space_lenght;

    dist += empty_columns
        .iter()
        .filter(|&&col| {
            (col > galaxy.y && col < other_galaxy.y) || (col > other_galaxy.y && col < galaxy.y)
        })
        .count() as u64
        * empty_space_lenght;

    dist
}

fn solve(input: &str, empty_space_lenght: u64) -> Option<u64> {
    let empty_space_lenght = empty_space_lenght - 1;

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (empty_rows, empty_columns) = find_empty_space(&map);

    let galaxies = map
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(y, _)| Galaxy { x, y })
        })
        .collect::<Vec<_>>();

    Some(
        galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, galaxy)| {
                galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|other_galaxy| {
                        get_distance(
                            galaxy,
                            other_galaxy,
                            &empty_rows,
                            &empty_columns,
                            empty_space_lenght,
                        )
                    })
                    .collect_vec()
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        for (empty_space_lenght, result) in vec![(10, 1030), (100, 8410)] {
            assert_eq!(solve(input, empty_space_lenght), Some(result));
        }
    }
}
