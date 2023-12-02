use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut min_idx = -1;
        let mut max_idx = -1;

        for (i, c) in line.char_indices() {
            if c <= '9' && c >= '0' {
                if min_idx == -1 {
                    min_idx = i as i32;
                } else {
                    min_idx = min_idx.min(i as i32);
                }
                max_idx = i as i32;
            }
        }
        sum += line
            .chars()
            .nth(min_idx as usize)
            .unwrap()
            .to_digit(10)
            .unwrap()
            * 10
            + line
                .chars()
                .nth(max_idx as usize)
                .unwrap()
                .to_digit(10)
                .unwrap();
    }

    println!("Part 1: {}", sum);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let number_vector = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut left_number = -1;
        let mut right_number = -1;

        for i in 0..line.len() {
            let c = line.chars().nth(i).unwrap();
            if c <= '9' && c >= '0' {
                if left_number == -1 {
                    left_number = c.to_digit(10).unwrap() as i32;
                    if right_number == -1 {
                        right_number = left_number;
                    }
                } else {
                    right_number = c.to_digit(10).unwrap() as i32;
                }
            } else {
                for (j, number) in number_vector.iter().enumerate() {
                    if j == 0 {
                        continue;
                    }
                    if line[i..].starts_with(number) {
                        if left_number == -1 {
                            left_number = j as i32;
                            if right_number == -1 {
                                right_number = left_number;
                            }
                        } else {
                            right_number = j as i32;
                        }
                    }
                }
            }
        }
        sum += left_number * 10 + right_number;
    }

    println!("Part 2: {}", sum);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
