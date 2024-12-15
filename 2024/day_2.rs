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

fn solve_part_one(input: String) {
    let reports = process_input(input);
    let answer = reports.iter().filter(|r| is_safe(r)).count();
    println!("Answer: {}", answer);
}

fn solve_part_two(input: String) {
    let reports = process_input(input);
    // let answer = reports.iter().filter(|r| is_safe_dampener(r)).count();
    let answer = reports.iter().filter(|r| is_safe_dampener_brute(r)).count();
    println!("Answer: {}", answer);
}

const RANGE: std::ops::RangeInclusive<i32> = 1..=3;

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let first = report[0];

    let mut prev = if first < report[1] {
        first - 1
    } else {
        first + 1
    };

    let sign = prev - first;

    let mut diff;
    for item in report {
        diff = prev - *item;
        if sign * diff < 0 || !RANGE.contains(&diff.abs()) {
            return false;
        }
        prev = *item;
    }

    true
}

fn is_safe_dampener_brute(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    
    let mut cut;
    for i in 0..report.len() {
        cut = [&report[..i], &report[i+1..]].concat();
        if is_safe(&cut) {
            return true;
        }
    }

    false
}

fn is_safe_dampener(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let mut prev = report[0];
    let mut diffs = Vec::with_capacity(report.len() - 1);

    for item in &report[1..] {
        diffs.push(prev - *item);
        prev = *item;
    }

    let mut idx = 0;
    let mut found_err = false;
    let mut eaten = diffs.len();

    while idx < diffs.len() {
        // println!("idx: {idx}");
        if RANGE.contains(&diffs[idx].abs()) {
            idx += 1;
            continue;
        }

        if found_err {
            return false;
        }

        found_err = true;

        // Try to fix the error by consuming the current measurement.
        if idx + 1 < diffs.len() && RANGE.contains(&(diffs[idx] + diffs[idx + 1]).abs()) {
            eaten = idx + 1;
            idx += 2;
            continue;
        }

        // Try to fix the error by consuming the previous measurement.
        if idx > 0 && RANGE.contains(&(diffs[idx - 1] + diffs[idx]).abs()) && idx - 1 != eaten {
            idx += 1;
            continue;
        }

        return false;
    }

    true
}

fn process_input(input: String) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut r;
    for line in input.lines() {
        let split = line.trim().split(" ");
        result.push(vec![]);
        r = result.last_mut().unwrap();
        for i in split {
            r.push(i.parse().unwrap());
        }
    }
    result
}
