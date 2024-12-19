#![allow(unused)]
use std::{fs::File, io::Read};

const PATH_1: &str = "input_1.txt";
const PATH_2: &str = "input_2.txt";

fn main() {
    let mut file = File::open(PATH_2).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf);

    // solve_part_one(buf);
    solve_part_two(buf);
}

const EMPTY: u8 = 0;
const UP: u8 = 1; // RIGHT, DOWN, LEFT = 2, 3, 4
const OBSTACLE: u8 = 5;

fn process_input(input: String) -> (i32, i32, Vec<u8>) {
    let width = input.find('\n').unwrap() as i32;
    let mut start = 0;
    let grid = input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .map(|(i, c)| match c {
            '#' => OBSTACLE,
            '^' => {
                start = i;
                EMPTY
            }
            _ => EMPTY,
        })
        .collect::<Vec<_>>();

    (width, start as i32, grid)
}

fn solve_part_one(input: String) {
    let (width, mut current, mut grid) = process_input(input);

    let dirs = [-width, 1, width, -1]; // UP, RIGHT, DOWN, LEFT
    let mut current_dir = 0;
    let mut dir = -width;

    let mut count = 1;
    let mut next;
    let mut current_x;
    let mut cell;

    grid[current as usize] = UP;

    loop {
        if dir == -width && current < width {
            break;
        }
        if dir == width && current + width >= grid.len() as i32 {
            break;
        }

        current_x = current % width;
        if dir == -1 && current_x == 0 {
            break;
        }
        if dir == 1 && current_x == width - 1 {
            break;
        }

        next = current + dir;
        cell = &mut grid[next as usize];
        if *cell == OBSTACLE {
            current_dir = (current_dir + 1) & 3;
            dir = dirs[current_dir];
            continue;
        }

        current = next;
        if *cell == EMPTY {
            *cell = current_dir as u8 + 1;
            count += 1;
        }
    }

    println!("Answer: {count}");
}

fn loops(width: i32, mut current: i32, grid: &mut[u8]) -> bool {
    
    let mut next; let mut current_x;

    let dirs = [-width, 1, width, -1];
    let mut current_dir = 0;
    let mut dir = -width;

    loop {
        if dir == -width && current < width {
            break;
        }
        if dir == width && current + width >= grid.len() as i32 {
            break;
        }
        current_x = current % width;
        if dir == -1 && current_x == 0 {
            break;
        }
        if dir == 1 && current_x == width - 1 {
            break;
        }

        next = current + dir;
        if grid[next as usize] == OBSTACLE {
            current_dir = (current_dir + 1) & 3;
            dir = dirs[current_dir];
            continue;
        }

        if grid[next as usize] == (current_dir + 1) as u8 {
            return true;
        }
        grid[next as usize] = (current_dir + 1) as u8;
        current = next;
    }
    false
}

fn solve_part_two(input: String) {
    let (width, start, mut grid) = process_input(input);
    let mut cpy = Vec::with_capacity(grid.len());
    let mut result = 0;
    for i in 0..grid.len() {
        if i == start as usize || grid[i] == OBSTACLE {
            continue;
        }

        cpy.clone_from(&grid);
        cpy[i] = OBSTACLE;
        if loops(width, start, &mut cpy) {
            result += 1;
        }
    }
    println!("Answer: {result}");
}

