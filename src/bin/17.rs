use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use itertools::Itertools;

advent_of_code::solution!(17);

fn find_path(map: &Vec<Vec<u32>>, min_straight_distance: u32, max_straight_distance: u32) -> Path {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Path {
        position: Position { row: 0, col: 0 },
        direction: Direction { row: 0, col: 1 },
        distance: 0,
        heat: 0,
    });
    queue.push(Path {
        position: Position { row: 0, col: 0 },
        direction: Direction { row: 1, col: 0 },
        distance: 0,
        heat: 0,
    });

    while let Some(path) = queue.pop() {
        if path.position.row == map.len() as i32 - 1 && path.position.col == map[0].len() as i32 - 1
        {
            return path;
        }

        if path.distance < max_straight_distance {
            try_move(&mut queue, &mut visited, &map, path.direction, &path);
        }

        if path.distance >= min_straight_distance {
            try_move(
                &mut queue,
                &mut visited,
                &map,
                path.direction.turn_left(),
                &path,
            );
            try_move(
                &mut queue,
                &mut visited,
                &map,
                path.direction.turn_right(),
                &path,
            );
        }
    }

    panic!("no path found")
}

fn try_move(
    queue: &mut BinaryHeap<Path>,
    visited: &mut HashSet<String>,
    map: &Vec<Vec<u32>>,
    direction: Direction,
    path: &Path,
) {
    let candidate = Path {
        position: path.position.move_dir(direction),
        direction,
        distance: if direction == path.direction {
            path.distance + 1
        } else {
            1
        },
        heat: 0,
    };

    if candidate.position.row < 0
        || candidate.position.row >= map.len() as i32
        || candidate.position.col < 0
        || candidate.position.col >= map[0].len() as i32
    {
        return;
    }

    let key = format!(
        "{},{},{},{},{}",
        candidate.position.row,
        candidate.position.col,
        candidate.direction.row,
        candidate.direction.col,
        candidate.distance
    );
    if visited.contains(&key) {
        return;
    }

    visited.insert(key);

    let new_heat =
        path.heat + map[candidate.position.row as usize][candidate.position.col as usize];

    queue.push(Path {
        heat: new_heat,
        ..candidate
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let path = find_path(&grid, 0, 3);
    Some(path.heat)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let path = find_path(&grid, 4, 10);
    Some(path.heat)
}

#[derive(Eq, PartialEq)]
struct Path {
    position: Position,
    direction: Direction,
    distance: u32,
    heat: u32,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Direction {
    row: i32,
    col: i32,
}

impl Direction {
    fn turn_left(self) -> Self {
        Direction {
            row: -self.col,
            col: self.row,
        }
    }

    fn turn_right(self) -> Self {
        Direction {
            row: self.col,
            col: -self.row,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn move_dir(self, dir: Direction) -> Self {
        Position {
            row: self.row + dir.row,
            col: self.col + dir.col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
