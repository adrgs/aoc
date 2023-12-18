#![feature(iter_map_windows)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let mut x = 0;
    let mut y = 0;

    let mut polygon: Vec<(i32, i32)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let _: Vec<i32> = line.split_whitespace().map_windows(|[dir, amount, hex]| {
            polygon.push((x, y));
            let amount = amount.parse::<i32>().unwrap();
            match *dir {
                "R" => x += amount,
                "L" => x -= amount,
                "U" => y += amount,
                "D" => y -= amount,
                _ => panic!("Invalid direction"),
            }
            0
        }).collect();
    }

    let mut perimeter = 0;
    for i in 0..polygon.len() {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % polygon.len()];
        perimeter += (x2 - x1).abs() + (y2 - y1).abs();
    }

    //ans = perimeter;

    let mut area = 0;
    for i in 0..polygon.len() {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % polygon.len()];
        let det = x1 * y2 - x2 * y1;
        area += det;
    }
    area = area.abs() / 2;
    ans = area + perimeter / 2 + 1;

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
