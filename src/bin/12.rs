use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(12);

fn count_valid_solutions(line: &[char], groups: &[u32]) -> u32 {
    if groups.is_empty() {
        return 1;
    }
    if line.iter().all(|c| *c == '.') {
        return 0;
    }
    let t = groups[0];
    let first_spring_start = line
        .iter()
        .find_position(|c| **c == '#')
        .map(|(i, _)| i)
        .unwrap_or(line.len());

    let result = line[0..(first_spring_start + t as usize).min(line.len())]
        .par_windows(t as usize)
        .enumerate()
        .map(|(i, w)| {
            let mut valid = w.iter().all(|c| *c != '.');
            if i + (t as usize) < line.len() && line[i + t as usize] == '#' {
                valid = false;
            }
            if valid {
                let mut next_start = i + t as usize;
                if next_start < line.len() {
                    next_start += 1;
                }
                count_valid_solutions(&line[next_start..], &groups[1..])
            } else {
                0
            }
        })
        .sum();
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .par_lines()
        .map(|line| {
            let (line, groups) = line.split_once(' ').unwrap();
            let line = line.chars().collect_vec();
            let groups = groups
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec();
            (line, groups)
        })
        .map(|(line, groups)| count_valid_solutions(&line, &groups))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
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
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
                .repeat(5);
            (line, groups)
        })
        .map(|(line, groups)| count_valid_solutions(&line, &groups))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_num_valid_solutions() {
        let result = count_valid_solutions(&"?###????????".chars().collect_vec(), &vec![3, 2, 1]);
        assert_eq!(result, 10);

        let result = count_valid_solutions(&"???.###".chars().collect_vec(), &vec![1, 1, 3]);
        assert_eq!(result, 1);

        let result = count_valid_solutions(&".??..??...?##.".chars().collect_vec(), &vec![1, 1, 3]);
        assert_eq!(result, 4);

        let result =
            count_valid_solutions(&"?#?#?#?#?#?#?#?".chars().collect_vec(), &vec![1, 3, 1, 6]);
        assert_eq!(result, 1);

        let result = count_valid_solutions(&"????.#...#...".chars().collect_vec(), &vec![4, 1, 1]);
        assert_eq!(result, 1);

        let result =
            count_valid_solutions(&"????.######..#####.".chars().collect_vec(), &vec![1, 6, 5]);
        assert_eq!(result, 4);
    }

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
}
