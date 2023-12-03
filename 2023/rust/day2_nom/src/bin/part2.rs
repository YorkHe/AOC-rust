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
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    rounds: Vec<Vec<Cube<'a>>>,
}

impl Game<'_> {
    fn get_score(&self) -> u32 {
        let mut map = BTreeMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for round in &self.rounds {
            for cube in round {
                if map[cube.color] < cube.amount {
                    map.insert(cube.color, cube.amount);
                }
            }
        }

        map.values().fold(1, |acc, f| acc * f)
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
    let (input, _id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn main() {
    let input = include_str!("../../../../input/day2.txt");
    let result = part2(input);
    println!("Result: {}", result.unwrap());
}

fn part2(input: &str) -> Result<i32> {
    let games = parse_games(input).expect("should parse");
    let result = games.1.iter().map(|g| g.get_score()).sum::<u32>();

    Ok(result as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../input/example.txt");
        let result = part2(input);
        assert_eq!(result.unwrap(), 2286);
    }
}
