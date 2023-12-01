use core::panic;

use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_digit(10)).unwrap();
            let last = line.chars().rfind(|c| c.is_digit(10)).unwrap();

            u32::from_str_radix(&format!("{first}{last}"), 10).unwrap()
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let result = input
        .lines()
        .map(|line| {
            let first = re.find(line).unwrap().as_str();

            let mut last = None;
            let mut search_index = line.len() - 1;
            while last.is_none() {
                last = re.find_at(line, search_index);
                if search_index > 0 {
                    search_index -= 1;
                }
            }

            let last = last.unwrap().as_str();

            let first = parse_number_text(first);
            let last = parse_number_text(last);

            first * 10 + last
        })
        .sum();

    Some(result)
}

fn parse_number_text(input: &str) -> u32 {
    if let Ok(value) = u32::from_str_radix(input, 10) {
        return value;
    }
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples",
            DAY,
            Some(1),
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples",
            DAY,
            Some(2),
        ));
        assert_eq!(result, Some(281));
    }
}
