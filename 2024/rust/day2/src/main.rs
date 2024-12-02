fn main() {
    let input: &str = include_str!("../../../input/day2/example.txt");
    let lists = parse_input(input);
    assert_eq!(2, part1(&lists));
    assert_eq!(4, part2(&lists));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(lists: &Vec<Vec<i32>>) -> usize {
    lists.iter().filter(|list| list_safe(*list)).count()
}

fn part2(lists: &Vec<Vec<i32>>) -> usize {
    lists
        .iter()
        .filter(|list| {
            if list_safe(*list) {
                true
            } else {
                (0..list.len())
                    .map(|i| {
                        let mut new_list = list.to_vec();
                        new_list.remove(i);
                        new_list
                    })
                    .any(|list| list_safe(&list))
            }
        })
        .count()
}

fn list_safe(list: &Vec<i32>) -> bool {
    (list.windows(2).all(|w| w[0] < w[1]) || list.windows(2).all(|w: &[i32]| w[0] > w[1]))
        && list.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    #[test]
    fn test_example() {
        let input: &str = include_str!("../../../input/day2/example.txt");
        let lists = parse_input(input);
        assert_eq!(2, part1(&lists));
        assert_eq!(4, part2(&lists));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../../input/day2/input.txt");
        let lists = parse_input(input);
        println!("{}", part1(&lists));
        println!("{}", part2(&lists));
    }
}
