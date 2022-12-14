use std::{io::{BufRead, BufReader}, fs::File};

fn main() {
    
    let path = "input.txt";
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("unable to open file"),
    };

    let lines = BufReader::new(file);
    let mut total : i32 = 0;

    for line in lines.lines() {
        if let Ok(value) = line {
            total += calculate_total_first_part(&value);
            // total += calculate_total_second_part(&value);
        }
    }

    println!("{total}");
}

// RPS - ABC - XYZ - 123
fn calculate_total_first_part(line: &str) -> i32 {
    
    let mut chars = line.chars();
    let opponent = match chars.nth(0) {
        Some(value) => letter_to_integer(value),
        None => 0,
    };
    let me = match chars.nth(1) {
        Some(value) => letter_to_integer(value),
        None => 0,
    };
    
    if  opponent == 0 || me == 0 {
        return 0;
    }
    let result = (me - opponent + 3) % 3;
    match result {
        0 => 3 + me,
        1 => 6 + me,
        2 => me,
        _ => 0,
    }
}
fn calculate_total_second_part(line: &str) -> i32 {
    
    let mut chars = line.chars();
    let opponent = match chars.nth(0) {
        Some(value) => letter_to_integer(value),
        None => 0,
    };
    let me = match chars.nth(1) {
        Some(value) => value,
        None => ' ',
    };
    
    if  opponent == 0 || me == ' ' {
        return 0;
    }
    match me {
        'X' => shift(opponent, 3, -1),
        'Y' => 3 + opponent,
        'Z' => 6 + shift(opponent, 3, 1),
        _ => 0,
    }
}

fn letter_to_integer(letter: char) -> i32 {
    match letter {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}
// RPS - ABC - XYZ - 123
// to win: 123 - 231
// to lose: 123 - 312
fn shift(number: i32, max: i32, amount: i32) -> i32 {
    let result = (number + amount) % max;
    match result {
        0 => max,
        _ => result
    }
}
