use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(10);

struct PipeGrid {
    grid: Vec<Vec<Pipe>>,
    animal: (usize, usize),
}

impl PipeGrid {
    fn get_pipe(&self, pos: &(usize, usize)) -> &Pipe {
        &self.grid[pos.1][pos.0]
    }

    fn pipe_neighbors(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        self.get_pipe(pos)
            .offsets()
            .iter()
            .map(|(x, y)| (x + pos.0 as isize, y + pos.1 as isize))
            .filter(|(x, y)| {
                x >= &0
                    && y >= &0
                    && *x < self.grid[0].len() as isize
                    && *y < self.grid.len() as isize
            })
            .map(|(x, y)| (x as usize, y as usize))
            .collect_vec()
    }

    fn traverse_loop(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut visited = HashSet::new();
        let mut current = start;
        let mut path = vec![];

        loop {
            let mut available = self
                .pipe_neighbors(&current)
                .into_iter()
                .filter(|p| !visited.contains(p))
                .filter(|p| self.pipe_neighbors(p).contains(&current))
                .collect_vec();

            if available.is_empty() {
                current = path.pop().unwrap();
                continue;
            }

            if available.contains(&start) && path.len() > 1 {
                path.push(current);
                return path;
            }

            if !path.is_empty() {
                if let Some(idx) = available.iter().position(|p| p == path.last().unwrap()) {
                    available.remove(idx);
                }
            }

            let next = available.first().unwrap();
            path.push(current);
            current = *next;
            visited.insert(*next);
        }
    }

    fn count_enclosed_tiles(&self, pipe_loop: Vec<(usize, usize)>) -> u32 {
        let enclosed = (0..self.grid[0].len())
            .cartesian_product(0..self.grid.len())
            .filter(|p| !pipe_loop.contains(p))
            .filter(|p| {
                if p.1 == 3 {
                    println!("{:?}", p);
                }

                let mut intersection_counts =
                    [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().map(|direction| {
                        let mut x = p.0 as isize;
                        let mut y = p.1 as isize;
                        let mut count = 0;

                        while x >= 0
                            && y >= 0
                            && x < self.grid[0].len() as isize
                            && y < self.grid.len() as isize
                        {
                            if pipe_loop.contains(&(x as usize, y as usize)) {
                                count += 1;
                            }

                            x += direction.0;
                            y += direction.1;
                        }

                        count
                    });

                if intersection_counts.clone().any(|c| c == 0) {
                    if p.1 == 3 {
                        println!("{:?}, {:?}", p, intersection_counts.clone().collect_vec());
                    }
                    return false;
                }
                if intersection_counts.clone().all(|c| c != 0) {
                    if p.1 == 3 {
                        println!("{:?}, {:?}", p, intersection_counts.clone().collect_vec());
                    }
                }

                if intersection_counts.clone().all(|c| c % 2 == 1) {
                    if p.1 == 3 {
                        println!("{:?}, {:?}", p, intersection_counts.clone().collect_vec());
                    }
                }

                intersection_counts.any(|c| c % 2 == 1)
            })
            .collect_vec();
        // .count() as u32

        println!("{}", grid_to_string(self, &pipe_loop, &enclosed));

        let enclosed = enclosed
            .iter()
            .filter(|p| {
                // check if all neighbors are either enclosed or part of loop
                let neighbors = [
                    (p.0 - 1, p.1),
                    (p.0 + 1, p.1),
                    (p.0, p.1 - 1),
                    (p.0, p.1 + 1),
                    (p.0 - 1, p.1 - 1),
                    (p.0 + 1, p.1 - 1),
                    (p.0 - 1, p.1 + 1),
                    (p.0 + 1, p.1 + 1),
                ];
                neighbors
                    .iter()
                    .all(|n| enclosed.contains(n) || pipe_loop.contains(n))
            })
            .cloned()
            .collect_vec();

        println!("{}", grid_to_string(self, &pipe_loop, &enclosed));

        enclosed.len() as u32
    }
}

fn pipe_to_char(pipe: &Pipe) -> char {
    match pipe {
        Pipe::Empty => '.',
        Pipe::Horizontal => '-',
        Pipe::Vertical => '|',
        Pipe::BendSW => '7',
        Pipe::BendSE => 'F',
        Pipe::BendNW => 'J',
        Pipe::BendNE => 'L',
        Pipe::Animal => 'S',
        // Add more cases as needed
    }
}

fn grid_to_string(
    grid: &PipeGrid,
    pipe_loop: &Vec<(usize, usize)>,
    enclosed: &Vec<(usize, usize)>,
) -> String {
    let mut output = String::new();
    for (y, row) in grid.grid.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if pipe_loop.contains(&(x, y)) {
                output.push(pipe_to_char(pipe));
            } else if enclosed.contains(&(x, y)) {
                output.push('█');
            } else {
                output.push('░');
            }
        }
        output.push('\n');
    }
    output
}

impl FromStr for PipeGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect_vec())
            .collect_vec();

        let animal = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, pipe)| {
                    if let Pipe::Animal = pipe {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        Ok(Self { grid, animal })
    }
}

enum Pipe {
    Empty,
    Animal,
    Horizontal,
    Vertical,
    BendNE, // L
    BendNW, // J
    BendSE, // F
    BendSW, // 7
}

impl Pipe {
    fn offsets(&self) -> Vec<(isize, isize)> {
        match self {
            Pipe::Empty => vec![],
            Pipe::Animal => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
            Pipe::Horizontal => vec![(-1, 0), (1, 0)],
            Pipe::Vertical => vec![(0, -1), (0, 1)],
            Pipe::BendNE => vec![(0, -1), (1, 0)],
            Pipe::BendNW => vec![(0, -1), (-1, 0)],
            Pipe::BendSE => vec![(0, 1), (1, 0)],
            Pipe::BendSW => vec![(0, 1), (-1, 0)],
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Pipe::Empty),
            'S' => Ok(Pipe::Animal),
            '-' => Ok(Pipe::Horizontal),
            '|' => Ok(Pipe::Vertical),
            'L' => Ok(Pipe::BendNE),
            'J' => Ok(Pipe::BendNW),
            'F' => Ok(Pipe::BendSE),
            '7' => Ok(Pipe::BendSW),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: PipeGrid = input.parse().unwrap();
    let pipe_loop = grid.traverse_loop(grid.animal);
    Some(pipe_loop.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: PipeGrid = input.parse().unwrap();
    let pipe_loop = grid.traverse_loop(grid.animal);
    println!("{:?}", pipe_loop.len());
    let enclosed_tiles = grid.count_enclosed_tiles(pipe_loop);
    Some(enclosed_tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(10));
    }
}
