use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(21);

static mut STEPS: u64 = 64;

fn calculate_distances(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    max_steps: u64,
) -> HashMap<(usize, usize), u64> {
    let mut destinations = HashMap::new();
    let mut heads = VecDeque::new();
    heads.push_back(start);
    destinations.insert(start, 0);

    while let Some((x, y)) = heads.pop_front() {
        if grid[y][x] == '#' {
            continue;
        }

        let cur = destinations[&(x, y)];

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let (nx, ny) = (x as i64 + dx, y as i64 + dy);
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if nx >= grid[y].len() || ny >= grid.len() {
                continue;
            }
            if grid[ny][nx] == '#' {
                continue;
            }
            if destinations.contains_key(&(nx, ny)) {
                continue;
            }
            if cur + 1 > max_steps {
                continue;
            }
            destinations.insert((nx, ny), cur + 1);
            heads.push_back((nx, ny));
        }
    }

    destinations
}

fn find_d(
    g: &Vec<Vec<char>>,
    sr: usize,
    sc: usize,
    r: usize,
    c: usize,
) -> HashMap<(i32, i32, usize, usize), i32> {
    let mut distances: HashMap<(i32, i32, usize, usize), i32> = HashMap::new();
    let mut q: VecDeque<(i32, i32, usize, usize, i32)> = VecDeque::new();
    q.push_back((0, 0, sr, sc, 0));

    while let Some((mut tr, mut tc, mut r, mut c, d)) = q.pop_front() {
        if r < 0 {
            tr -= 1;
            r += r;
        }
        if r >= r {
            tr += 1;
            r -= r;
        }
        if c < 0 {
            tc -= 1;
            c += c;
        }
        if c >= c {
            tc += 1;
            c -= c;
        }
        if !(0 <= r && r < r && 0 <= c && c < c && g[r][c] != '#') {
            continue;
        }
        if distances.contains_key(&(tr, tc, r, c)) {
            continue;
        }
        if tr.abs() > 4 || tc.abs() > 4 {
            continue;
        }
        distances.insert((tr, tc, r, c), d);
        for &(dr, dc) in &[(-1, 0), (0, 1), (1, 0), (0, -1)] {
            q.push_back((tr, tc, r + dr, c + dc, d + 1));
        }
    }

    distances
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    let max_steps = unsafe { STEPS };
    let destinations = calculate_distances(&grid, start, max_steps);
    let result = destinations
        .iter()
        .filter(|(_, &steps)| steps <= max_steps && steps % 2 == max_steps % 2)
        .count() as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid_0 = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = grid_0
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    let sol_1 = calculate_distances(&grid_0, start, unsafe { STEPS });

    // duplicate the grid in every direction
    let mut grid_1 = grid_0.iter().map(|row| row.repeat(3)).collect_vec();
    let mut add = grid_1.clone();
    let mut add2 = grid_1.clone();
    grid_1.append(&mut add);
    grid_1.append(&mut add2);

    let start = (start.0 + grid_0[0].len(), start.1 + grid_0.len());

    let sol_2 = calculate_distances(&grid_1, start, unsafe { STEPS });

    let mut grid_2 = grid_1.iter().map(|row| row.repeat(5)).collect_vec();
    let clone = grid_2.clone();
    for i in 0..5 {
        grid_2.append(&mut clone.clone());
    }

    let start = (start.0 + grid_0[0].len(), start.1 + grid_0.len());

    let sol_3 = calculate_distances(&grid_2, start, unsafe { STEPS });

    println!("{} {} {}", sol_1, sol_2, sol_3);

    let f = |x: i64| -> i64 {
        let b0 = sol_1 as i64;
        let b1 = sol_2 as i64 - sol_1 as i64;
        let b2 = sol_3 as i64 - sol_2 as i64;
        return b0 + b1 * x + (x * (x - 1) / 2) * (b2 - b1);
    };

    let result = f(unsafe { STEPS } as i64 / grid_0.len() as i64);
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        unsafe { STEPS = 6 };
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_solution_one() {
        unsafe { STEPS = 64 };
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        unsafe { STEPS = 6 };
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));

        // unsafe { STEPS = 10 };
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(50));

        unsafe { STEPS = 50 };
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1594));
    }

    #[test]
    fn test_solution_two() {
        unsafe { STEPS = 26501365 };
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(621494544278648));
    }
}
