use std::{collections::BTreeMap, io::Result};

fn main() {
    let input = include_str!("../../../../input/day3/input.txt");

    let result = part2(input).unwrap();
    println!("Result: {}", result);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    part_num: usize,
    x_start: usize,
    x_end: usize,
    y: usize,
}

fn update_parts_map(num: u32, i: usize, j: usize, parts_map: &mut BTreeMap<usize, Vec<Part>>) {
    let part = Part {
        part_num: num as usize,
        x_start: (j - num.to_string().len()),
        x_end: j - 1,
        y: i,
    };

    if parts_map.contains_key(&part.y) {
        parts_map.get_mut(&part.y).unwrap().push(part);
    } else {
        parts_map.insert(part.y, vec![part]);
    }
}

fn part2(input: &str) -> Result<u32> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut parts_map: BTreeMap<usize, Vec<Part>> = BTreeMap::new();
    let mut gears: Vec<(usize, usize)> = vec![];

    for i in 0..map.len() {
        let mut num = 0;
        for j in 0..map[i].len() {
            if map[i][j].is_digit(10) {
                num = num * 10 + map[i][j].to_digit(10).unwrap();
            } else {
                if num != 0 {
                    update_parts_map(num, i, j, &mut parts_map);
                    num = 0;
                }

                if map[i][j] == '*' {
                    gears.push((j, i));
                }
            }
        }

        if num != 0 {
            update_parts_map(num, i, map[i].len(), &mut parts_map);
        }
    }

    let mut total_sum = 0;

    for gear in gears {
        let mut adjacent_count = 0;
        let mut mul_sum = 1;
        for offset in -1..=1 {
            let x: i32 = gear.0 as i32;
            let y: i32 = gear.1 as i32 + offset;

            if x < 0 || y < 0 {
                continue;
            }

            if let Some(parts) = parts_map.get(&(y as usize)) {
                for part in parts {
                    if part.x_start as i32 - 1 == x
                        || part.x_end as i32 + 1 == x
                        || (part.x_start as i32 <= x && part.x_end as i32 >= x)
                    {
                        adjacent_count += 1;
                        mul_sum *= part.part_num;
                    }
                }
            }
        }

        if adjacent_count == 2 {
            total_sum += mul_sum;
        }
    }

    Ok(total_sum as u32)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let input = include_str!("../../../../input/day3/example.txt");
        let result = super::part2(input).unwrap();
        assert_eq!(result, 467835);
    }
}
