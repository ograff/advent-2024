advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        re.captures_iter(input)
            .map(|x| x.extract())
            .map(|(_, [op1, op2])| op1.parse::<u64>().unwrap() * op2.parse::<u64>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        input
            .split("do()")
            .map(|sub| sub.split_once("don't()").unwrap_or((sub, "")).0)
            .map(|sub| {
                re.captures_iter(sub)
                    .map(|x| x.extract())
                    .map(|(_, [op1, op2])| {
                        op1.parse::<u64>().unwrap() * op2.parse::<u64>().unwrap()
                    })
                    .sum::<u64>()
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
