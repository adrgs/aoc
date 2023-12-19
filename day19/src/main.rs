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

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i32,
    end: i32
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

    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let mut ans: i64 = 0;
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

    let mut queue: Vec<(&str, Interval, Interval, Interval, Interval)> = Vec::new();
    queue.push(("in", Interval { start: 1, end: 4000 }, Interval { start: 1, end: 4000 }, Interval { start: 1, end: 4000 }, Interval { start: 1, end: 4000 }));

    let mut accepted: Vec<(Interval, Interval, Interval, Interval)> = Vec::new();
    let mut rejected: Vec<(Interval, Interval, Interval, Interval)> = Vec::new();

    while queue.len() > 0 {
        let (rule, x, m, a, s) = queue.pop().unwrap();

        if x.start > x.end || m.start > m.end || a.start > a.end || s.start > s.end {
            continue;
        }

        if rule == "A" {
            // ans += (x.end - x.start + 1) as i64 * (m.end - m.start + 1) as i64 * (a.end - a.start + 1) as i64 * (s.end - s.start + 1) as i64;
            accepted.push((x, m, a, s));
            continue;
        } else if rule == "R" {
            rejected.push((x, m, a, s));
            continue;
        }

        let rule = rules.get(rule).unwrap();

        let mut x = x.clone();
        let mut m = m.clone();
        let mut a = a.clone();
        let mut s = s.clone();
        for condition in &rule.conditions {
            let value = match condition.field {
                'x' => &mut x,
                'm' => &mut m,
                'a' => &mut a,
                's' => &mut s,
                _ => panic!("Invalid field {}", condition.field)
            };
            let mut new_value = value.clone();
            match condition.criterion {
                '>' => {
                    if condition.value > value.start {
                        new_value.start = condition.value + 1;
                        value.end = condition.value;
                    }
                },
                '<' => {
                    if condition.value < value.end {
                        new_value.end = condition.value - 1;
                        value.start = condition.value;
                    }
                },
                _ => panic!("Invalid criterion {}", condition.criterion)
            }
            if new_value.start <= new_value.end {
                match condition.field {
                    'x' => queue.push((&condition.jump_to, new_value, m.clone(), a.clone(), s.clone())),
                    'm' => queue.push((&condition.jump_to, x.clone(), new_value, a.clone(), s.clone())),
                    'a' => queue.push((&condition.jump_to, x.clone(), m.clone(), new_value, s.clone())),
                    's' => queue.push((&condition.jump_to, x.clone(), m.clone(), a.clone(), new_value)),
                    _ => panic!("Invalid field {}", condition.field)
                }
            }
        }
        queue.push((&rule.default, x, m, a, s));
    }

    for accept in accepted {
        ans += (accept.0.end - accept.0.start + 1) as i64 * (accept.1.end - accept.1.start + 1) as i64 * (accept.2.end - accept.2.start + 1) as i64 * (accept.3.end - accept.3.start + 1) as i64;
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
