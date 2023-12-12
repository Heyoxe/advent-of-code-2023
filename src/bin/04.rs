use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn matching_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .copied()
            .collect_vec()
    }

    pub fn points(&self) -> u32 {
        let n = self.matching_numbers().len() as u32;
        if n == 0 {
            return 0;
        }

        2u32.pow(n - 1)
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, v) = s.split_once(':').expect("card are always valid");
        let (winning_numbers, numbers) = v.split_once(" | ").expect("card are always valid");
        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .filter_map(|part| {
                if part.is_empty() {
                    return None;
                }

                let n = part.parse::<u32>().expect("card are always valid");
                Some(n)
            })
            .unique()
            .collect_vec();

        let numbers = numbers
            .trim()
            .split(' ')
            .filter_map(|part| {
                if part.is_empty() {
                    return None;
                }

                let n = part.parse::<u32>().expect("card are always valid");
                Some(n)
            })
            .unique()
            .collect_vec();

        let card = Card {
            winning_numbers,
            numbers,
        };

        Ok(card)
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|ln| ln.parse::<Card>().expect("card are always valid"))
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_input(input)
        .iter()
        .map(|c| c.points())
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = parse_input(input)
        .iter()
        .map(|c| (c.matching_numbers().len(), 1))
        .collect_vec();

    let mut acc = 0;
    for i in 0..cards.len() {
        let (m, c) = cards[i];
        acc += c;
        let next = &mut cards[(i + 1)..(i + m + 1)];

        for v in next {
            v.1 += c;
        }
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
