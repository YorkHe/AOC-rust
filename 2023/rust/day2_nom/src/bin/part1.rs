use std::{collections::BTreeMap, io::Result};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: usize,
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl Game<'_> {
    fn is_valid(&self, map: &BTreeMap<&str, u32>) -> bool {
        for round in &self.rounds {
            for cube in round {
                if map[cube.color] < cube.amount as u32 {
                    return false;
                }
            }
        }
        return true;
    }
}

// 3 blue, 4 red
fn cubes(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(digit1, tag(" "), alpha1)(input)?;
    Ok((
        input,
        Cube {
            color,
            amount: amount.parse().unwrap(),
        },
    ))
}

// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cubes)(input)?;
    Ok((input, cubes))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { id, rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn main() {
    let input = include_str!("../../../../input/day2.txt");
    let result = part1(input);
    println!("Result: {}", result.unwrap());
}

fn part1(input: &str) -> Result<i32> {
    let map = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = parse_games(input).expect("should parse");
    Ok(games
        .1
        .iter()
        .filter(|g| g.is_valid(&map))
        .map(|g| g.id.parse::<i32>().unwrap())
        .sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../../../input/example.txt");
        let result = part1(input);
        assert_eq!(result.unwrap(), 8);
    }
}
