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

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
