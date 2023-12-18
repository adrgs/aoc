#![feature(iter_map_windows)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut x = 0;
    let mut y = 0;

    let mut polygon: Vec<(i32, i32)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let _: Vec<i32> = line.split_whitespace().map_windows(|[dir, amount, _]| {
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

    let mut area = 0;
    for i in 0..polygon.len() {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % polygon.len()];
        let det = x1 * y2 - x2 * y1;
        area += det;
    }
    area = area.abs() / 2;
    let ans = area + perimeter / 2 + 1;

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut x = 0;
    let mut y = 0;

    let mut polygon: Vec<(i64, i64)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let _: Vec<i32> = line.split_whitespace().map_windows(|[_, _, hex]| {
            polygon.push((x, y));
            let n = hex.len();
            let amount = &hex[2..n-2];
            let amount = i64::from_str_radix(amount, 16).unwrap();
            match (*hex).chars().nth(n-2).unwrap() {
                '0' => x += amount,
                '2' => x -= amount,
                '3' => y += amount,
                '1' => y -= amount,
                _ => panic!("Invalid direction {}", (*hex).chars().nth(n-2).unwrap()),
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

    let mut area = 0;
    for i in 0..polygon.len() {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % polygon.len()];
        let det = x1 * y2 - x2 * y1;
        area += det;
    }
    area = area.abs() / 2;
    let ans = area + perimeter / 2 + 1;

    println!("Part 1: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
