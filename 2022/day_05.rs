#![allow(unused)]
use std::{fs::File, io::{BufRead, BufReader}};

struct CommandData(usize, usize, usize);

fn main() {
    let path = "input.txt";
    
    let mut file = match File::open(path) {
        Ok(val) => val,
        Err(_) => panic!("couldn't open file!"),
    };
    solve(file);

}

fn solve(file: File) {
    let lines = BufReader::new(file).lines();

    let mut stacks : Vec<Vec<char>> = vec![];

    let mut reading_stack_data = true;
    for option_line in lines {

        if let Ok(line) = option_line {
            if line == "" {
                reading_stack_data = false;
                debug_print(&stacks);
                continue;
            }
            if reading_stack_data {
                process_stack_line(&line, &mut stacks);
            }
            else {
                // execute_command_line_v1(process_command_line(&line), &mut stacks); // part 1
                execute_command_line_v2(process_command_line(&line), &mut stacks); // part 2
            }
        }
    }

    let mut last_index_of_stack : usize = 0;
    for i in 0..stacks.len() {
        last_index_of_stack = stacks[i].len() - 1;
        print!("{}", stacks[i][last_index_of_stack]);
    }

    println!();

}

fn process_stack_line(line: &str, stacks: &mut Vec<Vec<char>>) {
    let mut position : usize = 0;

    for character in line.chars() {
        if character.is_ascii_uppercase() {
            let stack_index = position / 4;
            while stacks.len() <= stack_index {
                stacks.push(vec![]);
            }
            stacks[stack_index].insert(0, character);
        }
        position += 1;
    }
}

fn debug_print(stacks: &Vec<Vec<char>>) {
    for i in 0..stacks.len(){
        for j in 0..stacks[i].len() {
            print!("{} ", stacks[i][j]);
        }
        println!();
    }
    println!();
}

fn process_command_line(line: &str) -> CommandData {
    let space_1 = line.find(' ').unwrap();
    let space_2 = space_1 + 1 + line[space_1+1 ..].find(' ').unwrap();
    let space_3 = space_2 + 1 + line[space_2+1 ..].find(' ').unwrap();
    let space_4 = space_3 + 1 + line[space_3+1 ..].find(' ').unwrap();
    let space_5 = space_4 + 1 + line[space_4+1 ..].find(' ').unwrap();

    let quantity : usize = line[space_1+1..space_2].parse().unwrap();
    let from     : usize = line[space_3+1..space_4].parse().unwrap();
    let to       : usize = line[space_5+1..       ].parse().unwrap();

    CommandData(quantity, from-1, to-1)
}

fn execute_command_line_v1(data: CommandData, stacks: &mut Vec<Vec<char>>) {
    for i in 0..data.0 {
        let to_be_moved = stacks[data.1].pop().unwrap();
        stacks[data.2].push(to_be_moved);
    }
}

fn execute_command_line_v2(data: CommandData, stacks: &mut Vec<Vec<char>>) {
    let mut intermediate : Vec<char> = vec![];
    for i in 0..data.0 {
        let to_be_moved = stacks[data.1].pop().unwrap();
        intermediate.push(to_be_moved);
    }
    for i in 0..data.0 {
        let to_be_moved = intermediate.pop().unwrap();
        stacks[data.2].push(to_be_moved);
    }
}

