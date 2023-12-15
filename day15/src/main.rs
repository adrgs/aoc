use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn hash(input: &str) -> i32 {
    let mut val = 0;
    for c in input.chars() {
        val += c as i32;
        val *= 17;
        val %= 256;
    }

    val
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    

    let input = reader.lines().take(1).next().unwrap().unwrap();
    for step in input.split(',') {
        ans += hash(step);
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
