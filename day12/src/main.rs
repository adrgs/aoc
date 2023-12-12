use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    fn bt(row: &mut Vec<char>, conditions: &Vec<i64>, i: usize) -> i64 {
        if i == row.len() {
            let mut groups: Vec<i64> = Vec::new();
            let mut left: usize = 0;
            let mut right: usize = 0;
            while right < row.len() {
                if row[right] == '#' {
                    if row[left] != '#' {
                        left = right;
                    }
                    while right < row.len() && row[right] == '#' {
                        right += 1;
                    }
                    groups.push((right - left) as i64);
                    left = right;
                } else {
                    right += 1;
                }
            }
            return (groups == *conditions) as i64;
        }
        let mut ans = 0;
        if row[i] == '?' {
            row[i] = '.';
            ans += bt(row, conditions, i + 1);
            row[i] = '#';
            ans += bt(row, conditions, i + 1);
            row[i] = '?';
        } else {
            ans = bt(row, conditions, i + 1);
        }
        return ans;
    }

    for line in reader.lines() {
        let line = line.unwrap();
        
        let (row, conditions) = line.split_once(" ").unwrap();
        let mut row: Vec<char> = row.chars().collect();
        let conditions = conditions.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        ans += bt(&mut row, &conditions, 0);
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
