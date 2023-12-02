use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(2);

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn minimum_set(&self) -> [u32; 3] {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        self.draws.iter().for_each(|draw| {
            red = red.max(draw.red);
            green = green.max(draw.green);
            blue = blue.max(draw.blue);
        });

        [red, green, blue]
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_name, draws) = s.split(": ").collect_tuple().unwrap();
        let game_id = game_name.split(' ').last().unwrap().parse().unwrap();

        let draws = draws
            .split("; ")
            .map(|draw| draw.parse().unwrap())
            .collect();

        Ok(Game { id: game_id, draws })
    }
}

#[derive(Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Draw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut draw = Draw::default();
        s.split(", ").for_each(|part| {
            let (num, color) = part.split(' ').collect_tuple().unwrap();
            let num = num.parse().unwrap();
            match color {
                "red" => draw.red = num,
                "green" => draw.green = num,
                "blue" => draw.blue = num,
                _ => panic!("invalid color"),
            }
        });
        Ok(draw)
    }
}

const LIM_RED: u32 = 12;
const LIM_GREEN: u32 = 13;
const LIM_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(|game| game.parse::<Game>().unwrap());

    let valid_games = games.filter(|game| {
        game.draws
            .iter()
            .all(|draw| draw.red <= LIM_RED && draw.green <= LIM_GREEN && draw.blue <= LIM_BLUE)
    });

    let result = valid_games.map(|game| game.id).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(|game| game.parse::<Game>().unwrap());

    let result = games
        .map(|game| game.minimum_set().into_iter().product::<u32>())
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
