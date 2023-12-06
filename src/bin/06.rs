use itertools::Itertools;

advent_of_code::solution!(6);

fn parse_races_part1(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    let mut iter = input
        .lines()
        .map(|line| line.split_whitespace().skip(1).map(|n| n.parse().unwrap()));
    iter.next().unwrap().zip(iter.next().unwrap())
}

fn parse_races_part2(input: &str) -> (u64, u64) {
    let iter = input.lines().map(|line| {
        line.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });
    iter.collect_tuple().unwrap()
}

fn winning_options((time, dist): (u64, u64)) -> u64 {
    let first_victory = (1..time).find(|i| (time - i) * i > dist).unwrap();
    // the winning function is beautifully symmetric, so if we know the point where we
    // start winning (first_victory), we also know the end (time - first_victory)
    time + 1 - (first_victory) * 2
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_races_part1(input);
    races.map(|race| winning_options(race)).product1()
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_races_part2(input);
    Some(winning_options(race))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
