use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    fn check_reflection(matrix: &Vec<Vec<char>>) -> i64 {
        // check horizontal lines
        for i in 1..matrix.len() {
            let mut found = false;
            if matrix[i] == matrix[i - 1] {
                found = true;
                for k in 1..matrix.len() {
                    if i + k >= matrix.len() || i < k + 1 {
                        break;
                    }
                    if matrix[i + k] != matrix[i - 1 - k] {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                return (i * 100) as i64;
            }
        }
        // check vertical lines
        for j in 1..matrix[0].len() {
            let mut found = false;
            let column1 = matrix.iter().map(|row| row[j - 1]).collect::<Vec<char>>();
            let column2 = matrix.iter().map(|row| row[j]).collect::<Vec<char>>();

            if column1 == column2 {
                found = true;

                for k in 1..matrix.len() {
                    if j + k >= matrix[0].len() || j < k + 1 {
                        break;
                    }
                    let c1 = matrix
                        .iter()
                        .map(|row| row[j - 1 - k])
                        .collect::<Vec<char>>();
                    let c2 = matrix.iter().map(|row| row[j + k]).collect::<Vec<char>>();

                    if c1 != c2 {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                return j as i64;
            }
        }
        0
    }

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            ans += check_reflection(&matrix);
            matrix.clear();
        } else {
            let row = line.chars().collect();
            matrix.push(row);
        }
    }

    ans += check_reflection(&matrix);

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    fn check_reflection(matrix: &Vec<Vec<char>>) -> Vec<i64> {
        // check horizontal lines
        let mut ans = Vec::new();
        for i in 1..matrix.len() {
            let mut found = false;
            if matrix[i] == matrix[i - 1] {
                found = true;
                for k in 1..matrix.len() {
                    if i + k >= matrix.len() || i < k + 1 {
                        break;
                    }
                    if matrix[i + k] != matrix[i - 1 - k] {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                ans.push((i * 100) as i64);
            }
        }
        // check vertical lines
        for j in 1..matrix[0].len() {
            let mut found = false;
            let column1 = matrix.iter().map(|row| row[j - 1]).collect::<Vec<char>>();
            let column2 = matrix.iter().map(|row| row[j]).collect::<Vec<char>>();

            if column1 == column2 {
                found = true;

                for k in 1..matrix.len() {
                    if j + k >= matrix[0].len() || j < k + 1 {
                        break;
                    }
                    let c1 = matrix
                        .iter()
                        .map(|row| row[j - 1 - k])
                        .collect::<Vec<char>>();
                    let c2 = matrix.iter().map(|row| row[j + k]).collect::<Vec<char>>();

                    if c1 != c2 {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                ans.push(j as i64);
            }
        }
        ans
    }

    fn get_smudge_reflection(matrix: &mut Vec<Vec<char>>) -> i64 {
        let og_reflection = check_reflection(&matrix);
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix[i][j] = if matrix[i][j] == '#' { '.' } else { '#' };

                let reflection = check_reflection(&matrix);
                for r in &reflection {
                    if !og_reflection.contains(r) {
                        return *r;
                    }
                }

                matrix[i][j] = if matrix[i][j] == '#' { '.' } else { '#' };
            }
        }
        0
    }

    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            ans += get_smudge_reflection(&mut matrix);
            matrix.clear();
        } else {
            let row = line.chars().collect();
            matrix.push(row);
        }
    }

    ans += get_smudge_reflection(&mut matrix);

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
