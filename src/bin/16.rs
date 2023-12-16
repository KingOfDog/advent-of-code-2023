use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(16);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn traverse_beam_iter(
    grid: Vec<Vec<char>>,
    start_pos: (i32, i32),
    dir: Direction,
) -> HashSet<(i32, i32)> {
    let mut beam_heads = vec![(start_pos.0, start_pos.1, dir)];
    let mut visited = HashSet::new();

    while !beam_heads.is_empty() {
        let mut new_beam_heads = vec![];
        for (x, y, dir) in beam_heads {
            if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                continue;
            }
            if visited.contains(&(x, y, dir)) {
                continue;
            }

            let current = grid[y as usize][x as usize];
            match current {
                '/' => {
                    let dir = match dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    new_beam_heads.push((x + dir.offset().0, y + dir.offset().1, dir));
                }
                '\\' => {
                    let dir = match dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    new_beam_heads.push((x + dir.offset().0, y + dir.offset().1, dir));
                }
                '-' if matches!(dir, Direction::Down | Direction::Up) => {
                    new_beam_heads.push((
                        x + Direction::Left.offset().0,
                        y + Direction::Left.offset().1,
                        Direction::Left,
                    ));
                    new_beam_heads.push((
                        x + Direction::Right.offset().0,
                        y + Direction::Right.offset().1,
                        Direction::Right,
                    ));
                }
                '|' if matches!(dir, Direction::Left | Direction::Right) => {
                    new_beam_heads.push((
                        x + Direction::Up.offset().0,
                        y + Direction::Up.offset().1,
                        Direction::Up,
                    ));
                    new_beam_heads.push((
                        x + Direction::Down.offset().0,
                        y + Direction::Down.offset().1,
                        Direction::Down,
                    ));
                }
                _ => {
                    new_beam_heads.push((x + dir.offset().0, y + dir.offset().1, dir));
                }
            }
            visited.insert((x, y, dir));
        }
        beam_heads = new_beam_heads;
    }

    visited.into_iter().map(|(x, y, _)| (x, y)).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let energized = traverse_beam_iter(grid, (0, 0), Direction::Right);
    Some(energized.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut starting_points = Vec::new();
    starting_points.extend((0..grid.len()).map(|y| (0, y, Direction::Right)));
    starting_points.extend((0..grid.len()).map(|y| (grid[0].len() - 1, y, Direction::Left)));
    starting_points.extend((0..grid[0].len()).map(|x| (x, 0, Direction::Down)));
    starting_points.extend((0..grid[0].len()).map(|x| (x, grid.len() - 1, Direction::Up)));

    starting_points
        .into_par_iter()
        .map(|(x, y, dir)| traverse_beam_iter(grid.clone(), (x as i32, y as i32), dir).len() as u32)
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
