use rayon::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(5);

struct OrderingRule {
    first: u64,
    second: u64,
}

impl FromStr for OrderingRule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once("|").unwrap();
        Ok(Self {
            first: first.parse().unwrap(),
            second: second.parse().unwrap(),
        })
    }
}
impl OrderingRule {
    fn complies(&self, plan: &PrintPlan) -> bool {
        if plan.pages.contains(&self.first) && plan.pages.contains(&self.second) {
            plan.pages.iter().position(|x| *x == self.first).unwrap()
                < plan.pages.iter().position(|x| *x == self.second).unwrap()
        } else {
            true
        }
    }
}

#[derive(Clone, Debug)]
struct PrintPlan {
    pages: Vec<u64>,
}
impl FromStr for PrintPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pages: s.split(",").map(|x| x.parse().unwrap()).collect(),
        })
    }
}

impl PrintPlan {
    fn compliant(&self, rules: &Vec<OrderingRule>) -> bool {
        rules.iter().all(|x| x.complies(&self))
    }
    fn middle(&self) -> u64 {
        *(self.pages.get((self.pages.len() - 1) / 2).unwrap())
    }
    fn reorder(&self, rules: &Vec<OrderingRule>) -> PrintPlan {
        let mut candidate = self.clone();
        while !candidate.compliant(&rules) {
            for rule in rules {
                if !rule.complies(&candidate) {
                    let first_pos = candidate
                        .pages
                        .iter()
                        .position(|x| *x == rule.first)
                        .unwrap();
                    let second_pos = candidate
                        .pages
                        .iter()
                        .position(|x| *x == rule.second)
                        .unwrap();
                    candidate.pages.swap(first_pos, second_pos)
                }
            }
        }
        candidate
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut it = input.lines();
    let mut rules = Vec::new();
    loop {
        let line = it.next()?;
        if line == "" {
            break;
        }
        rules.push(OrderingRule::from_str(line).unwrap());
    }
    let plans: Vec<PrintPlan> = it.map(|line| PrintPlan::from_str(line).unwrap()).collect();

    Some(
        plans
            .par_iter()
            .filter(|plan| plan.compliant(&rules))
            .map(|plan| plan.middle())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut it = input.lines();
    let mut rules = Vec::new();
    loop {
        let line = it.next()?;
        if line == "" {
            break;
        }
        rules.push(OrderingRule::from_str(line).unwrap());
    }
    let plans: Vec<PrintPlan> = it.map(|line| PrintPlan::from_str(line).unwrap()).collect();
    Some(
        plans
            .par_iter()
            .filter(|plan| plan.compliant(&rules) == false)
            .map(|plan| plan.reorder(&rules))
            .map(|plan| plan.middle())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
