use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn hash(input: &str) -> i32 {
    let mut val = 0;
    for c in input.chars() {
        val += c as i32;
        val *= 17;
        val %= 256;
    }

    val
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    

    let input = reader.lines().take(1).next().unwrap().unwrap();
    for step in input.split(',') {
        ans += hash(step);
    }


    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let mut hashmap: Vec<Vec<(&str, i32)>> = Vec::new();
    for _ in 0..256 {
        hashmap.push(Vec::new());
    }

    let input = reader.lines().take(1).next().unwrap().unwrap();
    for step in input.split(',') {
        let mut elements: Vec<_> = step.split_terminator(&['-', '='][..]).collect();
        if elements.len() < 2 {
            elements.push("0");
        }

        for window in elements.windows(2) {
            let label = window[0];
            let value = window[1].parse::<i32>().unwrap_or_default();
            let box_id = hash(label);
            let add = step.find('=');
            if add.is_some() {
                let mut found = false;
                for i in 0..hashmap[box_id as usize].len() {
                    if hashmap[box_id as usize][i].0 == label {
                        hashmap[box_id as usize][i].1 = value;
                        found = true;
                        break;
                    }
                }
                if !found {
                    hashmap[box_id as usize].push((label, value));
                }
            } else {
                // remove label from hashmap
                let mut new_hashmap = Vec::new();
                for (lb, value) in hashmap[box_id as usize].iter() {
                    if label != *lb {
                        new_hashmap.push((*lb, *value));
                    }
                }
                hashmap[box_id as usize] = new_hashmap;
            }
        }
    }

    for i in 0..hashmap.len() {
        for j in 0..hashmap[i].len() {
            ans += (i as i32 + 1) * (j as i32 + 1) * hashmap[i][j].1;
        }
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
