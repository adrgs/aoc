use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        
        let (game, matches) = line.split_once(": ").unwrap();
        let id = game.split_once(" ").unwrap().1;
        let mut good = true;
        
        for m in matches.split("; ") {
            for result in m.split(", ") {
                let (quantity, color) = result.split_once(" ").unwrap();
                let quantity = quantity.parse::<i32>().unwrap();
                if (color == "red" && quantity > 12) || (color == "green" && quantity > 13) || (color == "blue" && quantity > 14) {
                    good = false;
                    break;
                }
            }
        }

        if good {
            ans += id.parse::<i32>().unwrap();
        }
    }

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        
        let (_, matches) = line.split_once(": ").unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        
        for m in matches.split("; ") {
            for result in m.split(", ") {
                let (quantity, color) = result.split_once(" ").unwrap();
                let quantity = quantity.parse::<i32>().unwrap();
                match color {
                    "red" if quantity > max_red => max_red = quantity,
                    "green" if quantity > max_green => max_green = quantity,
                    "blue" if quantity > max_blue => max_blue = quantity,
                    _ => ()
                }
            }
        }

        ans += max_red * max_green * max_blue;
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
