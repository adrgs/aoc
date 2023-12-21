use std::collections::{VecDeque, HashSet, HashMap};
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

    let matrix: Vec<Vec<i32>> = reader
    .lines()
    .map(|line| {
        line.unwrap().chars()
            .map(|c| if c == '#' { -1 } else if c == 'S' { -2 } else { 0 })
            .collect()
    })
    .collect();

    let n = matrix.len() as i32; // assume square matrix
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();

    let mut start = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == -2 {
                start = (i as i32, j as i32);
            } else if matrix[i][j] == -1 {
                rocks.insert((i as i32, j as i32));
            }
        }
    }

    let mut possible: HashSet<(i32, i32)> = HashSet::new();
    let mut points: HashMap<i32, i64> = HashMap::new();

    possible.insert(start);

    let steps = 26501365;

    for s in 1..steps {
        let mut new_possible: HashSet<(i32, i32)> = HashSet::new();
        for (i, j) in &possible {
            for di in -1..(2 as i32) {
                for dj in -1..(2 as i32) {
                    if di.abs() == dj.abs() {
                        continue;
                    }
                    if rocks.contains(&((i + di).rem_euclid(n), (j + dj).rem_euclid(n))) {
                        continue;
                    }
                    new_possible.insert((i + di, j + dj));
                }
            }
        }
        let x = new_possible.len();
        possible = new_possible;
        if s % n == steps % n {
            points.insert(s/n, x as i64);
        }
        if points.len() == 3 {
            break;
        }
    }

    let x = steps as i64/n as i64;

    let a = (points[&2] + points[&0] - 2 * points[&1])/2;
    let b = points[&1] - points[&0] - a;
    let c = points[&0];

    let ans = a*(x.pow(2)) + b*x + c;

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
