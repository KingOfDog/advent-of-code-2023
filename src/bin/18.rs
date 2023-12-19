use core::panic;

use geo::Coord;

advent_of_code::solution!(18);

struct Grid {
    nodes: Vec<Coord<i64>>,
    boundary_count: usize,
}

fn parse_direction(dir: &str, steps: i64) -> Coord<i64> {
    match dir {
        "R" => Coord { x: steps, y: 0 },
        "U" => Coord { x: 0, y: steps },
        "L" => Coord { x: -steps, y: 0 },
        "D" => Coord { x: 0, y: -steps },
        _ => panic!("can't find dir!"),
    }
}

fn parse_direction_from_number(dir: usize, steps: i64) -> Coord<i64> {
    match dir {
        0 => Coord { x: steps, y: 0 },
        1 => Coord { x: 0, y: steps },
        2 => Coord { x: -steps, y: 0 },
        3 => Coord { x: 0, y: -steps },
        _ => panic!("can't find dir!"),
    }
}

impl Grid {
    fn new_part_1(contents: &str) -> Self {
        let mut nodes = vec![];

        let lines = contents.lines();
        let mut cur = Coord { x: 0, y: 0 };
        let mut boundary_count: usize = 0;

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let dir = parts[0].to_owned();
            let steps = parts[1].parse::<i64>().expect("steps are a number");

            let inc = parse_direction(&dir, steps);

            boundary_count += steps as usize;
            cur = cur + inc;
            nodes.push(cur);
        }

        Self {
            nodes,
            boundary_count,
        }
    }

    fn new_part_2(contents: &str) -> Self {
        let mut nodes = vec![];

        let lines = contents.lines();
        let mut cur = Coord { x: 0, y: 0 };
        let mut boundary_count: usize = 0;

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let color = parts[2];
            let dir = usize::from_str_radix(&color[7..8], 16).expect("direction to be a number");
            let steps =
                usize::from_str_radix(&color[2..7], 16).expect("steps to be a number") as i64;

            let inc = parse_direction_from_number(dir, steps);

            boundary_count += steps as usize;
            cur = cur + inc;
            nodes.push(cur);
        }

        Self {
            nodes,
            boundary_count,
        }
    }

    // shoelace formula
    fn area(&self) -> i64 {
        let mut points = self.nodes.clone();

        // make sure it's closed
        points.push(points[0]);

        // 2A = (x1 * y2 - y1 * x2) + (x2 * y3...)...

        let sum: i64 = points
            .windows(2)
            .map(|p| p[0].x * p[1].y - p[0].y * p[1].x)
            .sum();

        let boundary_count = self.boundary_count as i64;

        sum.abs() / 2 + boundary_count / 2 + 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new_part_1(input);
    let result = grid.area();
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new_part_2(input);
    let result = grid.area();
    Some(result as u64)
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

    #[test]
    fn test_solution_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(48020869073824));
    }
}
