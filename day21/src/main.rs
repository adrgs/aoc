use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let mut matrix: Vec<Vec<i32>> = reader
    .lines()
    .map(|line| {
        line.unwrap().chars()
            .map(|c| if c == '#' { -1 } else if c == 'S' { -2 } else { 0 })
            .collect()
    })
    .collect();

    let mut start = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == -2 {
                start = (i, j);
            }
        }
    }

    let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();
    let max_steps = 64;
    queue.push_back((start, 0));
    while queue.len() > 0 {
        let (node, steps) = queue.pop_front().unwrap();
        let (i, j) = node;

        for di in -1..(2 as i32) {
            for dj in -1..(2 as i32) {
                if di.abs() == dj.abs() {
                    continue;
                }
                if i as i32 + di < 0 || i as i32 + di >= matrix.len() as i32 || j as i32 + dj < 0 || j as i32 + dj >= matrix[i].len() as i32 {
                    continue;
                }
                if matrix[(i as i32 + di) as usize][(j as i32 + dj) as usize] == 0 {
                    matrix[(i as i32 + di) as usize][(j as i32 + dj) as usize] = steps + 1;
                    if steps <= max_steps - 1 {
                        queue.push_back((((i as i32 + di) as usize, (j as i32 + dj) as usize), steps + 1));
                    }
                }
            }
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] > 0 && (matrix[i][j] % 2) == (max_steps % 2) || matrix[i][j] == -2 {
                ans += 1;
            }
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
        
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
