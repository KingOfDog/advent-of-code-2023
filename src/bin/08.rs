use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(8);

struct Map {
    operations: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn traverse(&self, start: String, goal: String) -> u64 {
        let mut current = start;
        let mut steps = 0;

        while current != goal {
            let (left, right) = self.nodes.get(&current).unwrap();
            let direction = &self.operations[steps as usize % self.operations.len()];

            current = match direction {
                Direction::Left => left.clone(),
                Direction::Right => right.clone(),
            };

            steps += 1;
        }

        steps
    }

    fn traverse_ghost(&self) -> u64 {
        let mut current = self
            .nodes
            .keys()
            .filter(|n| n.ends_with('A'))
            .cloned()
            .collect_vec();
        let mut steps = current.iter().map(|_| 0).collect_vec();

        while !current.iter().all(|n| n.ends_with('Z')) {
            for (current_node, steps) in current.iter_mut().zip(steps.iter_mut()) {
                if !current_node.ends_with('Z') {
                    let (left, right) = self.nodes.get(current_node).unwrap();
                    let direction = &self.operations[*steps as usize % self.operations.len()];

                    *current_node = match direction {
                        Direction::Left => left.clone(),
                        Direction::Right => right.clone(),
                    };

                    *steps += 1;
                }
            }
        }

        steps.iter().fold(1, |acc, x| lcm(acc, *x))
    }
}

/// Least common multiple
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

/// Greatest common denominator
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let operations = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect();
        lines.next().unwrap();

        let mut nodes = HashMap::new();

        let re = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();
        lines.for_each(|line| {
            let captures = re.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            let left = captures.get(2).unwrap().as_str().to_string();
            let right = captures.get(3).unwrap().as_str().to_string();

            nodes.insert(name, (left, right));
        });

        Ok(Self { operations, nodes })
    }
}

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.parse::<Map>().unwrap();
    Some(map.traverse("AAA".to_string(), "ZZZ".to_string()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.parse::<Map>().unwrap();
    Some(map.traverse_ghost())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
