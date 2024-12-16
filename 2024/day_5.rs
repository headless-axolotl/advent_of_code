#![allow(unused)]
use std::{fs::File, io::Read};

const PATH_1: &str = "input_1.txt";
const PATH_2: &str = "input_2.txt";

fn main() {
    let mut file = File::open(PATH_1).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf);

    // solve_part_one(buf);
    solve_part_two(buf);
}

type Mat = [[bool; 128]; 128];

fn check(rules: &Mat, update: &[usize]) -> bool {
    for current in 1..update.len() {
        let incorrect = (0..current).any(|previous| rules[update[current]][update[previous]]);
        if incorrect {
            return false;
        }
    }
    true
}

fn process_input(input: String) -> (Mat, Vec<Vec<usize>>) {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let str_rules = split[0];
    let str_updates = split[1];

    let mut result = 0;

    let mut rules = [[false; 128]; 128];
    let mut pair: Vec<usize>;
    for str_rule in str_rules.lines() {
        pair = str_rule
            .trim()
            .split('|')
            .map(|s| s.parse().unwrap())
            .collect();
        rules[pair[0]][pair[1]] = true;
    }

    let updates = str_updates
        .lines()
        .map(|str_update| {
            str_update
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

fn solve_part_one(input: String) {
    let mut result = 0;
    let (rules, mut updates) = process_input(input);

    for update in updates {
        if !check(&rules, &update) {
            continue;
        }

        result += update[update.len() >> 1];
    }

    println!("Answer: {result}");
}

use std::cmp::Ordering;
fn solve_part_two(input: String) {
    let mut result = 0;
    let (rules, updates) = process_input(input);

    for mut update in updates {
        if check(&rules, &update) {
            continue;
        }

        update.sort_by(|a, b| if rules[*a][*b] {Ordering::Less} else {Ordering::Equal});

        result += update[update.len() >> 1];
    }

    println!("Answer: {result}");
}
