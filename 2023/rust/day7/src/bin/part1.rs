use std::{cmp, io::Result};

use lazy_static::lazy_static;
use nom::{
    character::complete::{alphanumeric1, line_ending, space1, u32},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("../../../../input/day7/input.txt");

    let result = part1(input);

    println!("Result: {}", result);
}

lazy_static! {
    static ref CARD_ORDER: Vec<char> =
        vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',];
}

#[derive(Debug)]
struct Round {
    hand: Vec<char>,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_type(input: &Vec<char>) -> Option<Type> {
    let mut counts = [0; 13];

    for car in input {
        let index = CARD_ORDER
            .iter()
            .position(|&c| c == *car)
            .expect("should find card");

        counts[index] += 1;
    }

    counts.sort();

    if counts[12] == 5 {
        return Some(Type::FiveOfAKind);
    } else if counts[12] == 4 {
        return Some(Type::FourOfAKind);
    } else if counts[12] == 3 && counts[11] == 2 {
        return Some(Type::FullHouse);
    } else if counts[12] == 3 {
        return Some(Type::ThreeOfAKind);
    } else if counts[12] == 2 && counts[11] == 2 {
        return Some(Type::TwoPair);
    } else if counts[12] == 2 {
        return Some(Type::OnePair);
    } else {
        return Some(Type::HighCard);
    }
}

impl Round {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let self_type = get_type(&self.hand).expect("should get type");
        let other_type = get_type(&other.hand).expect("should get type");

        if self_type != other_type {
            return self_type.cmp(&other_type);
        }

        for (self_card, other_card) in self.hand.iter().zip(other.hand.iter()) {
            let self_index = CARD_ORDER
                .iter()
                .position(|&c| c == *self_card)
                .expect("should find card");
            let other_index = CARD_ORDER
                .iter()
                .position(|&c| c == *other_card)
                .expect("should find card");

            if self_index != other_index {
                return self_index.cmp(&other_index);
            }
        }

        return cmp::Ordering::Equal;
    }
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }

    let result: IResult<&str, (&str, u32)> = separated_pair(alphanumeric1, space1, u32)(input);

    let (input, (hand, bid)) = result.unwrap();

    let hand = hand.chars().collect();

    Ok((input, Round { hand, bid }))
}

fn parse_rounds(input: &str) -> Result<Vec<Round>> {
    let result: IResult<&str, Vec<Round>> = separated_list1(line_ending, parse_round)(input);

    Ok(result.unwrap().1)
}

fn part1(input: &str) -> u32 {
    let mut rounds = parse_rounds(input).expect("should parse rounds");

    rounds.sort_by(|a, b| b.cmp(&a));
    let mut sum = 0;

    for (index, round) in rounds.iter().enumerate() {
        let score = (index + 1) as u32 * round.bid;
        sum += score;
    }

    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = include_str!("../../../../input/day7/example.txt");

        let result = super::part1(input);
        assert_eq!(result, 6440);
    }
}
