use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ParseGameError {
    InvalidFormat,
    InvalidId(String),
    InvalidSet(ParseSetError),
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(':').ok_or(ParseGameError::InvalidFormat)?;

        let id = parts
            .0
            .split_once(' ')
            .ok_or(ParseGameError::InvalidFormat)?
            .1
            .parse::<u32>()
            .map_err(|_| ParseGameError::InvalidId(parts.0.to_string()))?;

        let sets: Vec<Set> = parts
            .1
            .split(';')
            .map(Set::from_str)
            .try_collect()
            .map_err(ParseGameError::InvalidSet)?;

        Ok(Game { id, sets })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ParseSetError {
    CubeAlreadyDefined,
    InvalidCube(ParseCubeError),
}

impl FromStr for Set {
    type Err = ParseSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(Cube::from_str)
            .try_fold((None, None, None), |mut acc, cube| {
                let cube = cube.map_err(ParseSetError::InvalidCube)?;
                macro_rules! set_color {
                    ($index:tt) => {
                        acc.$index = Some(
                            acc.$index
                                .map_or(Ok(cube.n), |_| Err(ParseSetError::CubeAlreadyDefined))?,
                        )
                    };
                }

                match cube.color {
                    CubeColor::Red => set_color!(0),
                    CubeColor::Green => set_color!(1),
                    CubeColor::Blue => set_color!(2),
                }

                Ok::<_, ParseSetError>(acc)
            })
            .map(|(red, green, blue)| Set {
                red: red.unwrap_or(0),
                green: green.unwrap_or(0),
                blue: blue.unwrap_or(0),
            })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cube {
    n: u32,
    color: CubeColor,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ParseCubeError {
    InvalidFormat,
    InvalidNumber(String),
    InvalidColor(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = ParseCubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            _ => Err(ParseCubeError::InvalidColor(s.to_string())),
        }
    }
}

impl FromStr for Cube {
    type Err = ParseCubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, color) = s
            .trim()
            .split_once(' ')
            .ok_or(ParseCubeError::InvalidFormat)?;

        let n = n
            .parse::<u32>()
            .map_err(|_| ParseCubeError::InvalidNumber(n.to_string()))?;
        let color = color.parse::<CubeColor>()?;

        Ok(Cube { n, color })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let v = input
        .lines()
        .filter_map(|ln| {
            let game = ln.parse::<Game>().expect("all games are valid");
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

pub fn part_two(input: &str) -> Option<u32> {
    let v = input
        .lines()
        .filter_map(|ln| {
            let game = ln.parse::<Game>().expect("all games are valid");
            let biggest = game.sets.iter().fold(Set::default(), |acc, set| Set {
                red: acc.red.max(set.red),
                green: acc.green.max(set.green),
                blue: acc.blue.max(set.blue),
            });

            Some(biggest.red * biggest.green * biggest.blue)
        })
        .sum();

    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cub() {
        let input = " 3 blue";
        let result = input.parse::<Cube>().expect("cube is valid");
        assert_eq!(
            result,
            Cube {
                n: 3,
                color: CubeColor::Blue
            }
        );
    }

    #[test]
    fn test_parse_set() {
        let input = " 3 blue, 4 red";
        let result = input.parse::<Set>().expect("set is valid");
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
        let result = input.parse::<Game>().expect("game is valid");
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
        assert_eq!(result, Some(2286));
    }
}
