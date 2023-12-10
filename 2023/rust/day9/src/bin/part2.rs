use nom::{
    character::complete::{i32, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../../../../input/day9/input.txt");

    let result = part2(input);

    println!("Result: {}", result);
}

fn parse_history(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(line_ending, separated_list1(space1, i32))(input)
}

fn compute_diff(input: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..input.len() - 1 {
        result.push(input[i + 1] - input[i]);
    }

    result
}

fn compute_prev(history: &[i32]) -> i32 {
    let mut firsts: Vec<i32> = Vec::new();
    firsts.push(history[0]);

    let mut current_vec: Vec<i32> = Vec::from(history);
    let mut sign = -1;

    while !(current_vec.iter().all(|&x| x == 0)) {
        let diff = compute_diff(&current_vec);
        firsts.push(sign * diff[0]);
        sign *= -1;
        current_vec = diff;
    }

    firsts.iter().sum()
}

fn part2(input: &str) -> i32 {
    let (_, histories) = parse_history(input).expect("Failed to parse history");

    histories.iter().map(|history| compute_prev(history)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../../../input/day9/example.txt");

        let result = part2(input);

        assert_eq!(result, 2);
    }
}
