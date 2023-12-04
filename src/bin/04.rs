use itertools::Itertools;

advent_of_code::solution!(4);

fn parse_cards(input: &str) -> Vec<(usize, u32)> {
    input
        .lines()
        .map(|card| {
            let (card, numbers) = card.split_once(": ").unwrap();
            let id = card.split_whitespace().last().unwrap().parse().unwrap();
            let (winning, own) = numbers.split_once(" | ").unwrap();
            let winning = winning
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_vec();

            let matches = own
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .filter(|n| winning.contains(n))
                .count();

            (id, matches as u32)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = parse_cards(input)
        .into_iter()
        .map(|(_, matches)| {
            if matches > 0 {
                2_u32.pow(matches - 1)
            } else {
                0
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_cards(input);
    let mut copies = cards.iter().map(|_| 1).collect_vec();
    let max_card = cards.len() - 1;

    cards.into_iter().for_each(|(id, matches)| {
        let id = id - 1;

        if id < max_card {
            let mut remaining = matches;
            let mut i = id + 1;
            while remaining > 0 {
                copies[i] += copies[id];
                remaining -= 1;

                i += 1;
                if i > max_card {
                    i = id + 1;
                }
            }
        }
    });

    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
