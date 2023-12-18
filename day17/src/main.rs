use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Reverse;

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32)
        }

        matrix.push(row);
    }

    let mut heap: BinaryHeap<(Reverse<i32>, Direction, (usize, usize))> = BinaryHeap::new();
    let mut visited: HashMap<(Direction, (usize, usize)), i32> = HashMap::new();

    let mut val = 0;
    for i in 1..=3 {
        val += matrix[i][0];
        heap.push((Reverse(val), Direction::Down, (i, 0)));
        visited.insert((Direction::Down, (i, 0)), val);
    }
    val = 0;
    for j in 1..=3 {
        val += matrix[0][j];
        heap.push((Reverse(val), Direction::Right, (0, j)));
        visited.insert((Direction::Right, (0, j)), val);
    }

    while !heap.is_empty() {
        let (val, dir, (i, j)) = heap.pop().unwrap();
        let val = val.0;

        if i == matrix.len() - 1 && j == matrix[0].len() - 1 {
            ans = val;
            break;
        }

        match dir {
            Direction::Up | Direction::Down => {
                let (mut pval, mut nval) = (val, val);
                for dj in 1..=3 {
                    let nj = j + dj;
                    if nj < matrix[0].len() {
                        pval += matrix[i][nj];
                        if pval < *visited.get(&(Direction::Right, (i, nj))).unwrap_or(&i32::MAX) {
                            heap.push((Reverse(pval), Direction::Right, (i, nj)));
                            visited.insert((Direction::Right, (i, nj)), pval);
                        }
                    }
                    if dj > j {
                        continue;
                    }
                    let nj = j - dj;
                    nval += matrix[i][nj];
                    if nval < *visited.get(&(Direction::Left, (i, nj))).unwrap_or(&i32::MAX) {
                        heap.push((Reverse(nval), Direction::Left, (i, nj)));
                        visited.insert((Direction::Left, (i, nj)), nval);
                    }
                }
            },
            Direction::Left | Direction::Right => {
                let (mut pval, mut nval) = (val, val);
                for di in 1..=3 {
                    let ni = i + di;
                    if ni < matrix.len() {
                        pval += matrix[ni][j];
                        if pval < *visited.get(&(Direction::Down, (ni, j))).unwrap_or(&i32::MAX) {
                            heap.push((Reverse(pval), Direction::Down, (ni, j)));
                            visited.insert((Direction::Down, (ni, j)), pval);
                        }
                    }
                    if di > i {
                        continue;
                    }
                    let ni = i - di;
                    nval += matrix[ni][j];
                    if nval < *visited.get(&(Direction::Up, (ni, j))).unwrap_or(&i32::MAX) {
                        heap.push((Reverse(nval), Direction::Up, (ni, j)));
                        visited.insert((Direction::Up, (ni, j)), nval);
                    }
                }
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
