use std::iter::once;

use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(12);

#[cached]
fn count_valid_solutions(line: Vec<char>, runs: Vec<usize>) -> u64 {
    if line.is_empty() {
        if runs.is_empty() {
            return 1;
        }
        return 0;
    }
    if runs.is_empty() {
        if line.iter().any(|&c| c == '#') {
            return 0;
        }
        return 1;
    }

    let min_remaining_len = runs.iter().sum::<usize>() + runs.len() - 1;
    if line.len() < min_remaining_len {
        return 0;
    }

    if line[0] == '.' {
        return count_valid_solutions(line[1..].to_vec(), runs);
    }
    if line[0] == '#' {
        let (run, leftover_runs) = runs.split_at(1);
        let run = run[0];
        if line[..run].iter().any(|&c| c == '.') {
            return 0;
        }
        if line.len() == run {
            if leftover_runs.is_empty() {
                return 1;
            }
            return 0;
        }
        if line.get(run) == Some(&'#') {
            return 0;
        }
        return count_valid_solutions(line[run + 1..].to_vec(), leftover_runs.to_vec());
    }

    // Otherwise dunno first spot, pick
    let left = count_valid_solutions(
        once('#').chain(line[1..].iter().cloned()).collect_vec(),
        runs.clone(),
    );
    let right = count_valid_solutions(
        once('.').chain(line[1..].iter().cloned()).collect_vec(),
        runs,
    );
    left + right
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .par_lines()
        .map(|line| {
            let (line, groups) = line.split_once(' ').unwrap();
            let line = line.chars().collect_vec();
            let groups = groups
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            (line, groups)
        })
        .map(|(line, groups)| count_valid_solutions(line, groups))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input
        .par_lines()
        .map(|line| {
            let (line, groups) = line.split_once(' ').unwrap();
            let mut line = line
                .chars()
                .chain(std::iter::once('?'))
                .collect_vec()
                .repeat(5);
            line.pop();
            let groups = groups
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec()
                .repeat(5);
            (line, groups)
        })
        .map(|(line, groups)| count_valid_solutions(line, groups))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }

    #[test]
    fn test_solution_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(17391848518844));
    }
}
