use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_type(val: u8) -> u8 {
    let ans = (val & (1 << 0)) + (val & (1 << 1)) + (val & (1 << 2)) + (val & (1 << 3));
    ans
}

fn get_new_coords(i: i32, j: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (i - 1, j),
        Direction::Down => (i + 1, j),
        Direction::Left => (i, j - 1),
        Direction::Right => (i, j + 1),
    }
}

fn traverse(matrix: &mut Vec<Vec<u8>>, i: i32, j: i32, dir: Direction) {
    if i < 0 || j < 0 || i >= matrix.len() as i32 || j >= matrix[0].len() as i32 {
        return;
    }
    let (i, j) = (i as usize, j as usize);
    let idx = match dir {
        Direction::Up => 4,
        Direction::Down => 5,
        Direction::Left => 6,
        Direction::Right => 7,
    };
    if matrix[i][j] & (1 << idx) != 0 {
        return;
    }
    matrix[i][j] |= 1 << idx;

    match get_type(matrix[i][j]) {
        1 => {
            if dir == Direction::Left || dir == Direction::Right {
                let (i_new, j_new) = get_new_coords(i as i32, j as i32, Direction::Up);
                traverse(matrix, i_new, j_new, Direction::Up);
                let (i, j) = get_new_coords(i as i32, j as i32, Direction::Down);
                traverse(matrix, i, j, Direction::Down);
            } else {
                let (i, j) = get_new_coords(i as i32, j as i32, dir);
                traverse(matrix, i, j, dir);
            }
        },
        2 => {
            if dir == Direction::Up || dir == Direction::Down {
                let (i_new, j_new) = get_new_coords(i as i32, j as i32, Direction::Left);
                traverse(matrix, i_new, j_new, Direction::Left);
                let (i, j) = get_new_coords(i as i32, j as i32, Direction::Right);
                traverse(matrix, i, j, Direction::Right);
            } else {
                let (i, j) = get_new_coords(i as i32, j as i32, dir);
                traverse(matrix, i, j, dir);
            }
        },
        4 => {
            let new_dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            let (i, j) = get_new_coords(i as i32, j as i32, new_dir);
            traverse(matrix, i, j, new_dir);
        }
        8 => {
            let new_dir = match dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            let (i, j) = get_new_coords(i as i32, j as i32, new_dir);
            traverse(matrix, i, j, new_dir);
        }
        _ => {
            let (i, j) = get_new_coords(i as i32, j as i32, dir);
            traverse(matrix, i, j, dir);
        }
    }
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(0),
                '|' => row.push(1 << 0),
                '-' => row.push(1 << 1),
                '/' => row.push(1 << 2),
                '\\' => row.push(1 << 3),
                _ => (),
            }
        }
        matrix.push(row);
    }

    traverse(&mut matrix, 0, 0, Direction::Right);

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] >> 4 > 0 {
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
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(0),
                '|' => row.push(1 << 0),
                '-' => row.push(1 << 1),
                '/' => row.push(1 << 2),
                '\\' => row.push(1 << 3),
                _ => (),
            }
        }
        matrix.push(row);
    }

    fn get_result(matrix: &Vec<Vec<u8>>) -> i32 {
        let mut ans = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] >> 4 > 0 {
                    ans += 1;
                }
            }
        }
        ans
    }

    for j in 0..matrix[0].len() {
        let mut new_matrix = matrix.clone();
        traverse(&mut new_matrix, 0, j as i32, Direction::Down);
        ans = ans.max(get_result(&new_matrix));

        let mut new_matrix = matrix.clone();
        traverse(&mut new_matrix, matrix.len() as i32 - 1, j as i32, Direction::Up);
        ans = ans.max(get_result(&new_matrix));
    }

    for i in 0..matrix.len() {
        let mut new_matrix = matrix.clone();
        traverse(&mut new_matrix, i as i32, 0, Direction::Right);
        ans = ans.max(get_result(&new_matrix));

        let mut new_matrix = matrix.clone();
        traverse(&mut new_matrix, i as i32, matrix[0].len() as i32 - 1, Direction::Left);
        ans = ans.max(get_result(&new_matrix));
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
