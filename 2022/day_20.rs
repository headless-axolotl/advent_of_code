#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

// for part 1 replace with 1
const KEY: i64 = 811589153;
const MIX_COUNT: i64 = 10;

fn main() {
    solve();
}

fn solve() {
    let mut file = File::open(PATH).unwrap();
    solve_part_one(file);
}

fn solve_part_one(file: File) {

    let order = generate_sequence(file);
    let mut items = generate_indeces(&order);

    reorder(&mut items, &order);

    let mut counter = 1_000;
    let mut answer = 0;
    let zero_order = order.iter().position(|&at| at == 0).unwrap() as i64;
    let zero = items.iter().position(|&at| at == zero_order).unwrap();
    while counter <= 3_000 {
        let val = order[items[
            (zero + counter).rem_euclid(items.len())
        ] as usize ] * KEY;
        println!("{}", val);
        answer += val;
        counter += 1_000;
    }
    println!("{}", answer);
}

fn generate_sequence(file: File) -> Vec<i64> {

    let mut output: Vec<i64> = Vec::new();

    for option_line in BufReader::new(file).lines() {
        let line = option_line.unwrap();
        output.push(line.parse().unwrap());
    }
    output
}

fn generate_indeces(order: &Vec<i64>) -> Vec<i64> {
    
    let mut indeces: Vec<i64> = vec![];
    for i in (0..order.len() as i64) {
        indeces.push(i);
    }
    
    indeces
}

fn reorder(
    items: &mut Vec<i64>,
    order: &Vec<i64>,
) {
    // got some help from a solution on reddit
    // there also were repeating numbers which threw my algorithm off
    let item_count = order.len() as i64;
    for _ in (0..MIX_COUNT) {
        for i in 0..item_count {
            let index = items.iter().position(|&at| at == i).unwrap();
            items.remove(index);
            let new_index = ((index as i64) + order[i as usize] * KEY).rem_euclid(item_count - 1);
            items.insert(new_index as usize, i);
        }
        println!("amount");
    }
}
