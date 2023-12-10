use std::default;

fn main() {
    let input = include_str!("../../../../input/day10/input.txt");
    let result = part1(input);
    println!("Result: {}", result);
}

fn part1(input: &str) -> u32 {
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut step_map = vec![vec![0; map[0].len()]; map.len()];

    let mut x = 0;
    let mut y = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                x = i;
                y = j;
            }
        }
    }

    let mut queue = vec![((x, y), 1)];

    let mut max_step = 0;

    while !queue.is_empty() {
        let ((x, y), steps) = queue.remove(0);
        step_map[x][y] = steps;

        if steps > max_step {
            max_step = steps;
        }

        if map[x][y] == 'S' {
            if x + 1 < map.len()
                && (map[x + 1][y] == 'L' || map[x + 1][y] == 'J' || map[x + 1][y] == '|')
            {
                queue.push(((x + 1, y), steps + 1));
            }
            if x > 0 && (map[x - 1][y] == 'F' || map[x - 1][y] == '7' || map[x - 1][y] == '|') {
                queue.push(((x - 1, y), steps + 1));
            }
            if y + 1 < map[0].len()
                && (map[x][y + 1] == 'J' || map[x][y + 1] == '7' || map[x][y + 1] == '-')
            {
                queue.push(((x, y + 1), steps + 1));
            }
            if y > 0 && (map[x][y - 1] == 'L' || map[x][y - 1] == 'F' || map[x][y - 1] == '-') {
                queue.push(((x, y - 1), steps + 1));
            }
        } else {
            match map[x][y] {
                'L' => {
                    if step_map[x - 1][y] == 0 {
                        queue.push(((x - 1, y), steps + 1));
                    } else if step_map[x][y + 1] == 0 {
                        queue.push(((x, y + 1), steps + 1));
                    }
                }
                'F' => {
                    if step_map[x + 1][y] == 0 {
                        queue.push(((x + 1, y), steps + 1));
                    } else if step_map[x][y + 1] == 0 {
                        queue.push(((x, y + 1), steps + 1));
                    }
                }
                'J' => {
                    if step_map[x - 1][y] == 0 {
                        queue.push(((x - 1, y), steps + 1));
                    } else if step_map[x][y - 1] == 0 {
                        queue.push(((x, y - 1), steps + 1));
                    }
                }
                '7' => {
                    if step_map[x + 1][y] == 0 {
                        queue.push(((x + 1, y), steps + 1));
                    } else if step_map[x][y - 1] == 0 {
                        queue.push(((x, y - 1), steps + 1));
                    }
                }
                '|' => {
                    if step_map[x + 1][y] == 0 {
                        queue.push(((x + 1, y), steps + 1));
                    } else if step_map[x - 1][y] == 0 {
                        queue.push(((x - 1, y), steps + 1));
                    }
                }
                '-' => {
                    if step_map[x][y + 1] == 0 {
                        queue.push(((x, y + 1), steps + 1));
                    } else if step_map[x][y - 1] == 0 {
                        queue.push(((x, y - 1), steps + 1));
                    }
                }
                default => {
                    panic!("Unknown char: {}", default);
                }
            }
        }
    }

    step_map.into_iter().for_each(|row| {
        println!("{:?}", row);
    });

    max_step - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = include_str!("../../../../input/day10/example.txt");
        let result = super::part1(input);
        assert_eq!(result, 8);
    }
}
