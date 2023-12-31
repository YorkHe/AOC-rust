use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space1, u64},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../../../input/day5/input.txt");

    let result = part1(input);

    println!("Result: {}", result);
}

type MapTuple = ((u64, u64), (u64, u64));

#[derive(Debug)]
struct Map {
    maps: Vec<MapTuple>,
}

impl Map {
    fn get(&self, from: u64) -> u64 {
        for map in &self.maps {
            if from >= map.0 .0 && from < map.0 .1 {
                return map.1 .0 + (from - map.0 .0);
            }
        }

        from
    }
}

fn parse_seed(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, u64)(input)
}

fn parse_maps(input: &str) -> IResult<&str, Map> {
    let result: IResult<&str, _> =
        delimited(alpha1, tag("-to-"), tuple((alpha1, tag(" map:\n"))))(input);

    let (input, _) = result.unwrap();

    let result: IResult<&str, Vec<Vec<u64>>> =
        separated_list1(tag("\n"), separated_list1(space1, u64))(input);

    let (input, map_vals) = result.unwrap();

    let mut value_maps: Vec<MapTuple> = vec![];

    for val in map_vals {
        value_maps.push(((val[1], val[1] + val[2]), (val[0], val[0] + val[2])));
    }

    Ok((input, Map { maps: value_maps }))
}

fn part1(input: &str) -> u64 {
    let (input, seeds) = preceded(tag("seeds: "), parse_seed)(input).unwrap();

    let (_input, maps) = separated_list1(tag("\n\n"), parse_maps)(input.trim()).unwrap();

    seeds
        .iter()
        .map(|seed| {
            let mut val = *seed;
            for map in &maps {
                val = map.get(val);
            }
            val
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = include_str!("../../../../input/day5/example.txt");
        let result = super::part1(input);
        assert_eq!(result, 35);
    }
}
