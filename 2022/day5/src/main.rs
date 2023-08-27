use std::{fs, io, path::Path};

fn main() {
    let path = Path::new("input.txt");
    let contents = read_lines(path).unwrap();

    let mut split = contents.split("\n\n");

    let stacks = split.next().unwrap();
    let procedure = split.next().unwrap();

    let stack_lines = stacks.lines();

    let mut stack_vec: Vec<Vec<char>> = Vec::new();
    stack_vec.resize(stack_lines.count(), Vec::new());

    for line in stacks.lines() {
        line.char_indices()
            .filter(|p| p.1.is_uppercase())
            .map(|p| ((p.0 - 1) / 4, p.1))
            .for_each(|p| stack_vec[p.0].push(p.1))
    }

    for line in procedure.lines() {
        let v: Vec<&str> = line.split_whitespace().collect();
        let count = v[1].parse::<usize>().unwrap();
        let from = v[3].parse::<usize>().unwrap();
        let to = v[5].parse::<usize>().unwrap();

        for i in 0..count {
            let x = stack_vec[from - 1].remove(count - i - 1);
            stack_vec[to - 1].insert(0, x);
        }
    }

    let answer = stack_vec
        .iter()
        .filter(|c| c.len() > 0)
        .map(|c| c.get(0).unwrap().to_owned())
        .collect::<String>();

    println!("The answer is: {}", answer);
}

fn read_lines<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
}
