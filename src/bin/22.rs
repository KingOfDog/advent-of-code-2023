use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use cgmath::Point3;
use geo::Intersects;
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(22);

struct Brick {
    id: u32,
    start: Point3<u32>,
    end: Point3<u32>,
}

impl Brick {
    fn min_z(&self) -> u32 {
        self.start.z.min(self.end.z)
    }

    fn contains(&self, coord: &Point3<u32>) -> bool {
        self.start.x <= coord.x
            && coord.x <= self.end.x
            && self.start.y <= coord.y
            && coord.y <= self.end.y
            && self.start.z <= coord.z
            && coord.z <= self.end.z
    }

    fn cubes(&self) -> Vec<Point3<u32>> {
        let mut cubes = vec![];
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    cubes.push(Point3::new(x, y, z));
                }
            }
        }
        cubes
    }
}

impl Intersects for Brick {
    fn intersects(&self, rhs: &Self) -> bool {
        self.cubes().iter().any(|x| rhs.contains(x))
    }
}

impl FromStr for Brick {
    type Err = ();

    // Parse from schema: x,y,z~x,y,z
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('~');
        let (x, y, z) = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let start = Point3::new(x, y, z);

        let (x, y, z) = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let end = Point3::new(x, y, z);

        Ok(Brick { id: 0, start, end })
    }
}

fn find_supports(input: &str) -> (Vec<Brick>, HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>) {
    let mut falling_bricks = input
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let mut brick = x.parse::<Brick>().unwrap();
            brick.id = i as u32;
            brick
        })
        .collect_vec();
    falling_bricks.sort_by_key(|brick| brick.min_z());

    let mut static_bricks: Vec<Brick> = vec![];

    let mut supports = HashMap::new();
    let mut supported_by = HashMap::new();

    for mut brick in falling_bricks {
        let mut supp = Vec::new();
        while brick.min_z() > 0 {
            brick.start.z -= 1;
            brick.end.z -= 1;

            if static_bricks.par_iter().any(|b| b.intersects(&brick)) {
                supp = static_bricks
                    .iter()
                    .filter(|b| b.intersects(&brick))
                    .map(|b| b.id)
                    .collect();
                brick.start.z += 1;
                brick.end.z += 1;
                break;
            }
        }

        supported_by
            .entry(brick.id)
            .or_insert_with(Vec::new)
            .extend(supp.iter());

        for support in supp {
            supports
                .entry(support)
                .or_insert_with(Vec::new)
                .push(brick.id);
        }
        static_bricks.push(brick);
    }
    (static_bricks, supports, supported_by)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (static_bricks, supports, supported_by) = find_supports(input);

    let result = static_bricks
        .iter()
        .filter(|b| {
            if let Some(supports) = supports.get(&b.id) {
                supports
                    .iter()
                    .all(|supported| supported_by.get(supported).unwrap().len() > 1)
            } else {
                true
            }
        })
        .count();

    Some(result as u32)
}

fn number_of_drops(
    brick: u32,
    supports: &HashMap<u32, Vec<u32>>,
    supported_by: &HashMap<u32, Vec<u32>>,
) -> u32 {
    let mut heads = VecDeque::new();
    heads.push_back(brick);
    let mut removed = HashSet::new();
    removed.insert(brick);

    let mut start = true;

    while let Some(head) = heads.pop_front() {
        let mut own_supports = supported_by.get(&head).cloned().unwrap_or(Vec::new());
        own_supports.retain(|c| !removed.contains(c));

        if own_supports.is_empty() || start {
            removed.insert(head);
            if let Some(sup) = supports.get(&head) {
                heads.extend(sup.iter());
            }
            start = false;
        }
    }

    removed.len() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let (static_bricks, supports, supported_by) = find_supports(input);

    let result = static_bricks
        .iter()
        .map(|b| {
            let r = number_of_drops(b.id, &supports, &supported_by);
            r - 1
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
