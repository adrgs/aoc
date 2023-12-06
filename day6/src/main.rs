use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 1;
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();

    let mut line_counter = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        for val in line.split_whitespace() {
            if val.contains(":") { continue; }
            if line_counter == 0 {
                times.push(val.parse::<i64>().unwrap());
            } else {
                distances.push(val.parse::<i64>().unwrap());
            }
        }
        line_counter += 1;
    }

    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut wins = 0;
        for j in 1..time {
            if (time - j) * j > distance {
                wins += 1;
            }
        }
        ans *= wins;
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
        
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
