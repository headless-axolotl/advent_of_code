#![allow(unused)]
use std::{fs::File, io::{BufReader, BufRead}};

struct ParsedLine((i32, i32), (i32, i32));

fn main() {
    let path = "input.txt";
    let mut file = match File::open(path) {
        Ok(value) => value,
        Err(_) => panic!("couldn't open file"),
    };
    solve_part_one(file);

    file = match File::open(path) {
        Ok(value) => value,
        Err(_) => panic!("couldn't open file"),
    };
    solve_part_two(file);
}

fn solve_part_one(file: File) {
    let lines = BufReader::new(file).lines();
    let mut answer : i32 = 0;
    
    for option_line in lines {
        if let Ok(line) = option_line {
            if either_is_fully_contained(process_line(&line)) {
                answer += 1;
            }
        }
    }

    println!("{answer}");
}

fn solve_part_two(file: File) {
    let lines = BufReader::new(file).lines();
    let mut answer : i32 = 0;
    
    for option_line in lines {
        if let Ok(line) = option_line {
            if are_overlapping(process_line(&line)) {
                answer += 1;
            }
        }
    }

    println!("{answer}");
}

fn process_line(line: &str) -> ParsedLine {

    let comma = match line.find(',') {
        Some(val) => val,
        None => 0,
    };
    let first_dash = match line[0..comma].find('-') {
        Some(val) => val,
        None => 0,
    };
    let second_dash = match line[comma..line.len()].find('-') {
        Some(val) => val+comma,
        None => comma,
    };

    let f_0 = line[0                ..  first_dash].parse::<i32>().unwrap();
    let f_1 = line[first_dash + 1   ..       comma].parse::<i32>().unwrap();
    let s_0 = line[comma + 1        .. second_dash].parse::<i32>().unwrap();
    let s_1 = line[second_dash + 1  ..  line.len()].parse::<i32>().unwrap();
    
    ParsedLine((f_0, f_1), (s_0, s_1))
}

fn either_is_fully_contained(parsed_line: ParsedLine) -> bool {
    (parsed_line.1.0 <= parsed_line.0.0 && parsed_line.0.1 <= parsed_line.1.1)
    || (parsed_line.0.0 <= parsed_line.1.0 && parsed_line.1.1 <= parsed_line.0.1)
}

fn are_overlapping(parsed_line: ParsedLine) -> bool {
    
    let min_left = match parsed_line.0.0 < parsed_line.1.0 {
        true => parsed_line.0.0,
        false => parsed_line.1.0,
    };
    let max_right = match parsed_line.0.1 > parsed_line.1.1 {
        true => parsed_line.0.1,
        false => parsed_line.1.1,
    };
    (max_right - min_left + 1) <
          (parsed_line.0.1 - parsed_line.0.0 + 1)
        + (parsed_line.1.1 - parsed_line.1.0 + 1)
}