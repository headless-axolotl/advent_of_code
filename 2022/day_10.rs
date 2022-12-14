#![allow(unused)]
use std::{fs::File, io::{BufRead, BufReader}, collections::HashSet};
use core::ops::{Add, Sub, AddAssign};
use core::cmp::{Eq, PartialEq};

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

type ScreenBuffer = [[bool; 40]; 6];

fn main() {
    solve();
}

fn solve() {
    
    let mut file = File::open(PATH).unwrap();
    // let mut file = File::open(TEST).unwrap();
    let mut tick_values = generate(file);
    // println!("{:?}", tick_values);
    solve_part_one(&tick_values);

    let mut screen_buffer: ScreenBuffer = [[false; 40]; 6];
    render(&mut screen_buffer, &tick_values);
    print_screen(&screen_buffer);
}

fn generate(file: File) -> Vec<i32> {
    let mut values: Vec<i32> = vec![];
    let mut space_index: usize;

    let mut register: i32 = 1;

    for option_line in BufReader::new(file).lines() {
        let line = option_line.expect("Line should be Ok.");
        
        if line == "noop" {
            values.push(register);
            continue;
        }

        space_index = line.find(' ').unwrap();
        values.push(register);
        values.push(register);
        register += line[space_index+1..].parse::<i32>().expect("Parse should be Ok.");
    }

    values
}

fn solve_part_one(tick_values: &Vec<i32>) {

    let mut answer: i32 = 0;
    let mut mul: i32 = 20;
    let mut index: usize = 19;
    while index < tick_values.len() {
        answer += tick_values[index] * mul;
        mul += 40;
        index += 40;
    }
    println!("{}", answer);
}

fn render(screen_buffer: &mut ScreenBuffer, tick_values: &Vec<i32>) {
    for i in 0..tick_values.len() {
        screen_buffer[i/40][i%40] = sprite_covers_crt_position(
            <usize as TryInto<i32>>::try_into(i).expect("Value should be convertible.") % 40,
            tick_values[i]
        );
    }
}

fn sprite_covers_crt_position(crt_position: i32, sprite_center: i32) -> bool {
    sprite_center - 1 <= crt_position && crt_position <= sprite_center + 1
}

fn print_screen(screen_buffer: &ScreenBuffer) {
    for i in 0..6 {
        for j in 0..40 {
            print!(
                "{}",
                match screen_buffer[i][j] {
                    true  => '#',
                    false => '.'
                }
            );
        }
        println!();
    }
}