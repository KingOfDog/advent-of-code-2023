use itertools::Itertools;

advent_of_code::solution!(13);

fn check_horiz_off(input: &Vec<Vec<char>>, reflect_before: usize, target_diff_count: u32) -> u32 {
    let min_height = (input.len() - reflect_before).min(reflect_before);
    let mut top = input[..reflect_before].iter().rev().take(min_height);
    let mut bottom = input[reflect_before..].iter().take(min_height);

    let mut diff_count = 0;
    while diff_count <= target_diff_count {
        let a = top.next();
        let b = bottom.next();
        if a.is_none() || b.is_none() {
            break;
        }
        if a != b {
            diff_count += a
                .unwrap()
                .iter()
                .zip(b.unwrap().iter())
                .filter(|(a, b)| a != b)
                .count() as u32;
        }
    }
    diff_count
}

fn reflection_score(input: &str, target_diff_count: u32) -> u32 {
    let orig = input;
    let input = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    for y in 1..input.len() {
        if check_horiz_off(&input, y, target_diff_count) == target_diff_count {
            return y as u32 * 100;
        }
    }
    let transposed: Vec<Vec<_>> = (0..input[0].len())
        .map(|col| (0..input.len()).map(|row| input[row][col]).collect())
        .collect();
    for x in 1..transposed.len() {
        if check_horiz_off(&transposed, x, target_diff_count) == target_diff_count {
            return x as u32;
        }
    }
    panic!("No reflection found for {orig}");
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split("\n\n").map(|b| reflection_score(b, 0)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input.split("\n\n").map(|b| reflection_score(b, 1)).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
