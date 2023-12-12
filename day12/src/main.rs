use std::collections::HashMap;
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

    fn dp(row: &mut Vec<char>, conditions: &Vec<i64>, memo: &mut HashMap<(usize, usize), i64>, i: usize, j: usize) -> i64 {
        if i >= row.len() && j >= conditions.len() { // empty row and empty conditions
            return 1;
        }
        if i >= row.len() && j < conditions.len() { // empty row can't satisfy non-empty conditions
            return 0;
        }
        if i < row.len() && j >= conditions.len() { // check if row doesn't contain any blocks
            for k in i..row.len() {
                if row[k] == '#' {
                    return 0;
                }
            }
            return 1;
        }
        if memo.contains_key(&(i, j)) {
            return memo[&(i, j)];
        }
        let mut ans = 0;

        // find the largest X? block that is larger than the j'th condition
        let mut k = i;
        let mut sum = 0;
        let mut contains_block = false;
        while k < row.len() {
            if row[k] == '?' || row[k] == '#' {
                sum += 1;
                if row[k] == '#' {
                    contains_block = true;
                }
            } else {
                if sum >= conditions[j] {
                    break;
                }
                if contains_block {
                    memo.insert((i, j), 0);
                    return 0;
                }
                sum = 0;
            }
            k += 1;
        }
        if sum < conditions[j]{
            memo.insert((i, j), 0);
            return 0;
        }
        for s in (k-sum as usize)..(k - conditions[j] as usize + 1) {
            if (s + conditions[j] as usize) < row.len() && row[s + conditions[j] as usize] == '#' {
                if row[s] == '#' {
                    contains_block = true;
                    break;
                }
                continue;
            }
            ans += dp(row, conditions, memo, s + conditions[j] as usize + 1, j + 1);
            if row[s] == '#' {
                contains_block = true;
                break;
            }
        }
        
        if !contains_block {
            ans += dp(row, conditions, memo, k, j);
        }

        memo.insert((i, j), ans);
        return ans;
    }

    for line in reader.lines() {
        let line = line.unwrap();
        
        let (row, conditions) = line.split_once(" ").unwrap();
        let mut row: Vec<char> = row.chars().collect();
        let mut conditions = conditions.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let mut memo: HashMap<(usize, usize), i64> = HashMap::new();

        row.push('?');
        row = row.repeat(5);
        row.pop(); // remove last element
        conditions = conditions.repeat(5);

        ans += dp(&mut row, &conditions, &mut memo, 0, 0);
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
