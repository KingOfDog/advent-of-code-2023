use core::panic;

use geo::Coord;
use itertools::Itertools;
use rayon::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(18);

#[derive(Debug)]
struct LineSegment {
    start: Coord<i64>,
    end: Coord<i64>,
    direction: Direction,
}

impl LineSegment {
    fn min_x(&self) -> i64 {
        self.start.x.min(self.end.x)
    }

    fn max_x(&self) -> i64 {
        self.start.x.max(self.end.x)
    }

    fn min_y(&self) -> i64 {
        self.start.y.min(self.end.y)
    }

    fn max_y(&self) -> i64 {
        self.start.y.max(self.end.y)
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn construct_edge_1(input: &str) -> Vec<LineSegment> {
    let mut current = Coord { x: 0, y: 0 };
    input
        .lines()
        .map(|line| {
            let (direction, distance, _) = line.split_whitespace().collect_tuple().unwrap();
            let distance = distance.parse::<i64>().unwrap();

            let direction = direction.parse().unwrap();

            let next = match direction {
                Direction::Up => Coord {
                    x: current.x,
                    y: current.y - distance,
                },
                Direction::Down => Coord {
                    x: current.x,
                    y: current.y + distance,
                },
                Direction::Left => Coord {
                    x: current.x - distance,
                    y: current.y,
                },
                Direction::Right => Coord {
                    x: current.x + distance,
                    y: current.y,
                },
            };

            let line = LineSegment {
                start: current,
                end: next,
                direction: direction,
            };
            current = next;
            line
        })
        .collect()
}

fn construct_edge_2(input: &str) -> Vec<LineSegment> {
    let mut current = Coord { x: 0, y: 0 };
    input
        .lines()
        .map(|line| {
            let (_, _, hex) = line.split_whitespace().collect_tuple().unwrap();
            let distance = i64::from_str_radix(&hex[2..7], 16).unwrap();

            let direction = match &hex[7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("invalid direction"),
            };

            let next = match direction {
                Direction::Up => Coord {
                    x: current.x,
                    y: current.y - distance,
                },
                Direction::Down => Coord {
                    x: current.x,
                    y: current.y + distance,
                },
                Direction::Left => Coord {
                    x: current.x - distance,
                    y: current.y,
                },
                Direction::Right => Coord {
                    x: current.x + distance,
                    y: current.y,
                },
            };

            let line = LineSegment {
                start: current,
                end: next,
                direction: direction,
            };
            current = next;
            line
        })
        .collect()
}

fn measure_inner_area(lines: &[LineSegment]) -> u64 {
    let min_x = lines.iter().map(|line| line.min_x()).min().unwrap();
    let min_y = lines.iter().map(|line| line.min_y()).min().unwrap();
    let max_x = lines.iter().map(|line| line.max_x()).max().unwrap();
    let max_y = lines.iter().map(|line| line.max_y()).max().unwrap();

    println!("{}", max_y - min_y);

    let result = (min_y..=max_y)
        .par_bridge()
        .map(|y| {
            println!("{:?}", y);
            let mut inside = false;
            let mut inside_count = 0;
            let mut prev_vert_line: Option<&LineSegment> = None;
            for x in min_x..=max_x {
                let line = lines.iter().find(|line| {
                    x >= line.min_x()
                        && x <= line.max_x()
                        && y >= line.min_y()
                        && y <= line.max_y()
                        && line.start.y != line.end.y
                });
                let find = lines.iter().find(|line| {
                    x >= line.min_x() && x <= line.max_x() && y == line.min_y() && y == line.max_y()
                });
                if inside || line.is_some() || find.is_some() {
                    inside_count += 1;
                }

                if let Some(line) = line {
                    if let Some(prev_line) = prev_vert_line {
                        if find.is_some() {
                            if prev_line.direction == line.direction {
                                inside = !inside;
                            }
                        } else {
                            inside = !inside;
                        }
                    } else if find.is_none() {
                        inside = !inside;
                    }
                    prev_vert_line = Some(line);
                }
            }
            inside_count
        })
        .sum();

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = construct_edge_1(input);

    Some(measure_inner_area(&lines))
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = construct_edge_2(input);

    Some(measure_inner_area(&lines))
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
