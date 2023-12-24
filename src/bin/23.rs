use std::collections::{BinaryHeap, HashMap, VecDeque};

use geo::Coord;
use itertools::Itertools;

advent_of_code::solution!(23);

fn reconstruct_path(
    came_from: &HashMap<Coord<u32>, Coord<u32>>,
    end: Coord<u32>,
) -> Vec<Coord<u32>> {
    let mut current = end;
    let mut path = vec![current];
    while let Some(&next) = came_from.get(&current) {
        path.push(next);
        current = next;
    }
    path
}

fn print_path(grid: &Vec<Vec<char>>, path: &Vec<Coord<u32>>) {
    let mut grid = grid.clone();
    for coord in path {
        grid[coord.y as usize][coord.x as usize] = 'O';
    }
    for line in grid {
        println!("{}", line.iter().join(""));
    }
}

fn path_length(came_from: &HashMap<Coord<u32>, Coord<u32>>, end: Coord<u32>) -> u32 {
    let mut current = end;
    let mut path = vec![current];
    while let Some(&next) = came_from.get(&current) {
        path.push(next);
        current = next;
    }
    path.len() as u32
}

fn find_longest_path(
    grid: &Vec<Vec<char>>,
    start: Coord<u32>,
    target: Coord<u32>,
    is_slippery: bool,
) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut came_from = HashMap::new();

    fn manhattan_distance(a: Coord<u32>, b: Coord<u32>) -> u32 {
        ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as u32
    }

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, manhattan_distance(start, target));

    let mut candidates = vec![];

    while !queue.is_empty() {
        let idx = queue
            .iter()
            .enumerate()
            .max_by_key(|x| f_score.get(x.1))
            .unwrap()
            .0;
        let current = queue.remove(idx).unwrap();

        if current == target {
            candidates.push((current, path_length(&came_from, current)));
            continue;
        }

        let offsets = match grid[current.y as usize][current.x as usize] {
            '>' if is_slippery => vec![(1, 0)],
            '<' if is_slippery => vec![(-1, 0)],
            '^' if is_slippery => vec![(0, -1)],
            'v' if is_slippery => vec![(0, 1)],
            _ => vec![(1, 0), (-1, 0), (0, -1), (0, 1)],
        };

        let neighbors = offsets
            .into_iter()
            .map(|(dx, dy)| (current.x as i32 + dx, current.y as i32 + dy))
            .filter(|(x, y)| {
                *x >= 0 && *y >= 0 && *x < grid[0].len() as i32 && *y < grid.len() as i32
            })
            .map(|(x, y)| <Coord<u32>>::from((x as u32, y as u32)));

        for neighbor in neighbors {
            if grid[neighbor.y as usize][neighbor.x as usize] == '#' {
                continue;
            }

            if reconstruct_path(&came_from, current).contains(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap() + 1;

            if let Some(g) = g_score.get(&neighbor) {
                if tentative_g_score <= *g {
                    continue;
                }
            }

            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_g_score);
            f_score.insert(
                neighbor,
                tentative_g_score + manhattan_distance(neighbor, target) * 2,
            );

            queue.push_back(neighbor);
        }
    }

    *candidates.iter().map(|(_, score)| score).max().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let start = (1, 0).into();
    let target = (grid[0].len() as u32 - 2, grid.len() as u32 - 1).into();

    let longest_path = find_longest_path(&grid, start, target, true);

    Some(longest_path - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let start = (1, 0).into();
    let target = (grid[0].len() as u32 - 2, grid.len() as u32 - 1).into();

    let longest_path = find_longest_path(&grid, start, target, false);

    Some(longest_path - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2130));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }

    #[test]
    fn test_solution_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6710));
    }
}
