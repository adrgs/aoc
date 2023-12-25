use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Brick {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
    z_start: i32,
    z_end: i32,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;
    let mut bricks: Vec<Brick> = Vec::new();
    let mut unsorted_bricks: Vec<Brick> = Vec::new();

    fn intervals_overlap(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
        a_start <= b_end && b_start <= a_end
    }

    for line in reader.lines() {
        let line = line.unwrap();
        let (start, end) = line.split_once("~").unwrap();
        
        let values_start: Vec<i32> = start.split(',')
        .map(|s| s.parse().unwrap())
        .collect();

        let values_end: Vec<i32> = end.split(',')
        .map(|s| s.parse().unwrap())
        .collect();

        let brick = Brick {
            x_start: values_start[0],
            x_end: values_end[0],
            y_start: values_start[1],
            y_end: values_end[1],
            z_start: values_start[2],
            z_end: values_end[2],
            supports: Vec::new(),
            supported_by: Vec::new(),
        };

        unsorted_bricks.push(brick);
    }

    unsorted_bricks.sort_by(|a, b| a.z_start.cmp(&b.z_start));

    for brick in unsorted_bricks.iter_mut() {
        let mut found = false;

        while !found && brick.z_start > 1 {
            let l = bricks.len();
            for i in 0..bricks.len() {
                let b = &bricks[i];
                if b.z_end != brick.z_start - 1 {
                    continue;
                }
                if intervals_overlap(brick.x_start, brick.x_end, b.x_start, b.x_end) &&
                    intervals_overlap(brick.y_start, brick.y_end, b.y_start, b.y_end) {
                        brick.supported_by.push(i);
                        bricks[i].supports.push(l);
                        found = true;
                    }
            }

            if found {
                break;
            }

            brick.z_start -= 1;
            brick.z_end -= 1;
        }

        bricks.push(brick.clone());
    }

    for i in 0..bricks.len() {
        let mut can_be_removed = true;
        for idx in &bricks[i].supports {
            if bricks[*idx].supported_by.len() == 1 {
                can_be_removed = false;
                break;
            }
        }

        if can_be_removed {
            ans += 1;
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
    let mut bricks: Vec<Brick> = Vec::new();
    let mut unsorted_bricks: Vec<Brick> = Vec::new();

    fn intervals_overlap(a_start: i32, a_end: i32, b_start: i32, b_end: i32) -> bool {
        a_start <= b_end && b_start <= a_end
    }

    for line in reader.lines() {
        let line = line.unwrap();
        let (start, end) = line.split_once("~").unwrap();
        
        let values_start: Vec<i32> = start.split(',')
        .map(|s| s.parse().unwrap())
        .collect();

        let values_end: Vec<i32> = end.split(',')
        .map(|s| s.parse().unwrap())
        .collect();

        let brick = Brick {
            x_start: values_start[0],
            x_end: values_end[0],
            y_start: values_start[1],
            y_end: values_end[1],
            z_start: values_start[2],
            z_end: values_end[2],
            supports: Vec::new(),
            supported_by: Vec::new(),
        };

        unsorted_bricks.push(brick);
    }

    unsorted_bricks.sort_by(|a, b| a.z_start.cmp(&b.z_start));

    for brick in unsorted_bricks.iter_mut() {
        let mut found = false;

        while !found && brick.z_start > 1 {
            let l = bricks.len();
            for i in 0..bricks.len() {
                let b = &bricks[i];
                if b.z_end != brick.z_start - 1 {
                    continue;
                }
                if intervals_overlap(brick.x_start, brick.x_end, b.x_start, b.x_end) &&
                    intervals_overlap(brick.y_start, brick.y_end, b.y_start, b.y_end) {
                        brick.supported_by.push(i);
                        bricks[i].supports.push(l);
                        found = true;
                    }
            }

            if found {
                break;
            }

            brick.z_start -= 1;
            brick.z_end -= 1;
        }

        bricks.push(brick.clone());
    }

    let mut destroyed: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];

    fn chain(bricks: &Vec<Brick>, destroyed: &mut HashSet<usize>, idx: usize) {
        if destroyed.contains(&idx) {
            return;
        }

        let mut okay = true;
        for i in &bricks[idx].supported_by {
            if !destroyed.contains(i) {
                okay = false;
                break;
            }
        }

        if !okay {
            return;
        }

        destroyed.insert(idx);
        for i in &bricks[idx].supports {
            chain(bricks, destroyed, *i);
        }
    }

    for i in 0..bricks.len() {
        if bricks[i].supported_by.len() == 1 {
            destroyed[bricks[i].supported_by[0]].insert(i);
        }
    }

    for i in 0..bricks.len() {
        for j in destroyed[i].clone() {
            for k in &bricks[j].supports {
                chain(&bricks, &mut destroyed[i], *k);
            }
        }
    }

    for i in 0..bricks.len() {
        ans += destroyed[i].len();
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
