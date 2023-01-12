#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Read}, fmt::Display,
};

const PATH: &str = "input.txt";
const PATH2: &str = "input_mod.txt";
const TEST: &str = "test.txt";

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum State { Empty, Blocked, Bounds(char) }

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
enum Orientation { Right, Down, Left, Up }
impl Orientation {
    
    fn to_integer(&self) -> i32 {
        match self {
            Self::Right => 0,
            Self::Down  => 1,
            Self::Left  => 2,
            Self::Up    => 3,
        }
    }

    fn shift(&self, amount: i32) -> Self {
        let converted = (*self as i32 + amount).rem_euclid(4);
        [Self::Right, Self::Down, Self::Left, Self::Up][converted as usize]
    }

    fn rotate(&self, left: bool) -> Self {
        if left {
            self.shift(-1)
        }
        else {
            self.shift(1)
        }
    }

    fn apply_to(&self, position: &Position) -> Position {
        match *self {
            Orientation::Right => (position.0, position.1 + 1),
            Orientation::Down  => (position.0 + 1, position.1),
            Orientation::Left  => (position.0, position.1 - 1),
            Orientation::Up    => (position.0 - 1, position.1),
        }
    }

}

// 150x200
type Grid = [[State; 152]; 202];

#[derive(Debug)]
#[derive(Clone, Copy)]
struct MetaData {
    min : usize,
    max : usize,
}
impl MetaData {
    fn new() -> Self {
        MetaData { min: 201, max: 0 }
    }
}
type Rows    = [MetaData; 152]; // stores min and max eligible rows for that column
type Columns = [MetaData; 202]; // stores min and max eligible columnts for that row

type Position = (usize, usize);
type Instructions = (Vec<i32>, Vec<bool>); // left - true, right - false

fn main() {
    solve();
}

fn solve() {

    let mut file = File::open(PATH2).unwrap();

    let (
        mut grid,
        mut row_data,
        mut column_data,
        mut instructions
    ) = parse_input(file);

    let mut current_position = initial_position(&grid).unwrap();
    let mut current_orientation = Orientation::Right;

    solve_parts(
        &grid,
        &row_data,
        &column_data,
        current_position,
        current_orientation,
        &instructions
    );
}

fn parse_input(mut file: File) -> (Grid, Rows, Columns, Instructions) {

    let mut buf = String::new();
    file.read_to_string(&mut buf);
    let mut lines: Vec<&str> = buf.lines().collect();

    let mut grid: Grid = [[State::Bounds(' '); 152]; 202];
    let mut row_data   : Rows    = [MetaData::new(); 152];
    let mut column_data: Columns = [MetaData::new(); 202];
    
    for i in 0..lines.len()-2 { // row indeces

        let chars: Vec<_> = lines[i].chars().collect();
        println!("{i}");

        for j in 0..chars.len() { // column indeces
            
            let row    = i; // + 1; part one
            let column = j; // + 1; part one

            grid[row][column] = match chars[j] {
                '.' => State::Empty,
                '#' => State::Blocked,
                _ => State::Bounds(chars[j]),
            };

            if grid[row][column] == State::Empty || grid[row][column] == State::Blocked { // part one
                row_data[column].min = usize::min(row_data[column].min, row);
                row_data[column].max = usize::max(row_data[column].max, row);
                column_data[row].min = usize::min(column_data[row].min, column);
                column_data[row].max = usize::max(column_data[row].max, column);
            }
        }
    }

    let mut instructions = generate_instructions(lines[lines.len()-1]);

    (grid, row_data, column_data, instructions)

}

fn generate_instructions(line: &str) -> Instructions {

    let mut steps: Vec<i32> = vec![];
    let mut rotations: Vec<bool> = vec![];

    for number in line.split(['R', 'L'].as_ref()) {
        steps.push(number.parse().unwrap());
    }

    for rotation in line.split(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].as_ref()) {
        match rotation {
            "L" => rotations.push(true),
            "R" => rotations.push(false),
            _   => {}
        };
    }

    (steps, rotations)
}

fn initial_position(grid: &Grid) -> Option<Position> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == State::Empty {
                return Some((i, j));
            }
        }
    }
    None
}

fn solve_parts(
    grid       : &Grid,
    row_data   : &Rows,
    column_data: &Columns,
    mut current_position   : Position,
    mut current_orientation: Orientation,
    instructions: &Instructions,
) {
    
    for i in 0..instructions.1.len() {
        execute(
            grid,
            row_data, column_data,
            &mut current_position,
            &mut current_orientation,
            instructions.0[i]
        );
        println!("{current_position:?}");
        current_orientation = current_orientation
        .rotate(instructions.1[i]);
    }
    execute(
        grid,
        row_data, column_data,
        &mut current_position,
        &mut current_orientation,
        instructions.0[instructions.0.len()-1]
    );
    println!("{current_position:?}");
    let mut answer = (1000 * current_position.0 + 4 * current_position.1) as i32;
    answer += current_orientation.to_integer();
    println!("{}", answer);
}

