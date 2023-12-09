use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(9);

fn prepare_rows(seq: Vec<i64>) -> Vec<VecDeque<i64>> {
    let mut rows: Vec<VecDeque<i64>> = Vec::new();
    rows.push(seq.into());

    let mut last_row = &rows[rows.len() - 1];
    while last_row.iter().any(|x| *x != 0) {
        let new_row = last_row
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        rows.push(new_row);
        last_row = &rows[rows.len() - 1];
    }

    rows
}

fn continue_sequence_start(seq: Vec<i64>) -> i64 {
    let mut rows = prepare_rows(seq);

    for i in (0..=rows.len() - 2).rev() {
        let next = rows[i].front().unwrap() - rows[i + 1].front().unwrap();
        rows[i].push_front(next);
    }
    *rows[0].front().unwrap()
}

fn continue_sequence_end(seq: Vec<i64>) -> i64 {
    let mut rows = prepare_rows(seq);

    for i in (0..=rows.len() - 2).rev() {
        let next = rows[i].back().unwrap() + rows[i + 1].back().unwrap();
        rows[i].push_back(next);
    }
    *rows[0].back().unwrap()
}

pub fn part_one(input: &str) -> Option<i64> {
    let result = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
        })
        .map(|s| continue_sequence_end(s))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let result = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
        })
        .map(|s| continue_sequence_start(s))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_a() {
        let result = continue_sequence_end(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_case_b() {
        let result = continue_sequence_start(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
