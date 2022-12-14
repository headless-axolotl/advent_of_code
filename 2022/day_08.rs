#![allow(unused)]
use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

const PATH: &str = "input.txt";

fn main() {
    solve();
}

fn solve() {
    
    let mut file = File::open(PATH).expect("input.txt wasn't found!");
    solve_part_one(file);

    file = File::open(PATH).expect("input.txt wasn't found!");
    solve_part_two(file);
}

fn solve_part_one(file: File) {

    let mut trees = create_tree_matrix(file);
    let matrix_size = trees.len();
    let mut already_visible: Vec<Vec<bool>> = vec![];

    for i in 0..matrix_size {
        already_visible.push(vec![]);
        for j in 0..matrix_size {
            already_visible[i].push(false);
        }
    }

    let mut answer: i32 = 0;
    let mut current_highest: i8;
    
    // horizontal
    for row in 0..matrix_size {
        // left-right
        current_highest = -1;
        for column in 0..matrix_size {
            answer += process_single_tree(
                row, 
                column, 
                &mut current_highest, 
                &trees, 
                &mut already_visible
            );
        }

        // right-left
        current_highest = -1;
        for column in (0..matrix_size).rev() {
            answer += process_single_tree(
                row, 
                column, 
                &mut current_highest, 
                &trees, 
                &mut already_visible
            );
        }
    }
    // vertical
    for column in 0..matrix_size {
        // top-bottom
        current_highest = -1;
        for row in 0..matrix_size {
            answer += process_single_tree(
                row, 
                column, 
                &mut current_highest, 
                &trees, 
                &mut already_visible
            );
        }

        // bottom-top
        current_highest = -1;
        for row in (0..matrix_size).rev() {
            answer += process_single_tree(
                row, 
                column, 
                &mut current_highest, 
                &trees, 
                &mut already_visible
            );
        }
    }

    println!("{}", answer);
    
}

fn solve_part_two(file: File) {

    let mut trees = create_tree_matrix(file);
    let matrix_size = trees.len();

    let mut scenic_score: Vec<Vec<usize>> = vec![];
    for i in 0..matrix_size {
        scenic_score.push(vec![]);
        for j in 0..matrix_size {
            scenic_score[i].push(1);
        }
    }

    let mut previous_trees: [usize; 16];
    let mut parsed_height: usize;

    // horizontal
    for row in 0..matrix_size {
        // left-right
        previous_trees = [0; 16];
        for column in 1..matrix_size {
            parsed_height = trees[row][column].try_into().unwrap();
            scenic_score[row][column] *= (column - previous_trees[parsed_height]);
            
            update_previous_trees(parsed_height, column, &mut previous_trees);
        }

        // right-left
        previous_trees = [matrix_size-1; 16];
        for column in (0..matrix_size-1).rev() {
            parsed_height = trees[row][column].try_into().unwrap();
            scenic_score[row][column] *= (previous_trees[parsed_height] - column);
            
            update_previous_trees(parsed_height, column, &mut previous_trees);
        }
    }
    
    let mut answer: usize = 0;

    // vertical
    for column in 0..matrix_size {
        // top-bottom
        previous_trees = [0; 16];
        for row in 1..matrix_size {
            parsed_height = trees[row][column].try_into().unwrap();
            scenic_score[row][column] *= (row - previous_trees[parsed_height]);
            
            update_previous_trees(parsed_height, row, &mut previous_trees);
        }

        // bottom-top
        previous_trees = [matrix_size-1; 16];
        for row in (0..matrix_size-1).rev() {
            parsed_height = trees[row][column].try_into().unwrap();
            
            scenic_score[row][column] *= (previous_trees[parsed_height] - row);

            answer = max(answer, scenic_score[row][column]);
            update_previous_trees(parsed_height, row, &mut previous_trees);
        }
    }
    
    println!("{}", answer);

}

fn create_tree_matrix(file: File) -> Vec<Vec<i8>> {

    let mut trees: Vec<Vec<i8>> = vec![];
    let mut row: usize = 0;

    for option_line in BufReader::new(file).lines() {
        
        let line = option_line.expect("Line should be Ok.");
        
        trees.push(vec![]);
        process_line(&line, &mut trees, row);
        row += 1;
    }

    trees
}

fn process_line(line: &str, trees: &mut Vec<Vec<i8>>, row: usize) {
    for i in 0..line.len() {
        trees[row].push(
            line[i..=i]
                .parse()
                .expect("Should be a single digit.")
        );
    }
}

fn process_single_tree(
    row: usize,
    column: usize,
    current_highest: &mut i8,
    trees: &Vec<Vec<i8>>,
    already_visible: &mut Vec<Vec<bool>>
) -> i32 {
    
    if trees[row][column] <= *current_highest {
        return 0;
    }

    *current_highest = trees[row][column];

    if already_visible[row][column] {
        return 0;
    }

    already_visible[row][column] = true;
    return 1;
}

fn update_previous_trees(tree_height: usize, relevant_tree_index: usize, values: &mut [usize; 16]) {
    for i in 0..=tree_height {
        values[i] = relevant_tree_index;
    }
}

fn max(left: usize, right: usize) -> usize {
    match left > right {
        true => left,
        false => right,
    }
}
