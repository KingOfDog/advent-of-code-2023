use std::collections::HashMap;

use cached::proc_macro::cached;
use itertools::Itertools;

advent_of_code::solution!(14);

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|col| (0..grid.len()).map(|row| grid[row][col]).collect())
        .collect()
}

#[cached]
fn tilt_left(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter()
        .map(|col| {
            col.split(|&c| c == '#')
                .map(|s| {
                    let mut new = s.to_vec();
                    new.sort_by_key(|&c| if c == 'O' { 0 } else { 1 });
                    String::from_iter(new)
                })
                .join("#")
                .chars()
                .collect()
        })
        .collect_vec()
}

#[cached]
fn tilt_north(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let transposed = transpose(&grid);
    let transposed = tilt_left(transposed);
    let orig = transpose(&transposed);
    orig
}

#[cached]
fn tilt_west(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    tilt_left(grid)
}

#[cached]
fn tilt_south(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.reverse();
    let mut grid = tilt_north(grid);
    grid.reverse();
    grid
}

#[cached]
fn tilt_east(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter_mut().for_each(|row| row.reverse());
    let mut grid = tilt_west(grid);
    grid.iter_mut().for_each(|row| row.reverse());
    grid
}

fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = tilt_north(grid);
    grid = tilt_west(grid);
    grid = tilt_south(grid);
    grid = tilt_east(grid);
    grid
}

fn total_load(grid: &Vec<Vec<char>>) -> u32 {
    let len = grid.len();
    let result = grid
        .iter()
        .enumerate()
        .map(|(y, row)| row.iter().filter(|&&c| c == 'O').count() * (len - y))
        .sum::<usize>();
    result as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let tilted = tilt_north(grid);

    Some(total_load(&tilted))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut seen = HashMap::new();
    let mut loads = Vec::new();

    const TARGET_CYCLE: u32 = 1_000_000_000;
    for i in 0..TARGET_CYCLE {
        let key = grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .join("\n");

        if let Some(prev_i) = seen.get(&key) {
            let loop_length = i - prev_i;
            let final_cycle = (TARGET_CYCLE - i) % loop_length + prev_i;
            return Some(loads[final_cycle as usize]);
        }

        loads.push(total_load(&grid));
        seen.insert(key, i);

        grid = cycle(grid);
    }

    panic!("Didn't find a loop");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
