use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

struct Workflow {
    rules: Vec<Rule>,
    default_target: String,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",").collect::<Vec<_>>();
        let default_target = parts.pop().unwrap().to_string();
        let re = Regex::new(r"(x|m|a|s)(<|>)(\d+):(.+)").unwrap();
        let rules = parts
            .into_iter()
            .map(|part| {
                let (param, op, val, target) = re
                    .captures(part)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|x| x.unwrap())
                    .collect_tuple()
                    .unwrap();

                let param = param.as_str();
                let val = val.as_str();

                let scheme = match param {
                    "x" => Scheme::X(val.parse().unwrap()),
                    "m" => Scheme::M(val.parse().unwrap()),
                    "a" => Scheme::A(val.parse().unwrap()),
                    "s" => Scheme::S(val.parse().unwrap()),
                    _ => panic!("Unknown param {}", param),
                };

                let operation = match op.as_str() {
                    "<" => Operation::Less,
                    ">" => Operation::Greater,
                    _ => panic!("Unknown operation {}", op.as_str()),
                };

                let target = target.as_str().to_string();

                Rule {
                    scheme,
                    operation,
                    target,
                }
            })
            .collect_vec();

        Ok(Workflow {
            rules,
            default_target,
        })
    }
}

struct Rule {
    scheme: Scheme,
    operation: Operation,
    target: String,
}

enum Scheme {
    X(u64),
    M(u64),
    A(u64),
    S(u64),
}

enum Operation {
    Greater,
    Less,
}

#[derive(Debug, Clone)]
struct AllowedRanges {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, workflow) = line.split_once("{").unwrap();
            (
                name.to_string(),
                Workflow::from_str(&workflow[..workflow.len() - 1]).unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|part| {
            let (x, m, a, s) = (&part[1..part.len() - 1])
                .split(",")
                .map(|c| c.split_once('=').unwrap().1.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Part {
                x,
                m: m,
                a: a,
                s: s,
            }
        })
        .collect_vec();

    (workflows, parts)
}

impl Default for AllowedRanges {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

impl AllowedRanges {
    fn contains(&self, part: &Part) -> bool {
        self.x.contains(&part.x)
            && self.m.contains(&part.m)
            && self.a.contains(&part.a)
            && self.s.contains(&part.s)
    }

    fn possibilities(&self) -> u64 {
        let x = self.x.end() - self.x.start() + 1;
        let m = self.m.end() - self.m.start() + 1;
        let a = self.a.end() - self.a.start() + 1;
        let s = self.s.end() - self.s.start() + 1;

        x * m * a * s
    }
}

fn find_paths(workflows: &HashMap<String, Workflow>) -> Vec<AllowedRanges> {
    let mut paths = vec![];
    let mut heads = vec![("in", AllowedRanges::default())];

    while let Some((id, range)) = heads.pop() {
        if id == "A" {
            paths.push(range);
            continue;
        }

        if id == "R" {
            continue;
        }

        let workflow = &workflows[id];
        let mut default_range = range;

        for rule in &workflow.rules {
            let mut own_range = default_range.clone();

            let (new_constraint, sub_range, default_sub_range) = match rule.scheme {
                Scheme::X(x) => (x, &mut own_range.x, &mut default_range.x),
                Scheme::M(m) => (m, &mut own_range.m, &mut default_range.m),
                Scheme::A(a) => (a, &mut own_range.a, &mut default_range.a),
                Scheme::S(s) => (s, &mut own_range.s, &mut default_range.s),
            };

            match rule.operation {
                Operation::Greater => {
                    if new_constraint > *sub_range.start() {
                        *sub_range = new_constraint + 1..=*sub_range.end();
                        *default_sub_range = *default_sub_range.start()..=new_constraint;
                    }
                }
                Operation::Less => {
                    if new_constraint < *sub_range.end() {
                        *sub_range = *sub_range.start()..=new_constraint - 1;
                        *default_sub_range = new_constraint..=*default_sub_range.end();
                    }
                }
            }

            heads.push((&rule.target, own_range));
        }

        heads.push((&workflow.default_target, default_range));
    }

    paths
}

pub fn part_one(input: &str) -> Option<u64> {
    let (workflows, parts) = parse_input(input);

    let paths = find_paths(&workflows);

    let accepted = parts
        .par_iter()
        .filter(|part| paths.iter().any(|path| path.contains(part)));

    let result = accepted
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u64>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (workflows, _) = parse_input(input);

    let paths = find_paths(&workflows);

    let result = paths
        .par_iter()
        .map(|path| path.possibilities())
        .sum::<u64>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(421983));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }

    #[test]
    fn test_solution_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(129249871135292));
    }
}
