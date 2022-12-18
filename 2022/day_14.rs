#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    fmt::Debug
};

const PATH: &str = "input.txt";
const GRID_WIDTH  : usize = 1024;
const GRID_HEIGHT : usize = 180;
const LOWEST_POINT: usize = 159;

type Point = (usize, usize);
type Grid = [[CaveState; GRID_WIDTH]; GRID_HEIGHT];

#[derive(Clone, Copy)]
#[derive(PartialEq)]
enum CaveState {
    Air,
    Rock,
    Snake,
}
type CS = CaveState;
impl Debug for CaveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "."),
            Self::Rock => write!(f, "o"),
            Self::Snake => write!(f, "#"),
        }
    }
}

fn main() {
    // analyse_input();
    solve();
}

fn solve() {

    let file = File::open(PATH).expect("File should exist.");

    let mut grid: Grid = [[CS::Air; GRID_WIDTH]; GRID_HEIGHT];

    for option_line in BufReader::new(file).lines() {

        let line = option_line.expect("Line should be Ok.");

        let rock_snake = parse_line(&line);
        grid = trace_rock_snake(grid, &rock_snake);
    }

    let floor: Vec<Point> = vec![(0, LOWEST_POINT+2), (GRID_WIDTH-1, LOWEST_POINT+2)];
    grid = trace_rock_snake(grid, &floor);

    let mut answer: i32 = 0;
    /*
    while simulate_rock(&mut grid) { // part 1
        answer += 1;
    }
    */
    while grid[0][500] == CS::Air { // part 2
        simulate_rock(&mut grid);
        answer += 1;
    }

    let mut output = match File::options().write(true).truncate(true).open("output.txt") {
        Ok(val) => val,
        Err(_) => File::create("output.txt").unwrap(),
    };
    print_grid(&grid, &mut output);
    println!("{}", answer);
}

fn analyse_input() {

    let file = File::open(PATH).expect("File should exist.");
    let mut lowest: usize = 0;
    let mut left_most: usize = 500;
    let mut right_most: usize = 500;

    for option_line in BufReader::new(file).lines() {
        let line = option_line.expect("Line should be Ok.");

        analyse_rock_snake(
            &parse_line(&line),
            &mut lowest,
            &mut left_most,
            &mut right_most
        );
    }

    println!(
        "lowest: {}\nleft_most: {}\nright_most: {}", 
        lowest, left_most, right_most
    );
}

fn analyse_rock_snake(
    rock_snake: &Vec<Point>, 
    lowest: &mut usize, 
    left_most: &mut usize, 
    right_most: &mut usize
) {
    for point in rock_snake {
        *lowest = usize::max(*lowest, point.1);
        *left_most = usize::min(*left_most, point.0);
        *right_most = usize::max(*right_most, point.0);
    }
}

fn parse_line(line: &str) -> Vec<Point> {

    let mut current_comma: usize = 0;
    let mut current_space: usize = 0;
    let mut current_start: usize = 0;
    let mut rock_snake: Vec<Point> = vec![];

    loop {
        current_comma = match line[current_start..].find(',') {
            Some(val) => current_start + val,
            None => break,
        };
        current_space = match line[current_comma..].find(' ') {
            Some(val) => current_comma + val,
            None => line.len(),
        };

        let x: usize = line[current_start    ..current_comma].parse().expect("Value should be parseable.");
        let y: usize = line[current_comma + 1..current_space].parse().expect("Value should be parseable.");

        rock_snake.push((x, y));

        current_start = match line[current_start + 1..].find('>') {
            Some(val) => current_start + 1 + val + 2,
            None => break,
        };
    }

    rock_snake
}

fn trace_rock_snake(mut grid: Grid, rock_snake: &Vec<Point>) -> Grid {

    let mut previous: Point = rock_snake[0];

    for i in 1..rock_snake.len() {
        
        trace_line(&mut grid, previous, rock_snake[i]);
        previous = rock_snake[i];
    }

    grid
}

fn trace_line(grid: &mut Grid, from: Point, to: Point) {

    if from.0 == to.0 {
        
        let min = usize::min(from.1, to.1);
        let max = usize::max(from.1, to.1);
        for i in min..=max {
            grid[i][from.0] = CS::Snake;
        }
        return;
    }
    let min = usize::min(from.0, to.0);
    let max = usize::max(from.0, to.0);
    for i in min..=max {
        grid[from.1][i] = CS::Snake;
    }
}

fn simulate_rock(grid: &mut Grid) -> bool {

    let mut current_horizontal: usize = 500;
    let mut current_vertical: usize = 0;

    loop {
        
        let result = try_falling(
            current_horizontal, 
            current_vertical, 
            &grid
        );
        
        if let Some(val) = result {
            
            current_vertical += 1;
            current_horizontal = current_horizontal + val - 1;
            
            if current_vertical == GRID_HEIGHT - 1 {
                return false;
            }
            continue;
        }

        grid[current_vertical][current_horizontal] = CS::Rock;
        return true;
    }

    true
}

fn try_falling(
    from_horizontal: usize, 
    from_vertical: usize, 
    grid: &Grid
) -> Option<usize> {

    if grid[from_vertical + 1][from_horizontal] == CS::Air {
        return Some(1);
    }
    if grid[from_vertical + 1][from_horizontal - 1] == CS::Air {
        return Some(0);
    }
    if grid[from_vertical + 1][from_horizontal + 1] == CS::Air {
        return Some(2);
    }
    None
}

fn print_grid(grid: &Grid, output: &mut File) {
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            write!(output, "{:?}", grid[i][j]);
        }
        writeln!(output);
    }
}
