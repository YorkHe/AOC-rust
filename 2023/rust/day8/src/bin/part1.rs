use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../../../input/day8/input.txt");

    let result = part1(input);

    println!("Result: {}", result);
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_steps(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

fn parse_nodename(input: &str) -> IResult<&str, &str> {
    take_while_m_n(3, 3, char::is_uppercase)(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let result: IResult<&str, (&str, (&str, &str), &str)> = tuple((
        parse_nodename,
        preceded(
            tag(" = ("),
            separated_pair(parse_nodename, tag(", "), parse_nodename),
        ),
        tag(")"),
    ))(input);

    let (input, (name, (left, right), _)) = result.unwrap();

    Ok((input, Node { name, left, right }))
}

fn parse_nodes<'a>(input: &'a str) -> HashMap<&'a str, Node<'a>> {
    let result = separated_list1(line_ending, parse_node)(input);

    let mut map = HashMap::new();

    for node in result.unwrap().1 {
        map.insert(node.name, node);
    }

    map
}

fn part1(input: &str) -> u32 {
    let split = input.split_once("\n\n").unwrap();

    let steps = parse_steps(split.0.trim());
    let node_map = parse_nodes(split.1.trim());

    let mut cur_node = node_map.get("AAA").expect("No AAA node");
    let mut step_length = 0;
    loop {
        for step in &steps {
            step_length += 1;
            match step {
                'L' => {
                    cur_node = node_map.get(cur_node.left).expect("No left node");
                }
                'R' => {
                    cur_node = node_map.get(cur_node.right).expect("No right node");
                }
                default => panic!("Unknown step: {}", default),
            }

            if cur_node.name == "ZZZ" {
                return step_length;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = include_str!("../../../../input/day8/example.txt");

        let result = super::part1(input);

        assert_eq!(result, 2);
    }
}
