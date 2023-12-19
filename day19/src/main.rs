use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Condition {
    value: i32,
    criterion: char,
    field: char,
    jump_to: String
}

#[derive(Debug)]
struct Rule {
    conditions: Vec<Condition>,
    default: String
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32
}

trait GetPart {
    fn get(&self, field: char) -> i32;
}

impl GetPart for Part {
    fn get(&self, field: char) -> i32 {
        match field {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid field {}", field)
        }
    }
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut ans = 0;
    let mut parse_rules = true;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            parse_rules = false;
            continue;
        }
        
        if parse_rules {
            let matches = regex::Regex::new(r"^(.*)\{(.*)\}$").unwrap().captures(&line).unwrap();
            let rule_name = matches.get(1).unwrap().as_str();
            let rule_conditions = matches.get(2).unwrap().as_str();

            let mut conditions: Vec<Condition> = Vec::new();
            let mut default = "";
            for condition in rule_conditions.split(",") {
                let (criterion, path) = condition.split_once(":").unwrap_or(("", condition));
                if criterion == "" {
                    default = path;
                } else {
                    let field = criterion.chars().nth(0).unwrap();
                    let operator = criterion.chars().nth(1).unwrap();
                    let value = criterion[2..].parse::<i32>().unwrap();
                    let condition = Condition {
                        value: value,
                        criterion: operator,
                        field: field,
                        jump_to: path.to_string()
                    };
                    conditions.push(condition);
                }
            }
            let rule = Rule {
                conditions: conditions,
                default: default.to_string()
            };
            rules.insert(rule_name.to_string(), rule);
        } else {
            let matches = regex::Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap().captures(&line).unwrap();
            let x = matches.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let m = matches.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let a = matches.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let s = matches.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let part = Part {
                x: x,
                m: m,
                a: a,
                s: s
            };
            parts.push(part);
        }
    }

    for part in parts {
        let mut rule_name = "in".to_string();
        loop {
            if rule_name == "A" {
                ans += part.x + part.m + part.a + part.s;
                break;
            }
            if rule_name == "R" { break; }
            let rule = rules.get(&rule_name).unwrap();
            let mut condition_met = false;
            for condition in &rule.conditions {
                let value = part.get(condition.field);
                match condition.criterion {
                    '>' => {
                        if value > condition.value {
                            condition_met = true;
                            rule_name = condition.jump_to.clone();
                            break;
                        }
                    },
                    '<' => {
                        if value < condition.value {
                            condition_met = true;
                            rule_name = condition.jump_to.clone();
                            break;
                        }
                    },
                    _ => panic!("Invalid criterion {}", condition.criterion)
                }
            }
            if !condition_met {
                rule_name = rule.default.clone();
            }
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
