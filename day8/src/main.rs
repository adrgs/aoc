use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let steps = lines.next().unwrap()?;
    let graph: HashMap<String, (String, String)> = lines
        .filter_map(|line| {
            let line = line.ok()?;
            let (node, edges) = line.split_once(" = ")?;
            let (edge1, edge2) = edges.split_once(", ")?;
            let edge1 = edge1.trim_matches('(').to_string();
            let edge2 = edge2.trim_matches(')').to_string();
            Some((node.to_string(), (edge1, edge2)))
        })
        .collect();

    let mut node = "AAA";
    let mut ans = 0;
    while node != "ZZZ" {
        let next_node = &graph[node];
        node = if steps.chars().nth(ans % steps.len()).unwrap() == 'R' {
            &next_node.1
        } else {
            &next_node.0
        };
        ans += 1;
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
