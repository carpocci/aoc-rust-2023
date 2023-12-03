use std::collections::HashMap;

advent_of_code::solution!(3);

fn generate(
    input: &str,
    function: &dyn Fn(char) -> bool,
) -> (Vec<Vec<char>>, Vec<Vec<u32>>, usize) {
    let mut map: Vec<Vec<char>> = input
        .replace('\n', ".\n")
        .split_terminator('\n')
        .map(|row| row.chars().collect())
        .collect();

    let size = map[0].len();
    map.push(vec!['.'; size]);
    map.insert(0, vec!['.'; size]);

    let mut values_table: Vec<Vec<u32>> = vec![vec![0; size]; size];

    let mut id = 1;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if function(*cell) {
                for &i in &[i - 1, i, i + 1] {
                    for &j in &[j - 1, j, j + 1] {
                        values_table[i][j] = id;
                    }
                }
                id += 1;
            }
        }
    }

    (map, values_table, size)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, values_table, size) = generate(input, &|n| !(n.is_numeric() || n == '.'));

    let mut number = 0;
    let mut take = false;

    let mut sum = 0;

    for i in 0..size {
        for j in 0..size {
            match map[i][j].to_digit(10) {
                Some(n) => {
                    number *= 10;
                    number += n;
                    if values_table[i][j] != 0 {
                        take = true;
                    }
                }
                None => {
                    if number != 0 {
                        if take {
                            sum += number;
                        }
                        take = false;
                        number = 0;
                    }
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, values_table, size) = generate(input, &|n| !(n.is_numeric() || n == '.'));

    let mut number = 0;
    let mut gear_id = 0;

    let mut gears = HashMap::<u32, Vec<u32>>::new();

    for i in 0..size {
        for j in 0..size {
            match map[i][j].to_digit(10) {
                Some(n) => {
                    number *= 10;
                    number += n;
                    if values_table[i][j] != 0 {
                        gear_id = values_table[i][j];
                    }
                }
                None => {
                    if number != 0 {
                        if gear_id != 0 {
                            gears.entry(gear_id).or_default().push(number);
                        }
                        gear_id = 0;
                        number = 0;
                    }
                }
            }
        }
    }

    Some(
        gears
            .iter()
            .filter(|(_, v)| (**v).len() > 1)
            .map(|(_, v)| (*v).iter().product::<u32>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
