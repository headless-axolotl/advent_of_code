#![allow(unused)]
use std::{fs::File, io::{BufRead, BufReader, Read}, collections::HashSet};
use core::ops::{Add, Sub, AddAssign};
use core::cmp::{Eq, PartialEq};

#[derive(Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Debug)]
struct Vec2 { 
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{x: self.x + other.x, y: self.y + other.y}
    }
}

const PATH: &str = "input.txt";

fn main() {
    solve();
}

fn solve() {
    let mut file = File::open(PATH).expect("input.txt not found");
    solve_rope(file, 10);
}

fn solve_rope(file: File, segment_count: i32) {

    let mut visited: HashSet<Vec2> = HashSet::new();
    let mut direction: Vec2;
    let mut repetition: i32;
    let mut space_index: usize;

    if segment_count < 1 { return; }

    let mut rope: Vec<Vec2> = vec![];
    for i in 0..segment_count {
        rope.push(Vec2{x: 0, y: 0});
    }

    for option_line in BufReader::new(file).lines() {
        let line = option_line.expect("Line should be Ok.");

        space_index = line.find(" ").expect("Every line should contain at least one space.");
        direction = to_direction(&line[0..space_index]);
        repetition = line[space_index+1..].parse().expect("Remaining string should be a parseable integer.");

        for i in 0..repetition {
            process_rope(&mut rope, direction);
            visited.insert(*rope.last().expect("There should always be at least one rope segment!"));
        }
    }

    println!("{}", visited.len());

}

fn process_rope(rope: &mut Vec<Vec2>, direction: Vec2) {
    let mut previous_position: Vec2 = rope[0];
    rope[0] = rope[0] + direction;
    for i in 1 .. rope.len() {
        rope[i] = process_delta(rope[i-1], rope[i]);
    }
}

fn process_delta(new_lead_position: Vec2, mut follower: Vec2) -> Vec2 {
    let x_diff = new_lead_position.x - follower.x;
    let y_diff = new_lead_position.y - follower.y;

    if x_diff > 2 || y_diff > 2 {
        panic!("Distance between segments became too large!");
    }

    match (x_diff.abs(), y_diff.abs()) {
        (2, 2) => {
            follower.x += x_diff.signum();
            follower.y += y_diff.signum();
        }
        (2, _) => {
            follower.x += x_diff.signum();
            follower.y += y_diff.signum();
        }
        (_, 2) => {
            follower.x += x_diff.signum();
            follower.y += y_diff.signum();
        }
        (_, _) => {}
    }

    follower
}

fn to_direction(command: &str) -> Vec2 {
    match command {
        "U" => Vec2{x:  0, y:  1},
        "D" => Vec2{x:  0, y: -1},
        "L" => Vec2{x: -1, y:  0},
        "R" => Vec2{x:  1, y:  0},
        _ => Vec2{x: 0, y: 0},
    }
}