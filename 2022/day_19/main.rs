#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

mod solver;
use solver::{
    Solver, Blueprint
};

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    // solve_part_one();
    solve_part_two();
}

fn solve_part_one() {

    let mut file = File::open("input.txt").unwrap();
    let mut solver = Solver::new();
    let mut row = 1;
    let mut answer = 0;

    for option_line in BufReader::new(file).lines() {
        let line = option_line.unwrap();

        println!("{}", row);
        let blueprint = parse_line(&line);
        let max_geodes = solver.solve(&blueprint, 24);
        answer += max_geodes * row;

        row += 1;
    }

    println!("{}", answer);
}

fn solve_part_two() {
    let mut file = File::open("input.txt").unwrap();
    let mut solver = Solver::new();
    let mut answer = 1;

    let mut row = 0;
    let mut answer = 1;

    for option_line in BufReader::new(file).lines() {
        let line = option_line.unwrap();

        println!("{}", row);
        let blueprint = parse_line(&line);
        
        answer *= solver.solve(&blueprint, 32);

        row += 1;
        if row == 3 {
            break;
        }
    }
    println!("{}", answer);
}

fn parse_line(line: &str) -> Blueprint {

    // Blueprint 1: Each ore robot costs [6]4 ore. 
    // Each clay robot costs [12]4 ore. Each obsidian robot costs
    // [18]4 ore and [21]9 clay. Each geode robot costs [27]3 ore and [30]9 obsidian.

    let sections: Vec<_> = line.split(' ').collect();
    let ore_bot = sections[6].parse().unwrap();
    let clay_bot = sections[12].parse().unwrap();
    let obsidian_bot = (
        sections[18].parse().unwrap(),
        sections[21].parse().unwrap(),
    );
    let geode_bot = (
        sections[27].parse().unwrap(),
        sections[30].parse().unwrap(),
    );

    let mut blueprint = Blueprint {
        ore_bot,
        clay_bot,
        obsidian_bot,
        geode_bot,
    };

    blueprint
}
