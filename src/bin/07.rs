use std::str::FromStr;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        let cards = hand.chars().map(|c| c.try_into().unwrap()).collect_vec();
        let hand_type = HandType::determine_type(&cards);
        let bid = bid.parse().unwrap();

        Ok(Self {
            cards,
            hand_type,
            bid,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeSame,
    FullHouse,
    FourSame,
    FiveSame,
}

impl HandType {
    fn determine_type(cards: &[Card]) -> Self {
        let counts = cards
            .iter()
            .sorted()
            .dedup_with_count()
            .sorted_by_key(|(count, _)| *count)
            .collect_vec();
        match counts.as_slice() {
            [(1, _), (1, _), (1, _), (2, _)] => Self::OnePair,
            [(1, _), (2, _), (2, _)] => Self::TwoPair,
            [(1, _), (1, _), (3, _)] => Self::ThreeSame,
            [(2, _), (3, _)] => Self::FullHouse,
            [(1, _), (4, _)] => Self::FourSame,
            [(5, _)] => Self::FiveSame,
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand2 {
    cards: Vec<Card2>,
    hand_type: HandType2,
    bid: u32,
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}

impl FromStr for Hand2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        let cards = hand.chars().map(|c| c.try_into().unwrap()).collect_vec();
        let hand_type = HandType2::determine_type(&cards);
        let bid = bid.parse().unwrap();

        Ok(Self {
            cards,
            hand_type,
            bid,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType2 {
    HighCard,
    OnePair,
    TwoPair,
    ThreeSame,
    FullHouse,
    FourSame,
    FiveSame,
}

impl HandType2 {
    fn determine_type(cards: &[Card2]) -> Self {
        let counts = cards
            .iter()
            .sorted()
            .dedup_with_count()
            .sorted_by_key(|(count, _)| *count)
            .collect_vec();
        match counts.as_slice() {
            [(5, _)] | [(_, Card2::J), (_, _)] | [(_, _), (_, Card2::J)] => Self::FiveSame,
            [(1, _), (4, _)]
            | [(1, Card2::J), (1, _), (3, _)]
            | [(1, _), (_, Card2::J), (_, _)]
            | [(1, _), (_, _), (_, Card2::J)] => Self::FourSame,
            [(2, _), (3, _)] | [(1, Card2::J), (2, _), (2, _)] => Self::FullHouse,
            [(1, _), (1, _), (3, _)]
            | [(1, Card2::J), (1, _), (1, _), (2, _)]
            | [(1, _), (1, _), (1, _), (2, Card2::J)] => Self::ThreeSame,
            [(1, _), (2, _), (2, _)] => Self::TwoPair,
            [(1, _), (1, _), (1, _), (2, _)] | [(1, Card2::J), (1, _), (1, _), (1, _), (1, _)] => {
                Self::OnePair
            }
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card2 {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card2 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => return Err(()),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = input.lines().map(|line| line.parse::<Hand>().unwrap());

    let result = hands
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands = input.lines().map(|line| line.parse::<Hand2>().unwrap());

    let result = hands
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
