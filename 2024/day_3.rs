#![allow(unused)]
use std::{fs::File, io::Read};
use regex::Regex; 

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
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result = 0;
    for (_, [lstr, rstr]) in re.captures_iter(&input).map(|c| c.extract()) {
        let lnum: i32 = lstr.parse().unwrap();
        let rnum: i32 = rstr.parse().unwrap();
        result += lnum * rnum;
    }
    println!("Answer: {result}");
}

fn solve_part_two(input: String) {
    let re = Regex::new(r"(mul)\(([0-9]+),([0-9]+)\)|(do)\(()()\)|(don't)\(()()\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for (_, [comm, lstr, rstr]) in re.captures_iter(&input).map(|c| c.extract()) {
        if comm == "do" {
            enabled = true;
            continue;
        }

        if comm == "don't" {
            enabled = false;
            continue;
        }

        if !enabled {
            continue;
        }

        let lnum: i32 = lstr.parse().unwrap();
        let rnum: i32 = rstr.parse().unwrap();
        result += lnum * rnum;
    }
    println!("Answer: {result}");
}
