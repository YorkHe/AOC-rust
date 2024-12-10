fn main() {
    println!("Hello, world!");
}

fn format_input(input: &str) -> Vec<(u64, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(":")
                .map(|(left, right)| {
                    (
                        left.trim().parse::<u64>().unwrap(),
                        right
                            .trim()
                            .split(" ")
                            .map(|n| n.parse::<u32>().unwrap())
                            .collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn dfs(target: &u64, current_sum: u64, index: usize, operands: &Vec<u32>) -> bool {
    if index == operands.len() {
        if current_sum == *target {
            return true;
        }
        return false;
    }

    dfs(
        target,
        current_sum + operands[index] as u64,
        index + 1,
        operands,
    ) || dfs(
        target,
        current_sum * operands[index] as u64,
        index + 1,
        operands,
    )
}

fn is_valid(num: &u64, operands: &Vec<u32>) -> bool {
    dfs(num, operands[0] as u64, 1, operands)
}

fn part1(input: &str) -> u64 {
    let data = format_input(input);

    data.iter()
        .filter(|e| is_valid(&e.0, &e.1))
        .map(|e| e.0)
        .sum()
}

fn dfs2(target: &u64, current_sum: u64, index: usize, operands: &Vec<u32>) -> bool {
    if index == operands.len() {
        if current_sum == *target {
            return true;
        }
        return false;
    }

    dfs2(
        target,
        current_sum + operands[index] as u64,
        index + 1,
        operands,
    ) || dfs2(
        target,
        current_sum * operands[index] as u64,
        index + 1,
        operands,
    ) || dfs2(
        target,
        current_sum * u64::pow(10, operands[index].to_string().len() as u32)
            + operands[index] as u64,
        index + 1,
        operands,
    )
}

fn is_valid_concat(num: &u64, operands: &Vec<u32>) -> bool {
    dfs2(num, operands[0] as u64, 1, operands)
}

fn part2(input: &str) -> u64 {
    let data = format_input(input);

    data.iter()
        .filter(|e| is_valid_concat(&e.0, &e.1))
        .map(|e: &(u64, Vec<u32>)| e.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{is_valid_concat, part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!("../../../input/day7/example.txt");
        assert_eq!(3749, part1(input));
        assert_eq!(11387, part2(input));
    }

    #[test]
    fn test_156() {
        assert!(is_valid_concat(&7290, &vec![6, 8, 6, 15]));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../../input/day7/input.txt");
        println!("p1: {}", part1(input));
        println!("p2: {}", part2(input));
    }
}
