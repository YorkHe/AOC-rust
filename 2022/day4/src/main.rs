use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

struct Range((i32, i32));

impl Range {
    fn contains(&self, r: &Range) -> bool {
        self.0 .0 <= r.0 .0 && self.0 .1 >= r.0 .1
    }

    fn overlap(&self, r: &Range) -> bool {
        (self.0 .0 <= r.0 .0 && self.0 .1 >= r.0 .0)
            || (self.0 .0 <= r.0 .1 && self.0 .1 >= r.0 .1)
            || self.contains(r)
    }
}

fn parse_range(s: &str) -> Range {
    let mut split = s.splitn(2, "-");
    let start = split.next().unwrap();
    let end = split.next().unwrap();

    Range((start.parse::<i32>().unwrap(), end.parse::<i32>().unwrap()))
}

fn main() {
    let path = Path::new("input.txt");
    let lines = read_lines(path).unwrap();

    let mut total_contains = 0;
    let mut total_overlap = 0;

    for line in lines {
        let line_str = line.unwrap();
        let mut split = line_str.splitn(2, ",");
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        println!("left: {} right: {}", left, right);
        let left_range = parse_range(left);
        let right_range = parse_range(right);

        if left_range.contains(&right_range) || right_range.contains(&left_range) {
            total_contains += 1;
        }

        if left_range.overlap(&right_range) || right_range.overlap(&left_range) {
            total_overlap += 1;
        }
    }

    println!("Total contains are: {}", total_contains);
    println!("Total overlap are: {}", total_overlap);
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
