use std::{collections::HashMap, iter::zip};

fn main() {
    let input = include_str!("../../../input/day1/example.txt");
    let (left_list, right_list) = parse_input(input);

    part1(&left_list, &right_list);
    part2(&left_list, &right_list);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left_list, mut right_list) = (Vec::new(), Vec::new());

    for line in input.lines() {
        if let Some((left, right)) = line.split_once(" ") {
            left_list.push(left.trim().parse().unwrap());
            right_list.push(right.trim().parse().unwrap());
        }
    }

    left_list.sort();
    right_list.sort();
    (left_list, right_list)
}

fn part1(left_list: &[i32], right_list: &[i32]) -> i32 {
    let sum: i32 = zip(left_list, right_list)
        .map(|(left, right)| (left - right).abs())
        .sum();
    println!("sum: {}", sum);
    sum
}

fn part2(left_list: &[i32], right_list: &[i32]) -> i32 {
    let frequency_map = right_list.iter().fold(HashMap::new(), |mut map, &val| {
        *map.entry(val).or_insert(0) += 1;
        map
    });

    let result: i32 = left_list
        .iter()
        .map(|&val| val * frequency_map.get(&val).unwrap_or(&0))
        .sum();
    println!("similarity: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    #[test]
    fn test_example() {
        let input: &str = include_str!("../../../input/day1/example.txt");
        let (left_list, right_list) = parse_input(input);

        part1(&left_list, &right_list);
        part2(&left_list, &right_list);
    }

    #[test]
    fn test_input() {
        let input: &str = include_str!("../../../input/day1/input.txt");
        let (left_list, right_list) = parse_input(input);

        part1(&left_list, &right_list);
        part2(&left_list, &right_list);
    }
}
