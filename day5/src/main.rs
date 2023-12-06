use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;
use std::cmp;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut seeds: Vec<i64> = vec![];
    let mut new_seeds: Vec<i64> = vec![];
    let mut used: Vec<i64> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        if seeds.len() == 0 {
            let (_, sds) = line.split_once(": ").unwrap();
            for sd in sds.split(" ") {
                let seed = sd.parse::<i64>().unwrap();
                seeds.push(seed);
            }
        }
        else if line.contains(":") {
            if new_seeds.len() > 0 {
                for i in 0..seeds.len() {
                    let el_i = i as i64;
                    if !used.contains(&el_i) {
                        new_seeds.push(seeds[i]);
                    }
                }
                seeds = new_seeds;
            }
            new_seeds = vec![];
            used = vec![];
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let (destination, source, range) = match &parts[..] {
                [destination, source, range] => (destination, source, range),
                _ => panic!("Unexpected number of parts"), // or handle error differently
            };
            let destination = destination.parse::<i64>().unwrap();
            let source = source.parse::<i64>().unwrap();
            let range = range.parse::<i64>().unwrap();

            for i in 0..seeds.len() {
                let el_i = i as i64;
                if used.contains(&el_i) {
                    continue;
                }
                let seed = seeds[i];
                if seed >= source && seed <= source + range {
                    used.push(i as i64);
                    new_seeds.push(destination + (seed - source));
                }
            }
        }
    }

    println!("Part 1: {}", seeds.iter().min().unwrap());

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut seeds: Vec<(i64, i64)> = vec![];
    let mut loaded_seeds = false;
    let mut new_seeds: Vec<(i64, i64)> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        if !loaded_seeds {
            let (_, sds) = line.split_once(": ").unwrap();
            for (sd, range) in sds.split(" ").tuples() {
                let seed = sd.parse::<i64>().unwrap();
                let range = range.parse::<i64>().unwrap();

                seeds.push((seed, seed + range - 1));
            }
            loaded_seeds = true;
        }
        else if line.contains(":") {
            for new_seed in new_seeds {
                seeds.push(new_seed);
            }
            new_seeds = vec![];
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let (destination, source, range) = match &parts[..] {
                [destination, source, range] => (destination, source, range),
                _ => panic!("Unexpected number of parts"), // or handle error differently
            };
            let destination = destination.parse::<i64>().unwrap();
            let source = source.parse::<i64>().unwrap();
            let range = range.parse::<i64>().unwrap();

            let new_interval = (source, source + range - 1);

            for i in (0..seeds.len()).rev() {
                let seed = seeds[i];
                if seed.0 >= new_interval.0 && seed.1 <= new_interval.1 {
                    // |--------| - mapping interval
                    //   |----|   - seed interval
                    new_seeds.push((destination + (seed.0 - new_interval.0), destination + (seed.1 - new_interval.0)));
                    seeds.remove(i);
                } else if new_interval.0 >= seed.0 && new_interval.1 <= seed.1 {
                    //   |----|   - mapping interval
                    // |--------| - seed interval
                    new_seeds.push((destination, destination + (range - 1)));
                    seeds.remove(i);
                    seeds.push((seed.0, new_interval.0 - 1));
                    seeds.push((new_interval.1 + 1, seed.1));
                } else if seed.0 >= new_interval.0 && seed.0 <= new_interval.1 {
                    // |----|     - mapping interval
                    //   |----|   - seed interval
                    new_seeds.push((destination + (seed.0 - new_interval.0), destination + (new_interval.1 - new_interval.0)));
                    seeds.remove(i);
                    seeds.push((new_interval.1 + 1, seed.1));
                } else if seed.1 >= new_interval.0 && seed.1 <= new_interval.1 {
                    //     |----| - mapping interval
                    //   |----|   - seed interval
                    new_seeds.push((destination, destination + (seed.1 - new_interval.0)));
                    seeds.remove(i);
                    seeds.push((seed.0, new_interval.0 - 1));
                }
            }
        }
    }

    let mut ans = i64::MAX;
    for seed in seeds {
        ans = cmp::min(ans, seed.0);
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
