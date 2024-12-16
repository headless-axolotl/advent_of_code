#![allow(unused)]
use std::{fs::File, io::Read};

const PATH_1: &str = "input_1.txt";
const PATH_2: &str = "input_2.txt";

fn main() {
    let mut file = File::open(PATH_1).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf);

    // solve_part_one(buf);
    solve_part_two(buf);
}

const LETTERS: [char; 4] = ['X', 'M', 'A', 'S'];
const PATTERN_LEN: usize = 4;

fn check(slice: &[char], index: usize, offset: i32) -> bool {
    slice[(index as i32 + offset) as usize]     == LETTERS[1] &&
    slice[(index as i32 + 2 * offset) as usize] == LETTERS[2] &&
    slice[(index as i32 + 3 * offset) as usize] == LETTERS[3]
}

fn solve_part_one(input: String) {
    let height = input.lines().count();
    let matrix = input.chars().filter(|c| LETTERS.contains(c)).collect::<Vec<_>>();
    let width = matrix.len() / height;

    let mut iwidth; let mut iheight;
    let mut left; let mut right; let mut up; let mut down;

    let mut result = 0;
    for i in 0..matrix.len() {

        if matrix[i] != LETTERS[0] {
            continue;
        }

        iwidth = i % width;
        iheight = i / width;

        left  = iwidth >= PATTERN_LEN - 1;
        right = iwidth + PATTERN_LEN <= width;
        up    = iheight >= PATTERN_LEN - 1;
        down  = iheight + PATTERN_LEN <= height;

        if left && check(&matrix, i, -1) {
            result += 1;
        }

        if left && up && check(&matrix, i, -1 - width as i32) {
            result += 1;
        }
 
        if up && check(&matrix, i, - (width as i32)) {
            result += 1;
        }
 
        if up && right && check(&matrix, i, 1 - width as i32) {
            result += 1;
        }
 
        if right && check(&matrix, i, 1) {
            result += 1;
        }
 
        if right && down && check(&matrix, i, 1 + width as i32) {
            result += 1;
        }
        
        if down && check(&matrix, i, width as i32) {
            result += 1;
        }

        if left && down && check(&matrix, i, -1 + width as i32) {
            result += 1;
        }
    }

    println!("Answer: {result}");
}

fn solve_part_two(input: String) {
    let height = input.lines().count();
    let matrix = input.chars().filter(|c| LETTERS.contains(c)).collect::<Vec<_>>();
    let width = matrix.len() / height;

    let mut index; let mut result = 0;
    for row in 1..height-1 {
        for col in 1..width-1 {
            index = col + row * width;

            if matrix[index] != 'A' {
                continue;
            }

            let x_mas = 
                (matrix[index - width - 1] == 'M' && matrix[index + width + 1] == 'S' ||
                matrix[index - width - 1] == 'S' && matrix[index + width + 1] == 'M') &&
                (matrix[index - width + 1] == 'M' && matrix[index + width - 1] == 'S' ||
                matrix[index - width + 1] == 'S' && matrix[index + width - 1] == 'M');

            if x_mas {
                result += 1;
            }
        }
    }

    println!("Answer: {result}");
}

