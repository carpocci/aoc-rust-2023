advent_of_code::solution!(13);

fn find_reflection_index(list: &Vec<&str>, tollerance: u32) -> Option<u32> {
    let mut reflection_index = None;
    for index in 0..list.len() - 1 {
        let (mut i, mut j) = (index, index + 1);
        let mut differences = 0;
        loop {
            if j == list.len() {
                break;
            }
            for (a, b) in list[i].chars().zip(list[j].chars()) {
                if a != b {
                    differences += 1;
                }
            }
            if i == 0 {
                break;
            }
            i -= 1;
            j += 1;
        }
        if differences == tollerance {
            reflection_index = Some((index + 1) as u32);
            break;
        }
    }

    reflection_index
}

fn solve(input: &str, tollerance: u32) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                let rows = group.lines().collect::<Vec<_>>();
                if let Some(reflection_index) = find_reflection_index(&rows, tollerance) {
                    return 100 * reflection_index;
                }

                let columns = (0..rows[0].len())
                    .map(|i| {
                        rows.iter()
                            .map(|row| row.chars().nth(i).unwrap())
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>();

                if let Some(reflection_index) =
                    find_reflection_index(&columns.iter().map(AsRef::as_ref).collect(), tollerance)
                {
                    reflection_index
                } else {
                    panic!("No reflection found");
                }
            })
            .sum(),
    )
    // None
}
pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
