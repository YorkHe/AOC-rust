use std::{arch::aarch64::float32x2_t, collections::btree_map::Keys, u32};

use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../../../input/day6/input.txt");

    let result = part1(input);

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

fn part1(input: &str) -> u32 {
    let (input, times) = parse_time(input).unwrap();
    let (input, distances) = parse_distance(input).unwrap();

    assert_eq!(times.len(), distances.len());

    // (time - x) * x >= distance
    // x^2 - time * x + distance <= 0
    //  (time - sqrt(time^2 - 4 * distance)) / 2 <= x <= (time + sqrt(time^2 - 4 * distance)) / 2

    let mut result = 1;

    for (time, distance) in times.iter().zip(distances.iter()) {
        println!("Time: {}, Distance: {}", time, distance);

        let lower_bound =
            (*time as f32 - ((time * time - 4 * distance) as f32).sqrt()) / 2.0 + 0.01;
        let upper_bound =
            (*time as f32 + ((time * time - 4 * distance) as f32).sqrt()) / 2.0 - 0.01;

        println!("Lower bound: {}, Upper bound: {}", lower_bound, upper_bound);

        let diff = (upper_bound.floor() as u32) - (lower_bound.ceil() as u32) + 1;

        println!("Diff: {}", diff);
        println!();

        result *= diff;
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = include_str!("../../../../input/day6/example.txt");

        let result = super::part1(input);

        assert_eq!(result, 288);
    }
}
