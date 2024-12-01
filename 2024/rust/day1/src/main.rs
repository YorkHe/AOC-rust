use std::{collections::HashMap, iter::zip};

fn main() {
    let input = include_str!("../../../input/day1/example.txt");
    part1(&input);
    part2(&input);
}

fn part1(input: &str) -> i32 {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" ").unwrap();
        left_list.push(left.trim().parse::<i32>().unwrap());
        right_list.push(right.trim().parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for (left, right) in zip(left_list, right_list) {
        sum += (left - right).abs();
    }
    println!("sum: {}", sum);

    return sum;
}

fn part2(input: &str) -> i32 {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" ").unwrap();
        left_list.push(left.trim().parse::<i32>().unwrap());
        right_list.push(right.trim().parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for val in right_list {
        *frequency_map.entry(val).or_insert(0) += 1;
    }

    let mut result = 0;

    for val in left_list {
        result += val * frequency_map.get(&val).unwrap_or(&0);
    }

    println!("similarity: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_example() {
        let input = include_str!("../../../input/day1/example.txt");
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 31);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../../input/day1/input.txt");
        part1(&input);
        part2(&input);
    }
}
