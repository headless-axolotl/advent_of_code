#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    collections::VecDeque
};

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

const MAXV: usize = 24;
type Matrix = [[[bool; MAXV]; MAXV]; MAXV];
const INIT_MATRIX: Matrix = [[[false; MAXV]; MAXV]; MAXV];
type Position = (i32, i32, i32);
type UPosition = (usize, usize, usize);

fn main() {
    solve();
}

fn solve() {

    let mut file = File::open(PATH).unwrap();
    let mut matrix = parse_input(file);
    solve_part_one(&matrix);
    solve_part_two(&matrix);

}

fn solve_part_one(matrix: &Matrix) {
    
    let mut answer: i32 = 0;
    
    let mut is_exterior: Matrix = [[[true; MAXV]; MAXV]; MAXV];
    for x in 1..(MAXV - 1) as i32 {
        for y in 1..(MAXV - 1) as i32 {
            for z in 1..(MAXV - 1) as i32 {
                answer += free_sides((x, y, z), matrix, &is_exterior);
            }
        }
    }

    println!("{}", answer);
}

fn solve_part_two(matrix: &Matrix) {

    let is_exterior = get_exterior_matrix(matrix);
    let mut answer: i32 = 0;
    
    for x in 1..(MAXV - 1) as i32 {
        for y in 1..(MAXV - 1) as i32 {
            for z in 1..(MAXV - 1) as i32 {
                answer += free_sides((x, y, z), matrix, &is_exterior);
            }
        }
    }

    println!("{}", answer);
}

fn parse_input(file: File) -> Matrix {

    let mut matrix: Matrix = INIT_MATRIX;

    for option_line in BufReader::new(file).lines() {
        
        let line = option_line.expect("Line should be Ok.");

        let comma_1 = line.find(',').unwrap();
        let comma_2 = comma_1 + 1 + line[comma_1+1..].find(',').unwrap();

        let x = line[            .. comma_1].parse::<usize>().unwrap();
        let y = line[comma_1 + 1 .. comma_2].parse::<usize>().unwrap();
        let z = line[comma_2 + 1 ..        ].parse::<usize>().unwrap();

        matrix[x+1][y+1][z+1] = true;
    }

    matrix
}

fn get_exterior_matrix(matrix: &Matrix) -> Matrix {

    let mut is_exterior: Matrix = INIT_MATRIX;

    let mut bfs: VecDeque<Position> = VecDeque::new();
    bfs.push_back((0, 0, 0));

    while !bfs.is_empty() {

        let top = bfs.pop_front().unwrap();

        let utop = (top.0 as usize, top.1 as usize, top.2 as usize);
        is_exterior[utop.0][utop.1][utop.2] = true;

        for neighbour in find_neighbours(top) {
            let uneighbour = (
                neighbour.0 as usize,
                neighbour.1 as usize,
                neighbour.2 as usize
            );

            if !matrix[uneighbour.0][uneighbour.1][uneighbour.2]
            && !is_exterior[uneighbour.0][uneighbour.1][uneighbour.2] {
                
                is_exterior[uneighbour.0][uneighbour.1][uneighbour.2] = true;
                bfs.push_back(neighbour);
            }
        }
    }
    
    is_exterior
}

fn find_neighbours(position: Position) -> Vec<Position> {

    let positions_to_check: Vec<Position> = vec![
        (position.0 + 1, position.1    , position.2    ),
        (position.0 - 1, position.1    , position.2    ),
        (position.0    , position.1 + 1, position.2    ),
        (position.0    , position.1 - 1, position.2    ),
        (position.0    , position.1    , position.2 + 1),
        (position.0    , position.1    , position.2 - 1),
    ];

    let mut neighbours: Vec<Position> = vec![];

    for option in positions_to_check {
        if is_eligible_position(option) {
            neighbours.push((option.0, option.1, option.2,));
        }
    }

    return neighbours;

    fn is_eligible_position(position: Position) -> bool
    {
        is_eligible_value(position.0)
        && is_eligible_value(position.1)
        && is_eligible_value(position.2)
    }
    fn is_eligible_value(value: i32) -> bool
    { 0 <= value && value < MAXV as i32 }
}

fn free_sides(position: Position, matrix: &Matrix, is_exterior: &Matrix) -> i32 {

    if !matrix[position.0 as usize][position.1 as usize][position.2 as usize] {
        return 0;
    }

    let mut answer: i32 = 0;

    for option in find_neighbours(position) {
        
        let option = (option.0 as usize, option.1 as usize, option.2 as usize);
        if !matrix[option.0][option.1][option.2]
        && is_exterior[option.0][option.1][option.2] {
            answer += 1;
        }
    }

    answer
}
