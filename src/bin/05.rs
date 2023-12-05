use std::str::FromStr;

use itermore::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

advent_of_code::solution!(5);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => panic!("invalid category"),
        })
    }
}

struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

struct Mapping {
    from: Category,
    to: Category,
    ranges: Vec<MapRange>,
}

struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Almanac {
    fn map_seeds_to(&self, target: Category) -> impl ParallelIterator<Item = u64> + '_ {
        self.seeds
            .par_iter()
            .map(move |seed| self.map_seed_to(*seed, target))
    }

    fn map_seed_ranges_to(&self, target: Category) -> impl ParallelIterator<Item = u64> + '_ {
        self.seeds
            .iter()
            .array_chunked()
            .par_bridge()
            .flat_map(|[start, len]| (*start..*start + *len))
            .map(move |seed| self.map_seed_to(seed, target))
    }

    fn map_seed_to(&self, seed: u64, target: Category) -> u64 {
        let mut value = seed;
        let mut index = 0;
        let mut current = self.mappings[0].from;
        while current != target {
            let mapping = &self.mappings[index];
            value = mapping.map(value);
            current = mapping.to;
            index += 1;
        }
        value
    }
}

impl Mapping {
    fn map(&self, value: u64) -> u64 {
        self.ranges
            .iter()
            .find(|r| r.includes(value))
            .map_or(value, |r| r.apply(value))
    }
}

impl MapRange {
    fn includes(&self, value: u64) -> bool {
        value >= self.source_start && value < self.source_start + self.length
    }

    fn apply(&self, value: u64) -> u64 {
        value - self.source_start + self.destination_start
    }
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");
        let seeds_line = blocks.next().unwrap();
        let seeds = seeds_line
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();

        let mappings = blocks.map(|block| block.parse().unwrap()).collect();

        Ok(Self { seeds, mappings })
    }
}

impl FromStr for Mapping {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().unwrap();

        let re = Regex::new(r"(.+)-to-(.+) map:").unwrap();
        let captures = re.captures(header).unwrap();
        let from = captures.get(1).unwrap().as_str().parse().unwrap();
        let to = captures.get(2).unwrap().as_str().parse().unwrap();

        let ranges = lines.map(|line| line.parse().unwrap()).collect_vec();

        Ok(Self { from, to, ranges })
    }
}

impl FromStr for MapRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, source_start, length) = s
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Ok(Self {
            destination_start,
            source_start,
            length,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let almanac: Almanac = input.parse().unwrap();
    let lowest_location = almanac.map_seeds_to(Category::Location).min();

    lowest_location
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac: Almanac = input.parse().unwrap();
    let lowest_location = almanac.map_seed_ranges_to(Category::Location).min();

    lowest_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
