use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let default_game = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    Some(
        input
            .split_terminator('\n')
            .filter_map(|line| {
                let mut elements = line.split(": ");
                let id = elements
                    .next()?
                    .strip_prefix("Game ")?
                    .parse::<u32>()
                    .unwrap();

                if elements.next()?.split("; ").any(|game| {
                    game.split(", ").any(|pick| {
                        let mut items = pick.split(' ');
                        let n = items.next().unwrap().parse::<u32>().unwrap();
                        let color = items.next().unwrap();
                        default_game[color] < n
                    })
                }) {
                    None
                } else {
                    Some(id)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split_terminator('\n')
            .map(|line| {
                let mut current_game = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split("; ")
                    .for_each(|game| {
                        game.split(", ").for_each(|pick| {
                            let mut items = pick.split(' ');
                            let n = items.next().unwrap().parse::<u32>().unwrap();
                            let color = items.next().unwrap();

                            if let Some(c) = current_game.get_mut(color) {
                                *c = u32::max(*c, n);
                            }
                        });
                    });

                current_game.values().product::<u32>()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
