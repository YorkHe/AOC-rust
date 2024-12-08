use std::collections::HashMap;

// Ugly but it works.
// use --release so this can run faster (~2s)

fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let data: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let (mut start_x, mut start_y) = (0, 0);
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == '<' || data[i][j] == '>' || data[i][j] == '^' || data[i][j] == 'v' {
                start_x = i;
                start_y = j;
            }
        }
    }

    (data, (start_x, start_y))
}

fn part1(input: &str) -> u32 {
    let (mut data, (start_x, start_y)) = parse_input(input);

    println!("x: {}, y: {}", start_x, start_y);

    let mut x = start_x;
    let mut y = start_y;

    let next_direction = HashMap::from([('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')]);
    let direction_map =
        HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

    loop {
        let current_direction = data[x][y];

        let next_x = (x as i32 + direction_map[&current_direction].0) as i32;
        let next_y = (y as i32 + direction_map[&current_direction].1) as i32;

        if next_x < 0 || next_x >= data.len() as i32 || next_y < 0 || next_y >= data[0].len() as i32
        {
            // The guard is moving out of the map
            break;
        }

        if data[next_x as usize][next_y as usize] == '#' {
            data[x][y] = next_direction[&current_direction];
        } else {
            data[x][y] = 'X';
            x = next_x as usize;
            y = next_y as usize;
            data[x][y] = current_direction;
        }
    }

    let mut result = 0;
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == 'X' {
                result += 1;
            }
        }
    }

    result + 1
}

fn has_loop(mut data: Vec<Vec<char>>, start_x: usize, start_y: usize) -> bool {
    let mut x = start_x;
    let mut y = start_y;

    let next_direction = HashMap::from([('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')]);
    let direction_map =
        HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);

    let mut steps_taken = 0;

    let max_steps = data.len() * data[0].len();

    loop {
        if steps_taken > max_steps {
            break;
        }

        let current_direction = data[x][y];

        let next_x = (x as i32 + direction_map[&current_direction].0) as i32;
        let next_y = (y as i32 + direction_map[&current_direction].1) as i32;

        if next_x < 0 || next_x >= data.len() as i32 || next_y < 0 || next_y >= data[0].len() as i32
        {
            // The guard is moving out of the map
            return false;
        }

        if data[next_x as usize][next_y as usize] == '#' {
            data[x][y] = next_direction[&current_direction];
        } else {
            data[x][y] = 'X';
            x = next_x as usize;
            y = next_y as usize;
            data[x][y] = current_direction;
            steps_taken += 1;
        }
    }

    println!("HAS LOOP:");
    for line in &data {
        println!("{:?}", String::from_iter(line));
    }
    println!();

    return true;
}

fn part2(input: &str) -> u32 {
    let (data, (start_x, start_y)) = parse_input(input);

    let mut result = 0;
    for x in 0..data.len() {
        for y in 0..data[0].len() {
            if data[x][y] == '.' {
                let mut data_copy = data.clone();
                data_copy[x][y] = '#';
                if has_loop(data_copy, start_x, start_y) {
                    result += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!("../../../input/day6/example.txt");
        assert_eq!(41, part1(input));
        assert_eq!(6, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../../input/day6/input.txt");
        println!("p1: {}", part1(input));
        println!("p2: {}", part2(input));
    }
}
