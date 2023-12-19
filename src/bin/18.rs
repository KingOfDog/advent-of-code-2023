use std::collections::{BinaryHeap, HashSet, VecDeque};

use geo::Coord;
use itertools::Itertools;

advent_of_code::solution!(18);

#[derive(Debug)]
struct LineSegment {
    start: Coord<i32>,
    end: Coord<i32>,
}

impl LineSegment {
    fn min_x(&self) -> i32 {
        self.start.x.min(self.end.x)
    }

    fn max_x(&self) -> i32 {
        self.start.x.max(self.end.x)
    }

    fn min_y(&self) -> i32 {
        self.start.y.min(self.end.y)
    }

    fn max_y(&self) -> i32 {
        self.start.y.max(self.end.y)
    }
}

struct Ray {
    start: Coord<i32>,
    direction: Direction,
}

enum Direction {
    Right,
    // Down,
}

fn construct_edge(input: &str) -> Vec<LineSegment> {
    let mut current = Coord { x: 0, y: 0 };
    input
        .lines()
        .map(|line| {
            let (direction, distance, color) = line.split_whitespace().collect_tuple().unwrap();
            let distance = distance.parse::<i32>().unwrap();

            let next = match direction {
                "U" => Coord {
                    x: current.x,
                    y: current.y - distance,
                },
                "D" => Coord {
                    x: current.x,
                    y: current.y + distance,
                },
                "L" => Coord {
                    x: current.x - distance,
                    y: current.y,
                },
                "R" => Coord {
                    x: current.x + distance,
                    y: current.y,
                },
                _ => panic!("invalid direction"),
            };

            let line = LineSegment {
                start: current,
                end: next,
                // color: color.to_string(),
            };
            current = next;
            line
        })
        .collect()
}

fn intersects(line: &LineSegment, ray: &Ray) -> i32 {
    match ray.direction {
        Direction::Right => {
            if ray.start.x < line.start.x
                && ray.start.x < line.end.x
                && ray.start.y >= line.min_y()
                && ray.start.y <= line.max_y()
            {
                if line.start.y == line.end.y {
                    -1
                } else {
                    1
                }
            } else {
                0
            }
        } // Direction::Down => {
          //     ray.start.y < line.start.y
          //         && ray.start.y < line.end.y
          //         && ray.start.x >= line.min_x()
          //         && ray.start.x <= line.max_x()
          //         && line.start.x != line.end.x
          // }
    }
}

fn is_inside(lines: &[LineSegment], cur: Coord<i32>) -> bool {
    let ray = Ray {
        start: cur,
        direction: Direction::Right,
    };
    let intersections: i32 = lines.iter().map(|line| intersects(line, &ray)).sum();
    intersections % 2 == 1
    // if intersections % 2 == 1 {
    //     return true;
    // }
}

fn fill_inner(lines: &[LineSegment]) -> HashSet<Coord<i32>> {
    let mut inner = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(Coord { x: 0, y: 0 });

    let min_x = lines.iter().map(|line| line.min_x()).min().unwrap();
    let min_y = lines.iter().map(|line| line.min_y()).min().unwrap();
    let max_x = lines.iter().map(|line| line.max_x()).max().unwrap();
    let max_y = lines.iter().map(|line| line.max_y()).max().unwrap();

    while let Some(cur) = queue.pop_front() {
        if cur.x < min_x || cur.y < min_y || cur.x > max_x || cur.y > max_y {
            continue;
        }
        if inner.contains(&cur) {
            continue;
        }

        if is_inside(lines, cur) {
            inner.insert(cur);
            queue.push_back(Coord {
                x: cur.x + 1,
                y: cur.y,
            });
            queue.push_back(Coord {
                x: cur.x - 1,
                y: cur.y,
            });
            queue.push_back(Coord {
                x: cur.x,
                y: cur.y + 1,
            });
            queue.push_back(Coord {
                x: cur.x,
                y: cur.y - 1,
            });
        }
    }

    inner
}

fn print_grid(grid: &HashSet<Coord<i32>>) {
    let min_x = grid.iter().map(|coord| coord.x).min().unwrap();
    let min_y = grid.iter().map(|coord| coord.y).min().unwrap();
    let max_x = grid.iter().map(|coord| coord.x).max().unwrap();
    let max_y = grid.iter().map(|coord| coord.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid.contains(&Coord { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_grid_2(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn flood_fill(grid: &mut Vec<Vec<bool>>, start: Coord<i32>) {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(cur) = queue.pop_front() {
        if cur.x < 0 || cur.y < 0 || cur.x >= grid[0].len() as i32 || cur.y >= grid.len() as i32 {
            continue;
        }
        if grid[cur.y as usize][cur.x as usize] {
            continue;
        }

        grid[cur.y as usize][cur.x as usize] = true;

        queue.push_back(Coord {
            x: cur.x + 1,
            y: cur.y,
        });
        queue.push_back(Coord {
            x: cur.x - 1,
            y: cur.y,
        });
        queue.push_back(Coord {
            x: cur.x,
            y: cur.y + 1,
        });
        queue.push_back(Coord {
            x: cur.x,
            y: cur.y - 1,
        });
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = construct_edge(input);
    let mut inner = fill_inner(&lines);
    // let mut inner = HashSet::new();

    let min_x = lines.iter().map(|line| line.min_x()).min().unwrap();
    let min_y = lines.iter().map(|line| line.min_y()).min().unwrap();
    let max_x = lines.iter().map(|line| line.max_x()).max().unwrap();
    let max_y = lines.iter().map(|line| line.max_y()).max().unwrap();

    let mut grid = vec![vec![false; (max_x - min_x) as usize + 1]; (max_y - min_y) as usize + 1];

    println!("{} {} {} {}", min_x, min_y, max_x, max_y);

    // add edges to inner
    for ele in lines.iter() {
        for x in ele.min_x()..=ele.max_x() {
            for y in ele.min_y()..=ele.max_y() {
                grid[(y - min_y) as usize][(x - min_x) as usize] = true;
                // inner.insert(Coord { x, y });
            }
        }
    }

    let start = Coord {
        x: lines[0].min_x() - min_x + 1,
        y: lines[1].min_y() - min_y + 1,
    };

    flood_fill(&mut grid, start);

    // print_grid(&inner);
    print_grid_2(&grid);

    let count = grid
        .iter()
        .flat_map(|row| row.iter().filter(|cell| **cell))
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(35401));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
