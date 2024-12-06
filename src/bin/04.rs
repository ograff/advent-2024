use std::str::FromStr;

advent_of_code::solution!(4);

struct Grid {
    rows: Vec<Vec<char>>,
}
impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s.lines().map(|line| line.chars().collect()).collect(),
        })
    }
}
#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    DiagDownRight,
    DiagDownLeft,
    DiagUpRight,
    DiagUpLeft,
}
impl Direction {
    fn get_adj(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::DiagDownRight => (1, 1),
            Direction::DiagDownLeft => (1, -1),
            Direction::DiagUpRight => (-1, 1),
            Direction::DiagUpLeft => (-1, -1),
        }
    }
}
impl Grid {
    fn get(&self, row: usize, col: usize) -> Option<char> {
        return self
            .rows
            .get(row)
            .map(|row| row.get(col).map(|x| *x))
            .flatten();
    }
    fn get_dir(&self, row: usize, col: usize, dir: Direction, n: isize) -> Option<char> {
        self.get(
            row.checked_add_signed(dir.get_adj().0 * n)?,
            col.checked_add_signed(dir.get_adj().1 * n)?,
        )
    }
    fn get_adj(&self, row_i: usize, col_i: usize) -> Vec<(usize, usize)> {
        [
            (1_isize, 0_isize),
            (-1, 0),
            (0, 1),
            (0, -1),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ]
        .iter()
        .filter_map(|(row_diff, col_diff)| {
            Some((
                row_i.checked_add_signed(*row_diff)?,
                col_i.checked_add_signed(*col_diff)?,
            ))
        })
        .collect()
    }
}

fn get_next(n: char) -> char {
    match n {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!(),
    }
}
fn remainder(
    grid: &Grid,
    row_i: usize,
    col_i: usize,
    direction: Direction,
    next_letter: char,
    i: isize,
) -> bool {
    if let Some(ch) = grid.get_dir(row_i, col_i, direction, i) {
        if ch == next_letter && ch == 'S' {
            return true;
        } else if ch == next_letter {
            remainder(grid, row_i, col_i, direction, get_next(next_letter), i + 1)
        } else {
            return false;
        }
    } else {
        false
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::from_str(input).unwrap();
    let mut num = 0;
    for (row_i, row) in grid.rows.iter().enumerate() {
        for (col_i, ch) in row.iter().enumerate() {
            if *ch == 'X' {
                num += [
                    remainder(&grid, row_i, col_i, Direction::Left, 'M', 1),
                    remainder(&grid, row_i, col_i, Direction::Right, 'M', 1),
                    remainder(&grid, row_i, col_i, Direction::Up, 'M', 1),
                    remainder(&grid, row_i, col_i, Direction::Down, 'M', 1),
                    remainder(&grid, row_i, col_i, Direction::DiagDownLeft, 'M', 1),
                    remainder(&grid, row_i, col_i, Direction::DiagDownRight, 'M', 1),
                    remainder(&grid, row_i, col_i, Direction::DiagUpLeft, 'M', 1),
                    remainder(&grid, row_i, col_i, Direction::DiagUpRight, 'M', 1),
                ]
                .iter()
                .filter(|x| **x)
                .count();
            }
        }
    }
    Some(num)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let mut num = 0;
    for (row_i, row) in grid.rows.iter().enumerate() {
        for (col_i, ch) in row.iter().enumerate() {
            if *ch == 'A' {
                if [
                    [
                        grid.get_dir(row_i, col_i, Direction::DiagUpLeft, 1) == Some('M'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownRight, 1) == Some('S'),
                        grid.get_dir(row_i, col_i, Direction::DiagUpRight, 1) == Some('S'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownLeft, 1) == Some('M'),
                    ]
                    .iter()
                    .all(|x| *x),
                    [
                        grid.get_dir(row_i, col_i, Direction::DiagUpLeft, 1) == Some('S'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownRight, 1) == Some('M'),
                        grid.get_dir(row_i, col_i, Direction::DiagUpRight, 1) == Some('S'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownLeft, 1) == Some('M'),
                    ]
                    .iter()
                    .all(|x| *x),
                    [
                        grid.get_dir(row_i, col_i, Direction::DiagUpLeft, 1) == Some('M'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownRight, 1) == Some('S'),
                        grid.get_dir(row_i, col_i, Direction::DiagUpRight, 1) == Some('M'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownLeft, 1) == Some('S'),
                    ]
                    .iter()
                    .all(|x| *x),
                    [
                        grid.get_dir(row_i, col_i, Direction::DiagUpLeft, 1) == Some('S'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownRight, 1) == Some('M'),
                        grid.get_dir(row_i, col_i, Direction::DiagUpRight, 1) == Some('M'),
                        grid.get_dir(row_i, col_i, Direction::DiagDownLeft, 1) == Some('S'),
                    ]
                    .iter()
                    .all(|x| *x),
                ]
                .iter()
                .any(|x| *x)
                {
                    num += 1;
                }
            }
        }
    }
    Some(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
