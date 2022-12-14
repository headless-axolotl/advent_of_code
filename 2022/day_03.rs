#![allow(unused)]
use std::{fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;

fn main() {    
    let path = "input.txt";
    let file = match File::open(path) {
        Ok(value) => value,
        Err(_) => panic!("couldn't open input"),
    };

    // solve_part_one(&file);
    solve_part_two(&file);
}

fn solve_part_one(file: &File) {
    let lines = BufReader::new(file).lines();
    let mut answer : i32 = 0;

    for line in lines {
        if let Ok(placeholder) = line {
            answer += calc_v1(&placeholder);
        }
    }

    println!("{answer}");
}

fn solve_part_two(file: &File) {
    let lines = BufReader::new(file).lines();
    let mut answer : i32 = 0;
    let mut counter : usize = 0;
    let mut group : Vec<String> = vec![String::new(); 3];

    for line in lines {
        if counter > 2 {
            answer += calc_v2(&group);
            counter = 0;
        }
        if let Ok(placeholder) = line {
            group[counter] = placeholder;
        }
        counter += 1;
    }
    answer += calc_v2(&group);

    println!("{}", answer);
}

fn wdigit(c: char) -> u32 {
    match c.to_digit(36) {
        Some(val) => val,
        None => 0,
    }
}
fn get_value(c: char) -> i32 {
    
    let digit_value : i32 = wdigit(c).try_into().unwrap();
    
    if c.is_ascii_lowercase() {
        digit_value - 9
    }
    else {
        digit_value + 17 // - 9 + 26
    }
}

fn calc_v1(line: &str) -> i32 {
    let half_len: usize = line.len()/2;
    let mut counter : usize = 0;

    let mut encountered : HashMap<char, bool> = HashMap::new();
    for character in line.chars() {
        if(counter < half_len) {
            encountered.entry(character).or_insert(true);
        }
        else if let Some(val) = encountered.get(&character) {
            if *val {
                return get_value(character);
            }
        }
        counter += 1;
    }
    0
}

fn populate(line: &str, hash_map: &mut HashMap<char, i32>, flag: i32) {
    for character in line.chars() {
        let is_contained = hash_map.entry(character).or_insert(0);
        *is_contained |= flag;
    }
}
fn calc_v2(lines: &Vec<String>) -> i32 {

    let mut hash_map : HashMap<char, i32> = HashMap::new();
    populate(&lines[0], &mut hash_map, 0b01);
    populate(&lines[1], &mut hash_map, 0b10);
    
    for character in lines[2].chars() {
        let count = hash_map.entry(character).or_insert(0);
        if *count == 0b11 {
            return get_value(character);
        }
    }
    0
}

