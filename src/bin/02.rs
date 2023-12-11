advent_of_code::solution!(2);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_cube(input: &str) -> (u32, &str) {
    let mut parts = input.trim().split(' ');
    let n = parts.next().expect("id always exists").parse().unwrap();
    let color = parts.next().expect("color always exists");
    (n, color)
}

fn parse_set(input: &str) -> Set {
    input
        .split(',')
        .map(parse_cube)
        .fold(Set::default(), |mut acc, (n, color)| {
            match color {
                "red" => acc.red = n,
                "green" => acc.green = n,
                "blue" => acc.blue = n,
                _ => unreachable!(),
            }

            acc
        })
}

fn parse_game(input: &str) -> Game {
    let mut parts = input.split(':');
    let id = parts
        .next()
        .expect("game hader always exists")
        .split(' ')
        .nth(1)
        .expect("id always exists");
    let sets = parts.next().expect("sets always exists");

    Game {
        id: id.trim().parse().unwrap(),
        sets: sets.split(';').map(parse_set).collect(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let v = input
        .lines()
        .filter_map(|ln| {
            let game = parse_game(ln);
            let is_valid = game
                .sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14);

            if is_valid {
                Some(game.id)
            } else {
                None
            }
        })
        .sum();

    Some(v)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cub() {
        let input = " 3 blue";
        let result = parse_cube(input);
        assert_eq!(result, (3, "blue"));
    }

    #[test]
    fn test_parse_set() {
        let input = " 3 blue, 4 red";
        let result = parse_set(input);
        assert_eq!(
            result,
            Set {
                red: 4,
                green: 0,
                blue: 3
            }
        );
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game(input);
        assert_eq!(
            result,
            Game {
                id: 1,
                sets: vec![
                    Set {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    Set {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Set {
                        red: 0,
                        green: 2,
                        blue: 0
                    }
                ]
            }
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
