use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use gcd::Gcd;

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

    let mut ans = 0;
    let nodes: Vec<&str> = graph
        .keys()
        .filter_map(|node| {
            if node.ends_with("A") {
                Some(node.as_str())
            } else {
                None
            }
        })
        .collect();

    let mut cycles: Vec<usize> = Vec::new();
    for node in nodes {
        let mut node = node;
        let mut cycle = 0;
        while !node.ends_with("Z") {
            let next_node = &graph[node];
            node = if steps.chars().nth((ans + cycle) % steps.len()).unwrap() == 'R' {
                &next_node.1
            } else {
                &next_node.0
            };
            cycle += 1;
        }
        cycles.push(cycle);
    }

    let lcm = cycles.iter().fold(1 as u128, |a, b| (a*(*b as u128))/a.gcd(*b as u128));

    println!("Part 2: {}", lcm);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
