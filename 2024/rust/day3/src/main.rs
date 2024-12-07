use regex::Regex;

// Reference: https://www.reddit.com/r/adventofcode/comments/1h5frsp/comment/m0equ1o/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
// Note: It's a pretty elegant solution using the regex. Actually better than the nom.
// The nom parser is better for total matching while the regex is better for extracting useful information from a string.

fn main() {
    let input = include_str!("../../../input/day3/example.txt");
    assert_eq!(161, part1(input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for (_, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        println!("x: {}, y: {}", x, y);
        sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
    }

    sum
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for (s, [x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        println!("s: {}, x: {}, y: {}", s, x, y);
        match s {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap() * enabled as i32,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1() {
        let input = include_str!("../../../input/day3/example.txt");
        assert_eq!(161, part1(input));
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../../../input/day3/example2.txt");
        assert_eq!(48, part2(input));
    }
    #[test]
    fn test_input() {
        let input = include_str!("../../../input/day3/input.txt");

        println!("Part1: {}", part1(input));
        println!("Part2: {}", part2(input));
    }
}
