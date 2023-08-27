use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn find_overlap<'a>(s1: &'a str, s2: &'a str, s3: &'a str) -> char {
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    let mut set3 = HashSet::new();

    s1.chars().for_each(|v| {
        set1.insert(v.clone());
    });
    s2.chars().for_each(|v| {
        set2.insert(v.clone());
    });
    s3.chars().for_each(|v| {
        set3.insert(v.clone());
    });

    match set1.intersection(&set2).filter(|c| set3.contains(c)).last() {
        Some(c) => c.clone(),
        None => '\0',
    }
}

fn get_score(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        return c.clone() as u32 - 'a' as u32 + 1;
    } else if c.is_ascii_uppercase() {
        return c.clone() as u32 - 'A' as u32 + 26 + 1;
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

    let lines_vec: Vec<_> = lines.collect();

    for i in (0..lines_vec.len()).step_by(3) {
        let line1 = lines_vec[i].as_deref().unwrap();
        let line2 = lines_vec[i + 1].as_deref().unwrap();
        let line3 = lines_vec[i + 2].as_deref().unwrap();

        let overlap_char = find_overlap(&line1, &line2, &line3);

        let score = get_score(&overlap_char);
        total_score += score;

        println!(
            "First {}, Second {}, Third {}, Overlap {}, score {}",
            line1, line2, line3, overlap_char, score
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
