#![allow(unused)]
use std::{
    fs::File,
    io::Read,
};


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
    let (mut l, mut r) = process_input(input);
    
    l.sort(); r.sort();
    let mut result = 0;
    for i in 0..l.len() {
        result += (l[i]-r[i]).abs();
    }

    println!("Answer: {}", result);
}

fn solve_part_two(input: String) {
    let (mut l, mut r) = process_input(input);

    l.sort(); r.sort();
    let mut result = 0;
    
    let mut id = -1;
    let mut id_idx = 0; // Index to the *next* element that should be counted.
    let mut count = 0;

    let mut prev = -1;

    // for i in 0..l.len() {
    //     println!("{}-{}", l[i], r[i]);
    // }

    for i in l {
        if id_idx >= r.len() {
            break;
        }

        while i > id {
            let start = id_idx;
            id = r[id_idx];

            'counting: loop {
                id_idx += 1;
                if id_idx >= r.len() || r[id_idx] != id {
                    break 'counting;
                }
            }

            count = id_idx - start;

            // println!("!> {}", id_idx);
        }

        if i == id {
            result += i * count as i32;
        }
    }

    println!("Answer: {}", result);
}

fn process_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut left = Vec::with_capacity(lines.len());
    let mut right = Vec::with_capacity(lines.len());

    for line in lines {
        let res = line.split(" ").collect::<Vec<_>>();
        left.push(res[0].trim().parse().unwrap());
        right.push(res[3].trim().parse().unwrap());
    }

    (left, right)
}

