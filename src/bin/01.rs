use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
    str::ParallelString,
};

advent_of_code::solution!(1);
fn parse_list(input: &str) -> (Vec<u64>, Vec<u64>) {
    let num_lines = input.par_lines().count();
    let (mut left, mut right) = (Vec::with_capacity(num_lines), Vec::with_capacity(num_lines));
    for line in input.lines() {
        let mut it = line.split_whitespace();
        left.push(it.next().unwrap().parse().unwrap());
        right.push(it.next().unwrap().parse().unwrap());
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_list(input);
    left.par_sort();
    right.par_sort();
    Some(
        left.par_iter()
            .zip(right)
            .map(|(left, right)| left.abs_diff(right))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_list(input);
    let val = left.iter().map(|&x| {
        x * (right
            .iter()
            .filter(|right_val| **right_val == x)
            .count() as u64)
    }).sum();
    Some(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
