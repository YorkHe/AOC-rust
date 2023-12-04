use std::{collections::HashSet, io::Result};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../../../../input/day4/input.txt");

    let result = part2(input).unwrap();

    println!("Result: {}", result);
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_num: HashSet<u32>,
    got_num: HashSet<u32>,
}

fn nums(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, num_str) = separated_list1(space1, digit1)(input.trim())?;

    let num: Vec<u32> = num_str
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Ok((input, num))
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = preceded(tag("Card"), preceded(space1, digit1))(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, (winning_nums, got_nums)) =
        separated_pair(nums, tag(" | "), nums)(input.trim()).expect("should parse nums");

    Ok((
        input,
        Card {
            id: id.parse::<u32>().unwrap(),
            winning_num: HashSet::from_iter(winning_nums),
            got_num: HashSet::from_iter(got_nums),
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(line_ending, card)(input)?;

    Ok((input, cards))
}

fn part2(input: &str) -> Result<u32> {
    let cards = parse_cards(input).unwrap().1;

    let mut card_multiplier = vec![1; cards.len() + 1];

    for card in cards {
        let mut card_score = 0;
        for num in card.got_num {
            if card.winning_num.contains(&num) {
                card_score += 1;
            }
        }

        for i in 1..=card_score {
            card_multiplier[(card.id + i) as usize] += 1 * card_multiplier[card.id as usize];
        }
    }

    let sum = card_multiplier.iter().sum::<u32>() - 1;

    Ok(sum)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let input = include_str!("../../../../input/day4/example.txt");
        let result = super::part2(input).unwrap();
        assert_eq!(result, 30)
    }
}
