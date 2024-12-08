use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (order_str, page_str) = input.split_once("\n\n").unwrap();

    let orders = order_str
        .lines()
        .map(|s| {
            let (left, right) = s.split_once("|").unwrap();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect();

    let pages = page_str
        .lines()
        .map(|s| s.split(',').map(|c| c.parse::<u32>().unwrap()).collect())
        .collect();

    (orders, pages)
}

fn part1(input: &str) -> u32 {
    let (orders, books) = parse_input(input);

    let mut order_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (x, y) in orders {
        if !order_map.contains_key(&x) {
            order_map.insert(x.clone(), HashSet::new());
        }

        order_map.get_mut(&x).unwrap().insert(y);
    }

    println!("{:?}", order_map);

    let mut result = 0;

    for book in books {
        let mut is_valid = true;
        'outer: for i in 1..book.len() {
            for j in 0..i {
                if let Some(set) = order_map.get(&book[i]) {
                    if set.contains(&book[j]) {
                        is_valid = false;
                        break 'outer;
                    }
                }
            }
        }

        if is_valid {
            result += book[book.len() / 2];
            println!("{:?}", book);
        }
    }

    result
}

fn part2(input: &str) -> u32 {
    let (orders, books) = parse_input(input);

    let mut order_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (x, y) in orders {
        if !order_map.contains_key(&x) {
            order_map.insert(x.clone(), HashSet::new());
        }

        order_map.get_mut(&x).unwrap().insert(y);
    }

    println!("{:?}", order_map);

    let mut result = 0;

    for book in books {
        let mut is_valid = true;
        'outer: for i in 1..book.len() {
            for j in 0..i {
                if let Some(set) = order_map.get(&book[i]) {
                    if set.contains(&book[j]) {
                        is_valid = false;
                        break 'outer;
                    }
                }
            }
        }

        if !is_valid {
            let mut new_book = book.clone();
            new_book.sort_by(|a, b| {
                if a == b {
                    Ordering::Equal
                } else {
                    let a_larger = order_map.get(a).map_or(false, |s| s.contains(b));
                    let b_larger = order_map.get(b).map_or(false, |s| s.contains(a));

                    if a_larger {
                        Ordering::Greater
                    } else if b_larger {
                        Ordering::Less
                    } else {
                        Ordering::Less
                    }
                }
            });

            println!("{:?} -> {:?}", book, new_book);

            result += new_book[new_book.len() / 2];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example() {
        let input = include_str!("../../input/day5/example.txt");
        assert_eq!(143, part1(input));
        assert_eq!(123, part2(input));
    }

    #[test]
    fn test_input() {
        let input = include_str!("../../input/day5/input.txt");
        println!("p1: {}", part1(input));
        println!("p2: {}", part2(input));
    }
}
