use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn has_duplicate(s: &str) -> bool {
    s.char_indices()
        .any(|p| s.chars().skip(p.0 + 1).any(|c| c == p.1))
}

fn main() {
    let path = Path::new("input.txt");
    let lines = read_lines(path).unwrap();

    for line in lines {
        let s = line.unwrap();

        let l: usize = 0;
        let r: usize = 14;

        for i in 0..s.len() - 14 {
            let window = &s[l + i..r + i];
            if !has_duplicate(window) {
                println!("answer: {}", i + r);
                break;
            }
        }
    }
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
