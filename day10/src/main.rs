use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn traverse(
    matrix: &Vec<Vec<char>>,
    start: (i32, i32),
    dir: (i32, i32),
    visited: &mut Vec<Vec<i32>>,
) {
    let mut curr = start;
    let mut dist = 0;
    let mut dir = dir;
    visited[curr.0 as usize][curr.1 as usize] = dist;
    curr.0 += dir.0;
    curr.1 += dir.1;
    while curr != start {
        dist += 1;
        if curr.0 >= matrix.len() as i32
            || curr.1 >= matrix[0].len() as i32
            || curr.0 < 0
            || curr.1 < 0
        {
            break;
        }
        match matrix[curr.0 as usize][curr.1 as usize] {
            '│' => {
                if dir.1.abs() > 0 {
                    break;
                }
            }
            '─' => {
                if dir.0.abs() > 0 {
                    break;
                }
            }
            '└' => {
                if dir.0 < 0 || dir.1 > 0 {
                    break;
                }
                if dir.0 == 0 {
                    dir = (-1, 0);
                } else {
                    dir = (0, 1);
                }
            }
            '┘' => {
                if dir.0 < 0 || dir.1 < 0 {
                    break;
                }
                if dir.0 == 0 {
                    dir = (-1, 0);
                } else {
                    dir = (0, -1);
                }
            }
            '┐' => {
                if dir.0 > 0 || dir.1 < 0 {
                    break;
                }
                if dir.0 == 0 {
                    dir = (1, 0);
                } else {
                    dir = (0, -1);
                }
            }
            '┌' => {
                if dir.0 > 0 || dir.1 > 0 {
                    break;
                }
                if dir.0 == 0 {
                    dir = (1, 0);
                } else {
                    dir = (0, 1);
                }
            }
            _ => break,
        }
        if visited[curr.0 as usize][curr.1 as usize] == -1 {
            visited[curr.0 as usize][curr.1 as usize] = dist;
        } else {
            visited[curr.0 as usize][curr.1 as usize] =
                dist.min(visited[curr.0 as usize][curr.1 as usize]);
        }
        curr.0 += dir.0;
        curr.1 += dir.1;
    }
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<i32>> = Vec::new();

    let mut start = (0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();
        let mut visited_row: Vec<i32> = Vec::new();
        for c in line.chars() {
            match c {
                '|' => row.push('│'),
                '-' => row.push('─'),
                'L' => row.push('└'),
                'J' => row.push('┘'),
                '7' => row.push('┐'),
                'F' => row.push('┌'),
                'S' => {
                    start = (matrix.len() as i32, row.len() as i32);
                    row.push('S')
                }
                x => row.push(x),
            }
            visited_row.push(-1);
        }
        matrix.push(row);
        visited.push(visited_row);
    }

    // print the matrix
    // for row in &matrix {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    traverse(&matrix, start, (0, 1), &mut visited);
    traverse(&matrix, start, (0, -1), &mut visited);
    traverse(&matrix, start, (1, 0), &mut visited);
    traverse(&matrix, start, (-1, 0), &mut visited);

    for row in &visited {
        for c in row {
            if *c > ans {
                ans = *c;
            }
        }
    }

    println!("Part 1: {}", ans);

    Ok(())
}

fn get(visited: &Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
    if i < 0 || j < 0 || i >= visited.len() as i32 || j >= visited[0].len() as i32 {
        return -1;
    }
    visited[i as usize][j as usize]
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<i32>> = Vec::new();

    let mut start = (0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();
        let mut visited_row: Vec<i32> = Vec::new();
        for c in line.chars() {
            match c {
                '|' => row.push('│'),
                '-' => row.push('─'),
                'L' => row.push('└'),
                'J' => row.push('┘'),
                '7' => row.push('┐'),
                'F' => row.push('┌'),
                'S' => {
                    start = (matrix.len() as i32, row.len() as i32);
                    row.push('S')
                }
                x => row.push(x),
            }
            visited_row.push(-1);
        }
        matrix.push(row);
        visited.push(visited_row);
    }

    traverse(&matrix, start, (0, 1), &mut visited);
    traverse(&matrix, start, (0, -1), &mut visited);
    traverse(&matrix, start, (1, 0), &mut visited);
    traverse(&matrix, start, (-1, 0), &mut visited);

    if get(&visited, start.0 + 1, start.1) == 1 && get(&visited, start.0 - 1, start.1) == 1 {
        matrix[start.0 as usize][start.1 as usize] = '│';
    } else if get(&visited, start.0, start.1 + 1) == 1 && get(&visited, start.0, start.1 - 1) == 1 {
        matrix[start.0 as usize][start.1 as usize] = '─';
    } else if get(&visited, start.0 - 1, start.1) == 1 && get(&visited, start.0, start.1 + 1) == 1 {
        matrix[start.0 as usize][start.1 as usize] = '└';
    } else if get(&visited, start.0 - 1, start.1) == 1 && get(&visited, start.0, start.1 - 1) == 1 {
        matrix[start.0 as usize][start.1 as usize] = '┘';
    } else if get(&visited, start.0 + 1, start.1) == 1 && get(&visited, start.0, start.1 - 1) == 1 {
        matrix[start.0 as usize][start.1 as usize] = '┐';
    } else if get(&visited, start.0 + 1, start.1) == 1 && get(&visited, start.0, start.1 + 1) == 1 {
        matrix[start.0 as usize][start.1 as usize] = '┌';
    }

    for i in 0..matrix.len() {
        let mut is_inside = false;
        let mut last_char = ' ';
        for j in 0..matrix[0].len() {
            if visited[i][j] >= 0 {
                match matrix[i][j] {
                    '│' => {
                        is_inside = !is_inside;
                    }
                    '└' => {
                        last_char = '└';
                    }
                    '┘' => {
                        if last_char == '┌' {
                            is_inside = !is_inside;
                        }
                    }
                    '┐' => {
                        if last_char == '└' {
                            is_inside = !is_inside;
                        }
                    }
                    '┌' => {
                        last_char = '┌';
                    }
                    _ => {}
                }
            } else {
                if is_inside {
                    ans += 1;
                    matrix[i][j] = 'X';
                }
            }
        }
    }

    // print the matrix
    // for row in &matrix {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
