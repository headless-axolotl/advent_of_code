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

fn process_input(input: String) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split([':', ' '])
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn can_be_corrected_v1(entry: &[u64]) -> bool {
    let op_count = entry.len() - 2;

    let mut calculation;
    for set in 0..(1 << op_count) {
        calculation = entry[1];
        for i in 0..op_count {
            if set & (1 << i) != 0 {
                calculation += entry[i + 2];
            } else {
                calculation *= entry[i + 2];
            }
        }
        if calculation == entry[0] {
            return true
        }
    }
    false
}

fn solve_part_one(input: String) {
    let data = process_input(input);

    let mut result = 0;
    for entry in &data {
        if can_be_corrected_v1(entry) {
            result += entry[0];
        }
    }

    println!("Answer {result}");
}

fn conc(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn can_be_corrected_v2(entry: &[u64]) -> bool {
    let op_count = entry.len() - 2;

    let mut calculation; let mut op;
    for mut set in 0..3u32.pow(op_count as u32) {
        calculation = entry[1];

        for i in 0..op_count {
            op = set % 3; set /= 3;
            match op {
                0 => calculation += entry[i + 2],
                1 => calculation *= entry[i + 2],
                _ => calculation = conc(calculation, entry[i + 2]),
            };
        }
        if calculation == entry[0] {
            return true
        }
    }
    false
}

fn solve_part_two(input: String) {
    let data = process_input(input);

    let mut result = 0;
    for entry in &data {
        if can_be_corrected_v2(entry) {
            result += entry[0];
        }
    }

    println!("Answer {result}");
}
