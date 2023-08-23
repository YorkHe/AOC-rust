use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn find_overlap(s1: &str, s2: &str) -> char {
    for c1 in s1.chars() {
        for c2 in s2.chars() {
            if c1 == c2 {
                return c1;
            }
        }
    }

    return '\0';
}

fn get_score(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return c as u32 - 'a' as u32 + 1;
    } else if c.is_ascii_uppercase() {
        return c as u32 - 'A' as u32 + 26 + 1;
    } else {
        0
    }
}

fn main() {
    let path = Path::new("input.txt");
    let lines = match read_lines(path) {
        Err(e) => panic!("Failed to read lines from file {}", e),
        Ok(lines) => lines,
    };

    let mut total_score = 0;

    for line in lines {
        let s = line.unwrap();
        let first_half = &s[..s.len() / 2];
        let second_half = &s[s.len() / 2..];

        let overlap_char = find_overlap(first_half, second_half);
        let score = get_score(overlap_char);
        total_score += score;

        println!(
            "First {}, Second {}, Overlap {}, score {}",
            first_half, second_half, overlap_char, score
        );
    }

    println!("Total score: {}", total_score);
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
