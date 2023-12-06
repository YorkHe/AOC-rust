use std::{arch::aarch64::float32x2_t, collections::btree_map::Keys, num, u32};

use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../../../input/day6/input.txt");

    let result = part2(input);

    println!("Result: {}", result);
}

fn parse_time(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(tuple((tag("Time:"), space1)), separated_list1(space1, u32))(input)
}

fn parse_distance(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tuple((tag("\nDistance:"), space1)),
        separated_list1(space1, u32),
    )(input)
}

fn part2(input: &str) -> u32 {
    let (input, times) = parse_time(input).unwrap();
    let (input, distances) = parse_distance(input).unwrap();

    assert_eq!(times.len(), distances.len());

    // (time - x) * x >= distance
    // x^2 - time * x + distance <= 0
    //  (time - sqrt(time^2 - 4 * distance)) / 2 <= x <= (time + sqrt(time^2 - 4 * distance)) / 2

    let mut result = 1;

    let sum_time = times
        .iter()
        .map(u32::to_string)
        .fold("".to_owned(), |a, b| a + &b);

    let sum_distance = distances
        .iter()
        .map(u32::to_string)
        .fold("".to_owned(), |a, b| a + &b);

    println!("Sum time: {}", sum_time);
    println!("Sum distance: {}", sum_distance);

    let time = sum_time.parse::<u128>().unwrap();
    let distance = sum_distance.parse::<u128>().unwrap();

    println!("Time: {}, Distance: {}", time, distance);

    println!("Diff: {}", time * time - 4 * distance);

    let lower_bound = (time as f64 - ((time * time - 4 * distance) as f64).sqrt()) / 2.0 + 0.01;
    let upper_bound = (time as f64 + ((time * time - 4 * distance) as f64).sqrt()) / 2.0 - 0.01;

    println!("Lower bound: {}, Upper bound: {}", lower_bound, upper_bound);

    let diff = (upper_bound.floor() as u32) - (lower_bound.ceil() as u32) + 1;

    diff
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = include_str!("../../../../input/day6/example.txt");

        let result = super::part2(input);

        assert_eq!(result, 71503);
    }
}
