fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    let matrix = parse_input(input);

    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                for dir in &directions {
                    let mut s = String::new();
                    let mut ii = i;
                    let mut jj = j;
                    for _ in 0..4 {
                        s.push(matrix[ii][jj]);
                        let next_i = ii as i32 + dir.0;
                        let next_j = jj as i32 + dir.1;

                        if next_i < 0 || next_i >= matrix.len() as i32 {
                            break;
                        }

                        if next_j < 0 || next_j >= matrix[0].len() as i32 {
                            break;
                        }

                        ii = next_i as usize;
                        jj = next_j as usize;
                    }

                    if s == "XMAS" {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let matrix = parse_input(input);

    let mut count = 0;

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] == 'A' {
                let upleft = matrix[i - 1][j - 1];
                let upright = matrix[i - 1][j +  1];
                let downleft = matrix[i + 1][j - 1];
                let downright = matrix[i + 1][j + 1];

                println!("{} {} {}", upleft, matrix[i][j], downright);
                println!("{} {} {}", upright, matrix[i][j], downleft);

                let left_cross =
                    upleft == 'M' && downright == 'S' || upleft == 'S' && downright == 'M';

                let right_cross =
                    upright == 'M' && downleft == 'S' || upright == 'S' && downleft == 'M';

                println!("left: {}, right: {}", left_cross, right_cross);
                println!();

                if left_cross && right_cross {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!("../../../input/day4/example.txt");
        assert_eq!(18, part1(input));
        assert_eq!(9, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../../input/day4/input.txt");
        println!("p1: {}", part1(input));
        println!("p2: {}", part2(input));
    }
}
