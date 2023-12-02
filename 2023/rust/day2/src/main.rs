use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

fn main() {
    let _color_map: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let lines = match read_lines("phase_1.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Error: {}", e),
    };

    let mut sum = 0;

    for line in lines {
        let mut min_color_map: HashMap<&str, u32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        if let Ok(line) = line {
            println!("{}", line);

            let split = line.split_once(":");

            if let Some((_id_str, game_str)) = split {
                let games = game_str.split(";");

                for game in games {
                    let rounds = game.split(",");
                    for round in rounds {
                        let (count, color) = round.trim().split_once(" ").unwrap();
                        println!("{} {}", count, color);
                        if min_color_map[color] < count.parse::<u32>().unwrap() {
                            min_color_map.insert(color, count.parse::<u32>().unwrap());
                        }
                    }
                }

                println!("{:?}", min_color_map);

                let mul = min_color_map["red"] * min_color_map["green"] * min_color_map["blue"];
                sum += mul;
            }
        }

        println!("Sum: {}", sum);
    }
}
