use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn tiltMatrix(matrix:&mut Vec<Vec<char>>, dir: Direction) {
    match dir {
        Direction::North => {
            for i in 0..matrix.len()-1 {
                for j in 0..matrix[i].len() {
                    for k in (0..=i).rev() {
                        if matrix[k][j] == '.' && matrix[k+1][j] == 'O' {
                            matrix[k][j] = 'O';
                            matrix[k+1][j] = '.';
                        }
                    }
                }
            }
        },
        Direction::South => {
            for i in 1..matrix.len() {
                for j in 0..matrix[i].len() {
                    for k in (i..matrix.len()).rev() {
                        if matrix[k][j] == '.' && matrix[k-1][j] == 'O' {
                            matrix[k][j] = 'O';
                            matrix[k-1][j] = '.';
                        }
                    }
                }
            }
        },
        Direction::West => {
            for i in 0..matrix.len() {
                for j in 0..matrix[i].len()-1 {
                    for k in (0..=j).rev() {
                        if matrix[i][k] == '.' && matrix[i][k+1] == 'O' {
                            matrix[i][k] = 'O';
                            matrix[i][k+1] = '.';
                        }
                    }
                }
            }
        },
        Direction::East => {
            for i in 0..matrix.len() {
                for j in 1..matrix[i].len() {
                    for k in (j..matrix[i].len()).rev() {
                        if matrix[i][k] == '.' && matrix[i][k-1] == 'O' {
                            matrix[i][k] = 'O';
                            matrix[i][k-1] = '.';
                        }
                    }
                }
            }
        },
    }
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let row = line.chars().collect::<Vec<char>>();
        matrix.push(row);
    }

    tiltMatrix(&mut matrix, Direction::North);

    let n = matrix.len();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'O' {
                ans += n-i;
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
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let row = line.chars().collect::<Vec<char>>();
        matrix.push(row);
    }

    let mut i_slow = 1;
    let mut i_fast = 2;

    let arr = vec![Direction::North, Direction::West, Direction::South, Direction::East];

    let mut slow = matrix.clone();
    let mut fast = matrix.clone();

    loop {
        for k in 0..4 {
            tiltMatrix(&mut slow, arr[k]);
        }
        i_slow += 1;
        for k in 0..4 {
            tiltMatrix(&mut fast, arr[k]);
        }
        i_fast += 1;
        for k in 0..4 {
            tiltMatrix(&mut fast, arr[k]);
        }
        i_fast += 1;

        if slow == fast {
            break;
        }
    }

    let cycle_len = i_fast - i_slow - 1;

    for _ in 0..(1000000000%(cycle_len) + cycle_len) {
        for k in 0..4 {
            tiltMatrix(&mut matrix, arr[k]);
        }
    }

    ans = 0;
    let n = matrix.len();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'O' {
                ans += n-i;
            }
        }
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
