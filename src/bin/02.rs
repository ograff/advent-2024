use std::str::FromStr;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(2);
#[derive(Debug)]
struct Report {
    levels: Vec<u64>,
}

impl FromStr for Report {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            levels: s.split_whitespace().map(|x| x.parse().unwrap()).collect(),
        })
    }
}
impl Report {
    fn is_safe(&self) -> bool {
        let all_decreasing = self.levels.windows(2).all(|window| {
            let left = window[0];
            let right = window[1];
            left > right && (left - right) <= 3
        });
        let all_increasing = self.levels.windows(2).all(|window| {
            let left = window[0];
            let right = window[1];
            right > left && (right - left) <= 3
        });

        all_decreasing || all_increasing
    }
    fn is_safe2(&self) -> bool {
        self.is_safe() || {
            self.levels.iter().enumerate().any(|(pos, _)| {
                let mut new_vec: Vec<u64> = Vec::new();
                new_vec.extend(&self.levels[0..pos]);
                new_vec.extend(&self.levels[(pos+1)..]);
                Report {levels: new_vec}.is_safe()
            })
        }
    }
}
fn get_reports(input: &str) -> Vec<Report> {
    input.lines().map(|line| Report::from_str(line).unwrap()).collect()
}
pub fn part_one(input: &str) -> Option<usize> {
    let reports = get_reports(input);
    Some(reports.iter().filter(|x| x.is_safe()).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = get_reports(input);
    Some(reports.iter().filter(|x| x.is_safe2()).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
