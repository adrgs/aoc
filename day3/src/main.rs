use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;
use std::collections::HashSet;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let mut matrix: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        matrix.push(line)
    }

    for i in 0..matrix.len() {
        let mut number_j = -1;
        let mut good = false;
        for j in 0..matrix[i].len() {
            if !matrix[i].chars().nth(j).unwrap().is_digit(10) {
                if good && number_j != -1 {
                    ans += matrix[i][number_j as usize..j].parse::<i32>().unwrap();
                }
                number_j = -1;
                good = false;
            } else {
                if number_j == -1 {
                    number_j = j as i32;
                }
                for di in -1..2 {
                    for dj in -1..2 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let new_i = i as i32 + di;
                        let new_j = j as i32 + dj;
                        if new_i < 0 || new_i >= matrix.len() as i32 || new_j < 0 || new_j >= matrix[i].len() as i32 {
                            continue;
                        }
                        let char = matrix[new_i as usize].chars().nth(new_j as usize).unwrap();
                        if char != '.' && !char.is_digit(10) {
                            good = true;
                        }
                    }
                }
            }
            if j == matrix[i].len() - 1 && good && number_j != -1 {
                ans += matrix[i][number_j as usize..j + 1].parse::<i32>().unwrap();
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

    let mut matrix: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        matrix.push(line)
    }

    fn get_number(matrix: &Vec<String>, i: i32, j: i32) -> (i32, i32) {
        let mut new_j = j;
        let mut low_j = j;
        let mut high_j = j;
        while new_j >= 0 && matrix[i as usize].chars().nth(new_j as usize).unwrap().is_digit(10) {
            low_j = cmp::min(low_j, new_j);
            new_j -= 1;
        }
        new_j = j;
        while new_j < matrix[i as usize].len() as i32 && matrix[i as usize].chars().nth(new_j as usize).unwrap().is_digit(10) {
            high_j = cmp::max(high_j, new_j);
            new_j += 1;
        }
        return (low_j, high_j);
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i].chars().nth(j).unwrap() == '*' {
                let mut set: HashSet<(i32, i32, i32)> = HashSet::new();
                for di in -1..2 {
                    for dj in -1..2 {
                        let new_i = i as i32 + di;
                        let new_j = j as i32 + dj;
                        if new_i < 0 || new_i >= matrix.len() as i32 || new_j < 0 || new_j >= matrix[i].len() as i32 {
                            continue;
                        }
                        let char = matrix[new_i as usize].chars().nth(new_j as usize).unwrap();
                        if char.is_digit(10) {
                            let (low_j, high_j) = get_number(&matrix, new_i, new_j);
                            set.insert((new_i, low_j, high_j));
                        }
                    }
                }
                if set.len() == 2 {
                    let mut mult = 1;
                    for (new_i, low_j, high_j) in set {
                        mult *= matrix[new_i as usize][low_j as usize..high_j as usize + 1].parse::<i32>().unwrap();
                    }
                    ans += mult;
                }
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
