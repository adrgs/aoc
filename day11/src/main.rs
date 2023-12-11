use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let mut i = 0 as usize;
    while i < matrix.len() {
        let mut empty_row = true;
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '#' {
                empty_row = false;
                break;
            }
        }
        if empty_row {
            matrix.insert(i, matrix[i].clone());
            i += 1;
        }
        i += 1;
    }
    let mut j = 0 as usize;
    while j < matrix[0].len() {
        let mut empty_col = true;
        for i in 0..matrix.len() {
            if matrix[i][j] == '#' {
                empty_col = false;
                break;
            }
        }
        if empty_col {
            for i in 0..matrix.len() {
                matrix[i].insert(j, '.');
            }
            j += 1;
        }
        j += 1;
    }

    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let dist = (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
            ans += dist;
        }
    }

    // print matrix
    // for i in 0..matrix.len() {
    //     for j in 0..matrix[i].len() {
    //         print!("{}", matrix[i][j]);
    //     }
    //     println!();
    // }

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans: i64 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let row = line.chars().collect::<Vec<char>>();
        matrix.push(row);
    }
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    let mut i = 0 as usize;
    while i < matrix.len() {
        let mut empty_row = true;
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '#' {
                empty_row = false;
                break;
            }
        }
        if empty_row {
            empty_rows.push(i);
        }
        i += 1;
    }
    let mut j = 0 as usize;
    while j < matrix[0].len() {
        let mut empty_col = true;
        for i in 0..matrix.len() {
            if matrix[i][j] == '#' {
                empty_col = false;
                break;
            }
        }
        if empty_col {
            empty_cols.push(j);
        }
        j += 1;
    }

    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    let empty_add = 1000000 - 1;

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let mut dist: i64 = ((galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs()) as i64; // base distance

            for gi in (galaxies[i].0.min(galaxies[j].0) as usize)..=(galaxies[i].0.max(galaxies[j].0) as usize) {
                if empty_rows.binary_search(&gi).is_ok() {
                    dist += empty_add;
                }
            }
            for gj in (galaxies[i].1.min(galaxies[j].1) as usize)..=(galaxies[i].1.max(galaxies[j].1) as usize) {
                if empty_cols.binary_search(&gj).is_ok() {
                    dist += empty_add;
                }
            }

            ans += dist;
        }
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
