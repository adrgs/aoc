use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop, Vec<String>),
    Conjunction(Conjunction, Vec<String>),
    Broadcast(Vec<String>),
    Output,
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<String, bool>,
}

#[derive(Debug)]
struct Output {}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut module_outputs: Vec<(String, String)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut module: Module = match line.chars().nth(0).unwrap() {
            '%' => Module::FlipFlop(FlipFlop { state: false }, Vec::new()),
            '&' => Module::Conjunction(
                Conjunction {
                    inputs: HashMap::new(),
                },
                Vec::new(),
            ),
            _ => Module::Broadcast(Vec::new()),
        };
        let mut name = String::new();

        let re = regex::Regex::new(r"\w+").unwrap();
        for cap in re.captures_iter(&line) {
            if name == "" {
                name = cap[0].to_string();
            } else {
                match &mut module {
                    Module::FlipFlop(_, outputs) => {
                        outputs.push(cap[0].to_string());
                    }
                    Module::Conjunction(_conjunction, outputs) => {
                        outputs.push(cap[0].to_string());
                    }
                    Module::Broadcast(outputs) => {
                        outputs.push(cap[0].to_string());
                    },
                    _ => {panic!("Invalid module type")}
                }
                module_outputs.push((name.to_string(), cap[0].to_string()));
            }
        }
        modules.insert(name, module);
    }

    for (parent, dest) in module_outputs {
        if !modules.contains_key(&dest) {
            modules.insert(dest.clone(), Module::Output);
        }
        match modules.get_mut(&dest).unwrap() {
            Module::Conjunction(conjunction, _outputs) => {
                conjunction.inputs.insert(parent, false);
            }
            _ => {}
        }
    }

    let mut event_queue: VecDeque<(String, String, bool)> = VecDeque::new();
    let mut events = vec![0; 2];

    let button_presses = 1000;

    for _ in 0..button_presses {
        event_queue.push_back(("broadcaster".to_string(), "button".to_string(), false));

        while event_queue.len() > 0 {
            let (name, parent, value) = event_queue.pop_front().unwrap();

            //println!("{} -{}-> {}", parent, value, name);

            events[value as usize] += 1;
            let module = modules.get_mut(&name).unwrap();

            match module {
                Module::FlipFlop(flip_flop, outputs) => {
                    if value {
                        continue;
                    }
                    flip_flop.state = !flip_flop.state;
                    for output in outputs {
                        event_queue.push_back((output.clone(), name.clone(), flip_flop.state));
                    }
                }
                Module::Conjunction(conjunction, outputs) => {
                    conjunction.inputs.insert(parent.clone(), value);
                    if conjunction.inputs.values().all(|&x| x) {
                        for output in outputs {
                            event_queue.push_back((output.clone(), name.clone(), false));
                        }
                    } else {
                        for output in outputs {
                            event_queue.push_back((output.clone(), name.clone(), true));
                        }
                    }
                }
                Module::Broadcast(outputs) => {
                    for output in outputs {
                        event_queue.push_back((output.clone(), name.clone(), value));
                    }
                }
                Module::Output => {}
            }
        }
    }

    let ans = events[0] * events[1];
    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut module_outputs: Vec<(String, String)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut module: Module = match line.chars().nth(0).unwrap() {
            '%' => Module::FlipFlop(FlipFlop { state: false }, Vec::new()),
            '&' => Module::Conjunction(
                Conjunction {
                    inputs: HashMap::new(),
                },
                Vec::new(),
            ),
            _ => Module::Broadcast(Vec::new()),
        };
        let mut name = String::new();

        let re = regex::Regex::new(r"\w+").unwrap();
        for cap in re.captures_iter(&line) {
            if name == "" {
                name = cap[0].to_string();
            } else {
                match &mut module {
                    Module::FlipFlop(_, outputs) => {
                        outputs.push(cap[0].to_string());
                    }
                    Module::Conjunction(_conjunction, outputs) => {
                        outputs.push(cap[0].to_string());
                    }
                    Module::Broadcast(outputs) => {
                        outputs.push(cap[0].to_string());
                    },
                    _ => {panic!("Invalid module type")}
                }
                module_outputs.push((name.to_string(), cap[0].to_string()));
            }
        }
        modules.insert(name, module);
    }

    for (parent, dest) in module_outputs {
        if !modules.contains_key(&dest) {
            modules.insert(dest.clone(), Module::Output);
        }
        match modules.get_mut(&dest).unwrap() {
            Module::Conjunction(conjunction, _outputs) => {
                conjunction.inputs.insert(parent, false);
            }
            _ => {}
        }
    }

    let mut event_queue: VecDeque<(String, String, bool)> = VecDeque::new();
    let mut events = vec![0; 2];

    let module_before_rx_name = "lb";
    let mut counter: HashMap<String, i32> = HashMap::new();

    let mut button_presses = 0;

    loop {
        event_queue.push_back(("broadcaster".to_string(), "button".to_string(), false));
        button_presses += 1;

        while event_queue.len() > 0 {
            let (name, parent, value) = event_queue.pop_front().unwrap();

            if name == module_before_rx_name && value == true {
                if counter.contains_key(&parent) {
                    continue;
                } else {
                    counter.insert(parent.clone(), button_presses);
                }
            }

            events[value as usize] += 1;
            let module = modules.get_mut(&name).unwrap();

            match module {
                Module::FlipFlop(flip_flop, outputs) => {
                    if value {
                        continue;
                    }
                    flip_flop.state = !flip_flop.state;
                    for output in outputs {
                        event_queue.push_back((output.clone(), name.clone(), flip_flop.state));
                    }
                }
                Module::Conjunction(conjunction, outputs) => {
                    conjunction.inputs.insert(parent.clone(), value);
                    if conjunction.inputs.values().all(|&x| x) {
                        for output in outputs {
                            event_queue.push_back((output.clone(), name.clone(), false));
                        }
                    } else {
                        for output in outputs {
                            event_queue.push_back((output.clone(), name.clone(), true));
                        }
                    }
                }
                Module::Broadcast(outputs) => {
                    for output in outputs {
                        event_queue.push_back((output.clone(), name.clone(), value));
                    }
                }
                Module::Output => {}
            }
        }
        let module_before_rx = modules.get_mut(module_before_rx_name).unwrap();

        match module_before_rx {
            Module::Conjunction(conjunction, _outputs) => {
                if conjunction.inputs.len() == counter.len() {
                    break;
                }
            }
            _ => {}
        }
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            return a;
        }
        return gcd(b, a % b);
    }

    let values = counter.values().collect::<Vec<&i32>>();

    let ans: i64 = values.iter().fold(1, |acc, x| (acc * (**x) as i64) / gcd(acc, **x as i64));
    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
