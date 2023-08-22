// AOC 2022 - Day1 Calorie Counting

use std::{
    cmp,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use ::priority_queue::PriorityQueue;
use priority_queue::priority_queue;

fn main() {
    let lines = match read_lines("input.text") {
        Err(e) => panic!("Failed to read lines from input.text {}", e),
        Ok(lines) => lines,
    };

    let mut pq = PriorityQueue::new();

    let mut current_sum = 0;
    for line in lines {
        match line.unwrap().parse::<i32>() {
            Err(e) => current_sum = 0,
            Ok(current_wight) => {
                current_sum += current_wight;
                pq.push(current_sum, -current_sum);
                if pq.len() > 3 {
                    pq.pop();
                }
            }
        };
    }

    println!(
        "The sum of the top 3 weights are: {}",
        pq.pop().unwrap().0 + pq.pop().unwrap().0 + pq.pop().unwrap().0,
    )
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
