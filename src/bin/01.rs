use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let v = input
        .lines()
        .map(|ln| {
            let mut parsed = ln.chars().filter_map(|c| c.to_digit(10));
            let first = parsed.next().unwrap_or(0);
            let last = parsed.last().unwrap_or(first);
            first * 10 + last
        })
        .sum();

    Some(v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"^(one|two|three|four|five|six|seven|eight|nine|\d)")
        .expect("regex is correct");
    let v = input
        .lines()
        .map(|ln| {
            // NOTE: There are overlapping matches, so we need to use a
            // char_indices iterator to get the correct matches.
            // Source: https://stackoverflow.com/a/77594663
            let mut parsed = ln
                .char_indices()
                .filter_map(|(i, _)| re.captures(&ln[i..]))
                .map(|c| match c.get(0).expect("should always match").as_str() {
                    "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    "four" => 4,
                    "five" => 5,
                    "six" => 6,
                    "seven" => 7,
                    "eight" => 8,
                    "nine" => 9,
                    d => d.parse::<u32>().expect("should always be a valid digit"),
                });

            let first = parsed.next().unwrap_or(0);
            let last = parsed.last().unwrap_or(first);
            first * 10 + last
        })
        .sum();

    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
