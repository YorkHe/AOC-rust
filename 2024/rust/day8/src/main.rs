use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get_antinode(coord1: (usize, usize), coord2: (usize, usize)) -> Option<(usize, usize)> {
    if coord1.0 * 2 < coord2.0 || coord1.1 * 2 < coord2.1 {
        return None;
    }
    Some((coord1.0 * 2 - coord2.0, coord1.1 * 2 - coord2.1))
}

fn part1(input: &str) -> usize {
    let mut map = parse_input(input);

    let mut freq_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                continue;
            }

            if !freq_map.contains_key(&map[i][j]) {
                freq_map.insert(map[i][j], Vec::new());
            }

            freq_map.get_mut(&map[i][j]).unwrap().push((i, j));
        }
    }

    println!("{:?}", freq_map);

    for (c, coords) in &freq_map {
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j {
                    continue;
                }

                if let Some((x, y)) = get_antinode(coords[i], coords[j]) {
                    if x < map.len() && y < map[0].len() {
                        map[x][y] = '#'
                    }
                }
            }
        }
    }

    for line in &map {
        println!("{:?}", line);
    }
    println!();

    map.iter().flatten().filter(|c| **c == '#').count()
}

fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    let mut freq_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                continue;
            }

            if !freq_map.contains_key(&map[i][j]) {
                freq_map.insert(map[i][j], Vec::new());
            }

            freq_map.get_mut(&map[i][j]).unwrap().push((i, j));
        }
    }

    for (c, coords) in &freq_map {
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j {
                    continue;
                }

                let (step_x, step_y) = (
                    coords[i].0 as i32 - coords[j].0 as i32,
                    coords[i].1 as i32 - coords[j].1 as i32,
                );

                let mut x = coords[i].0 as i32;
                let mut y = coords[i].1 as i32;

                while x >= 0 && x < map.len() as i32 && y >= 0 && y < map[0].len() as i32 {
                    map[x as usize][y as usize] = '#';
                    x += step_x;
                    y += step_y;
                }
            }
        }
    }

    for line in &map {
        println!("{:?}", line);
    }
    println!();

    map.iter().flatten().filter(|&&c| c != '.').count()
}

#[cfg(test)]
mod tests {
    use crate::{get_antinode, part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!("../../../input/day8/example.txt");
        assert_eq!(14, part1(input));
        assert_eq!(34, part2(input));
    }

    #[test]
    fn test_antinode() {
        assert_eq!(Some((0, 11)), get_antinode((1, 8), (2, 5)));
        assert_eq!(Some((3, 2)), get_antinode((2, 5), (1, 8)));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../../input/day8/input.txt");
        println!("p1: {}", part1(input));
        println!("p2: {}", part2(input));
    }
}
