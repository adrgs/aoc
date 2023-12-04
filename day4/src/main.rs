use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, our_numbers) = numbers.split_once(" | ").unwrap();
        let mut exponent = -1;
        let winning: HashSet<i32> = winning_numbers.split_whitespace().filter_map(|x| {
            x.parse::<i32>().ok()
        }).collect();

        for number in our_numbers.split(" ") {
            let num = number.parse::<i32>();
            match num {
                Ok(num) => {
                    if winning.contains(&num) {
                        exponent += 1
                    }
                },
                Err(_) => {}
            }
        }

        if exponent >= 0 {
            ans += 2_i32.pow(exponent as u32);
        }
        
    }

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut scratchcards = vec![1; 300];
    scratchcards[0] = 0;

    let mut last_card_id = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let (card, numbers) = line.split_once(": ").unwrap();

        let card_id = card.split_whitespace().last().unwrap().parse::<usize>().unwrap();
        last_card_id = card_id;
        if scratchcards[card_id] == 0 {
            continue;
        }

        let (winning_numbers, our_numbers) = numbers.split_once(" | ").unwrap();
        let mut won = 0;
        let winning: HashSet<i32> = winning_numbers.split_whitespace().filter_map(|x| {
            x.parse::<i32>().ok()
        }).collect();

        for number in our_numbers.split(" ") {
            let num = number.parse::<i32>();
            match num {
                Ok(num) => {
                    if winning.contains(&num) {
                        won += 1
                    }
                },
                Err(_) => {}
            }
        }

        for i in card_id+1..card_id+won+1 {
            scratchcards[i] += scratchcards[card_id];
        }
        
    }

    for i in last_card_id+1..scratchcards.len() {
        scratchcards[i] = 0;
    }

    let ans: i32 = scratchcards.iter().sum();

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
