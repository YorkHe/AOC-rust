use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn compute_score(opp_hand: &str, self_hand: &str) -> i32 {
    let mut score = 0;
    if self_hand == "X" {
        score = 0
    } else if self_hand == "Y" {
        score = 3
    } else if self_hand == "Z" {
        score = 6
    }

    if opp_hand == "A" {
        if self_hand == "X" {
            score += 3;
        } else if self_hand == "Y" {
            score += 1;
        } else if self_hand == "Z" {
            score += 2;
        }
    }

    if opp_hand == "B" {
        if self_hand == "X" {
            score += 1;
        } else if self_hand == "Y" {
            score += 2;
        } else if self_hand == "Z" {
            score += 3;
        }
    }

    if opp_hand == "C" {
        if self_hand == "X" {
            score += 2;
        } else if self_hand == "Y" {
            score += 3;
        } else if self_hand == "Z" {
            score += 1;
        }
    }

    score
}

fn main() {
    let path = Path::new("input.txt");

    let lines = match read_lines(path) {
        Err(e) => panic!("Failed to read lines {}", e),
        Ok(lines) => lines,
    };

    let mut total_score = 0;

    for line in lines {
        let line_str = line.unwrap();
        let mut split = line_str.splitn(3, ' ');
        let opp_hand = split.next().unwrap();
        let self_hand = split.next().unwrap();

        println!("opp: {} self: {}", opp_hand, self_hand);
        total_score += compute_score(opp_hand, self_hand);
    }

    println!("Total Score: {}", total_score)
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
