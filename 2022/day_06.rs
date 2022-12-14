#![allow(unused)]
use std::{fs::File, io::{BufRead, BufReader}};
use std::collections::HashMap;

fn main() {

    let path = "input.txt";
    
    let file = match File::open(path) {
        Ok(val) => val,
        Err(_) => panic!("couldn't open file!"),
    };
    solve(file);
}

fn solve(file: File) {

    let lines = BufReader::new(file).lines();
    let mut answer : usize = 0;

    for option_line in lines {
        if let Ok(line) = option_line {
            answer = process_line(&line, 4); // part 1
            // answer = process_line(&line, 14); // part 2
        }
    }

    println!("{answer}");
}

fn process_line(line: &str, distinct_in_a_row: usize) -> usize {
    
    let mut encountered : HashMap<char, i32> = HashMap::new();
    let characters : Vec<char> = line.chars().collect();
    let mut non_distinct_count : i32 = 0;

    let mut counter : i32 = 0;

    for pre_process_index in 0..(distinct_in_a_row - 1) {
        counter+=1;
        let mut entry = encountered.entry(characters[pre_process_index]).or_insert(0);
        *entry += 1;
        if *entry == 2 {
            non_distinct_count += 1;
        }
    }

    for i in (distinct_in_a_row - 1)..characters.len() {

        let new_entry = encountered.entry(characters[i]).or_insert(0);
        *new_entry += 1;
        if *new_entry == 2 {
            non_distinct_count += 1;
        }

        if non_distinct_count == 0 {
            return (i + 1);
        }

        let old_entry = encountered.entry(characters[i + 1 - distinct_in_a_row]).or_insert(0);
        *old_entry -= 1;
        if *old_entry == 1 {
            non_distinct_count -= 1;
        }
    }

    panic!("exited without answer!");
}
