use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    North,
    South,
    East,
    West,
}

fn tiltMatrix(matrix: &Vec<Vec<char>>, dir: Direction) -> Vec<Vec<char>> {
    let mut new_matrix = matrix.clone();

    match dir {
        Direction::North => {
            for i in 0..matrix.len()-1 {
                for j in 0..matrix[i].len() {
                    for k in (0..=i).rev() {
                        if new_matrix[k][j] == '.' && new_matrix[k+1][j] == 'O' {
                            new_matrix[k][j] = 'O';
                            new_matrix[k+1][j] = '.';
                        }
                    }
                }
            }
        },
        Direction::South => {
            for i in 1..matrix.len() {
                for j in 0..matrix[i].len() {
                    for k in i..matrix.len() {
                        if new_matrix[k][j] == '.' && new_matrix[k-1][j] == 'O' {
                            new_matrix[k][j] = 'O';
                            new_matrix[k-1][j] = '.';
                        }
                    }
                }
            }
        },
        Direction::East => {
            for i in 0..matrix.len() {
                for j in 0..matrix[i].len()-1 {
                    for k in (0..=j).rev() {
                        if new_matrix[i][k] == '.' && new_matrix[i][k+1] == 'O' {
                            new_matrix[i][k] = 'O';
                            new_matrix[i][k+1] = '.';
                        }
                    }
                }
            }
        },
        Direction::West => {
            for i in 0..matrix.len() {
                for j in 1..matrix[i].len() {
                    for k in j..matrix[i].len() {
                        if new_matrix[i][k] == '.' && new_matrix[i][k-1] == 'O' {
                            new_matrix[i][k] = 'O';
                            new_matrix[i][k-1] = '.';
                        }
                    }
                }
            }
        },
    }

    new_matrix
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

    let tilted_matrix = tiltMatrix(&matrix, Direction::North);

    let n = tilted_matrix.len();

    for i in 0..tilted_matrix.len() {
        for j in 0..tilted_matrix[i].len() {
            if tilted_matrix[i][j] == 'O' {
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
