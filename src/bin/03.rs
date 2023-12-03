advent_of_code::solution!(3);

fn set_true(mut symbols_table: Vec<Vec<bool>>, i: usize, j: usize) -> Vec<Vec<bool>> {
    if i > 0 {
        if j > 0 {
            symbols_table[i - 1][j - 1] = true;
        }
        symbols_table[i - 1][j] = true;
        symbols_table[i - 1][j + 1] = true;
    }

    if j > 0 {
        symbols_table[i][j - 1] = true;
    }
    symbols_table[i][j] = true;
    symbols_table[i][j + 1] = true;

    if j > 0 {
        symbols_table[i + 1][j - 1] = true;
    }
    symbols_table[i + 1][j] = true;
    symbols_table[i + 1][j + 1] = true;

    symbols_table
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input
        .replace("\n", ".\n")
        .split_terminator('\n')
        .map(|row| row.chars().collect())
        .collect();

    let columns = map[0].len();
    map.push((0..columns).map(|_| '.').collect());
    let rows = map.len();

    let mut symbols_table: Vec<Vec<bool>> = (0..rows)
        .map(|_i| (0..columns).map(|_j| false).collect())
        .collect();

    for i in 0..rows {
        for j in 0..columns {
            if !(map[i][j].is_numeric() || map[i][j] == '.') {
                symbols_table = set_true(symbols_table, i, j);
            }
        }
    }

    let mut number = 0;
    let mut take = false;

    let mut sum = 0;

    for i in 0..rows {
        for j in 0..columns {
            let cell = map[i][j].to_digit(10);
            match cell {
                Some(n) => {
                    number *= 10;
                    number += n;
                    if symbols_table[i][j] {
                        take = true;
                    }
                }
                None => {
                    if number != 0 {
                        if take {
                            sum += number;
                        } else {
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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