fn execute(
    grid       : &Grid,
    row_data   : &Rows,
    column_data: &Columns,
    current_position   : &mut Position,
    current_orientation: &mut Orientation,
    mut steps: i32
) {
    for i in 0..steps {
        if !try_move(
            grid,
            row_data, column_data,
            current_position,
            current_orientation,
        ) {
            break;
        }
    }
}

fn try_move(
    grid       : &Grid,
    row_data   : &Rows,
    column_data: &Columns,
    from_position   : &mut Position,
    orientation     : &mut Orientation,
) -> bool {
    
    let mut next_position = orientation.apply_to(from_position);
    let previous_orientation = *orientation;

    if let State::Bounds(option) = grid[next_position.0][next_position.1] {
        next_position = wrap_part_two( // wrap_part_one(
            &next_position,
            orientation,
            row_data, column_data,
            option,
        );
    }

    if grid[next_position.0][next_position.1] == State::Blocked {
        *orientation = previous_orientation;
        return false;
    }

    *from_position = next_position;
    true
}

fn wrap_part_one(
    position   : &Position,
    orientation: &mut Orientation,
    row_data   : &Rows,
    column_data: &Columns,
    option     : char,
) -> Position {
    match orientation {
        Orientation::Right => (position.0, column_data[position.0].min),
        Orientation::Left  => (position.0, column_data[position.0].max),
        Orientation::Down  => (row_data[position.1].min, position.1),
        Orientation::Up    => (row_data[position.1].max, position.1),
    }
}

// hard-coded values
fn wrap_part_two(
    position   : &Position,
    orientation: &mut Orientation,
    row_data   : &Rows,
    column_data: &Columns,
    option     : char,
) -> Position {

    let mut answer: Position = (0, 0);
    println!("{}", option);

    match option {
        'a' => {

            if *orientation == Orientation::Down
            || *orientation == Orientation::Right {
                panic!("unknown wrap: wrong orientation");
            }

            if *orientation == Orientation::Left {
                answer.0 = 1;
                answer.1 = position.0 - 100; // - 151 + 51
                *orientation = Orientation::Down;
            }
            else {
                answer.1 = 1;
                answer.0 = position.1 + 100; // - 51 + 151
                *orientation = Orientation::Right;
            }
        }
        'b' => {

            if *orientation == Orientation::Left
            || *orientation == Orientation::Right {
                panic!("unknown wrap: wrong orientation");
            }

            if *orientation == Orientation::Up {
                answer.0 = 200;
                answer.1 = position.1 - 100;
            }
            else {
                answer.0 = 1;
                answer.1 = position.1 + 100;
            }
        }
        'c' => {

            if *orientation == Orientation::Up
            || *orientation == Orientation::Left {
                panic!("unknown wrap: wrong orientation");
            }

            if *orientation == Orientation::Right {
                answer.0 = 50;
                answer.1 = position.0 + 50; // - 51 + 101
                *orientation = Orientation::Up;
            }
            else {
                answer.1 = 100;
                answer.0 = position.1 - 50; // + 51 - 101
            }
            
        }
        'd' => {
            
            if *orientation == Orientation::Down
            || *orientation == Orientation::Right {
                panic!("unknown wrap: wrong orientation");
            }

            if *orientation == Orientation::Left {
                answer.0 = 101;
                answer.1 = position.0 - 50; // + 51 - 101
                *orientation = Orientation::Down;
            }
            else {
                answer.1 = 51;
                answer.0 = position.1 + 50; // - 51 + 101
                *orientation = Orientation::Right;
            }
        }
        'e' => {

            if *orientation == Orientation::Up
            || *orientation == Orientation::Left {
                panic!("unknown wrap: wrong orientation");
            }

            if *orientation == Orientation::Right {
                answer.0 = 150;
                answer.1 = position.0 - 100; // + 51 - 151
                *orientation = Orientation::Up;
            }
            else {
                answer.1 = 50;
                answer.0 = position.1 + 100; // - 51 + 151
                *orientation = Orientation::Left;
            }

        }
        'f' => {
            
            if *orientation == Orientation::Up
            || *orientation == Orientation::Down 
            || *orientation == Orientation::Right{
                panic!("unknown wrap: wrong orientation");
            }

            *orientation = Orientation::Right;
            if position.1 == 50 {
                answer.1 = 1;
                answer.0 = 151 - position.0;
            }
            else {
                answer.1 = 51;
                answer.0 = 151 - position.0; // + 1 - 101
            }
        }
        'g' => {
            
            if *orientation == Orientation::Up
            || *orientation == Orientation::Down 
            || *orientation == Orientation::Left{
                panic!("unknown wrap: wrong orientation");
            }

            *orientation = Orientation::Left;
            if position.1 == 151 {
                answer.1 = 100;
                answer.0 = 151 - position.0;
            }
            else {
                answer.1 = 150;
                answer.0 = 151 - position.0;
            }
        }
        
        _ => { panic!("unknown wrap") }
    };
    
    answer
}
