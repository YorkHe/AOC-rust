use std::{io::{self, BufRead, BufReader, Lines}, fs::File, path::Path};


fn read_lines<P> (path: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn match_number(input: &str) -> Option<u8> {
    let number_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    for (word, number) in number_words.iter() {
        if input.starts_with(word) {
            return Some(*number);
        }
    }

    None
}

fn main() {
    let lines = match read_lines("phase1.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Error: {}", e),
    };

    let mut sum = 0;

    for line in lines {
        if let Ok(s) = line {
            let mut first_num: u32 = 10;
            let mut last_num: u32 = 0;
            for i in 0..s.len() {
                let c = s.chars().nth(i).unwrap();

                if c.is_numeric() {
                    if first_num == 10 {
                        first_num = c.to_digit(10).unwrap();
                    }
                    last_num = c.to_digit(10).unwrap();
                } else {
                    let parsed_num = match_number(&s[i..]);
                    if let Some(num) = parsed_num {
                        if first_num == 10 {
                            first_num = num as u32;
                        }
                        last_num = num as u32;
                    }
                }
            }

            println!("{} {} {}", s, first_num, last_num);

            sum += (first_num * 10) + last_num;
        }
    }

    println!("Sum: {}", sum);
}
