#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    collections::{HashMap, VecDeque},
};

mod shapes;
use shapes::{
    HORIZONTAL, PLUS, LSHAPE, VERTICAL, CUBE,
    State, Shape,
    get_rock_shape, get_rock_height
};

type Row = [State; 10];
type Tower = VecDeque<Row>;
type Jets = Vec<bool>; // left - true; right - false
type Position = (usize, i64);

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    solve();    
}

fn solve() {

    let file = File::open(PATH).expect("File should exist.");

    let mut jets = process_input(file);
    let mut tower: Tower = VecDeque::new();
    let mut highest_rock: i64 = 0;
    let mut lower_bound: i64 = 0;
    let mut jet_index: usize = 0;

    solve_part_one(
        &mut tower,
        &mut highest_rock,
        &mut lower_bound,
        &jets, &mut jet_index,
        2022
    );
    
    solve_part_two(
        &mut tower,
        &mut highest_rock,
        &mut lower_bound,
        &jets, &mut jet_index
    );

}

fn process_input(file: File) -> Jets {
    
    let lines: Vec<_> = BufReader::new(file).lines().collect();
    let line = lines[0].as_ref().expect("Line should be Ok");

    let mut jets: Jets = vec![];

    for character in line.chars() {
        if character == '<' {
            jets.push(true);
        }
        else {
            jets.push(false);
        }
    }

    jets
}

fn solve_part_one(
    tower: &mut Tower,
    highest_rock: &mut i64,
    lower_bound: &mut i64,
    jets: &Jets,
    current_jet: &mut usize,
    iteration_count: usize
) {

    simulate_rock(
        tower,
        highest_rock,
        lower_bound,
        0, &jets,
        current_jet,
        true
    );
    for i in 1..iteration_count as i64 {
        simulate_rock(
            tower,
            highest_rock,
            lower_bound,
            (i % 5) as usize, &jets,
            current_jet,
            false
        );
    }
    println!("{}", *highest_rock+1);

    tower.clear();
    *highest_rock = 0;
    *lower_bound = 0;
    *current_jet = 0;

}

fn solve_part_two(
    tower: &mut Tower,
    highest_rock: &mut i64,
    lower_bound: &mut i64,
    jets: &Jets,
    current_jet: &mut usize
) {
    let (mut tower_heights, loop_start) = find_loop_data(
        tower,
        highest_rock,
        lower_bound,
        &jets, current_jet
    );

    let iterations_count: i64 = 1_000_000_000_000;

    let mut answer: i64 = 0;
    let pre_loop_height        = tower_heights[(loop_start - 1) as usize];
    let loop_additional_height = tower_heights[tower_heights.len() - 1] - pre_loop_height;
    let loop_iteration_count   = (tower_heights.len() as i64) - loop_start;
    
    let loop_count      = (iterations_count - loop_start) / loop_iteration_count;
    let incomplete_loop = (iterations_count - loop_start) % loop_iteration_count;

    answer += loop_count * loop_additional_height;
    answer += tower_heights[(loop_start + incomplete_loop - 1) as usize];

    println!("{}", answer);
}

fn find_loop_data(
    tower: &mut Tower,
    highest_rock: &mut i64,
    lower_bound: &mut i64,
    jets: &Jets,
    current_jet: &mut usize
) -> (Vec<i64>, i64) {

    simulate_rock(
        tower,
        highest_rock,
        lower_bound,
        0, jets,
        current_jet,
        true
    );
    
    let mut rock_index: i64 = 1;

    type Combination = (i64, i64);
    let mut encountered_combinations: HashMap<Combination, i64> = HashMap::new();
    let mut encountered_threshold = 5;

    let current_combination: Combination = (0, 0);
    encountered_combinations.insert(current_combination, 0);

    let mut tower_heights: Vec<i64> = vec![];
    tower_heights.push(*highest_rock+1);
    let mut first_encoutnered: i64 = -1;

    loop {
        if rock_index >= 10000 { // more stable cycle and tower (help from reddit)
            let current_combination: Combination = (*current_jet as i64, rock_index % 5);
            if encountered_combinations.contains_key(&current_combination) && rock_index % 5 == 0 {
                first_encoutnered = *encountered_combinations
                    .entry(current_combination).or_insert(-1);
                break;
            }
            encountered_combinations.insert(current_combination, rock_index);
        }

        simulate_rock(
            tower,
            highest_rock,
            lower_bound,
            (rock_index % 5) as usize, jets,
            current_jet,
            false
        );

        tower_heights.push(*highest_rock+1);

        rock_index += 1;
    }
    
    (tower_heights, first_encoutnered)

}

fn increase_capacity(
    tower: &mut Tower,
    highest_rock: i64,
    lower_bound: i64
) {
    
    // 4 for the distance from the highest rock
    // and 4 for the height of the shape grid
    let new_height = (highest_rock + 8 - lower_bound) as usize;

    while tower.len() <= new_height {
        let mut row = [State::Air; 10];
        for i in 7..10 {
            row[i] = State::Rock;
        }
        tower.push_back(row);
    }
}

fn remove_unused(
    tower: &mut Tower,
    lower_bound: &mut i64
) {

    const THRESHOLD: usize = 1024;

    while tower.len() > THRESHOLD {
        *lower_bound += 1;
        tower.pop_front();
    }
}

fn simulate_rock(
    tower: &mut Tower,
    highest_rock: &mut i64,
    lower_bound: &mut i64,
    rock_index: usize,
    jets: &Jets,
    current_jet: &mut usize,
    first_rock: bool
) {
    let mut current_position: Position = (2, *highest_rock + 4);
    if first_rock {
        current_position.1 -= 1;
    }
    let mut shape = get_rock_shape(rock_index);

    increase_capacity(tower, *highest_rock, *lower_bound);
    remove_unused(tower, lower_bound);

    loop {
        if *current_jet == jets.len() {
            *current_jet = 0;
        }

        let mut was_succesful = try_move(
            tower,
            &shape,
            current_position,
            *lower_bound,
            false,
            jets[*current_jet]
        );

        if was_succesful {
            if jets[*current_jet] {
                current_position.0 -= 1;
            }
            else {
                current_position.0 += 1;
            }
        }

        was_succesful = try_move(
            tower,
            &shape,
            current_position,
            *lower_bound,
            true,
            false
        );

        if !was_succesful {
            emprint(tower, &shape, current_position, *lower_bound);
            *highest_rock = i64::max(
                *highest_rock, 
                current_position.1 - 1 + (get_rock_height(rock_index) as i64)
            );
            *current_jet += 1;
            break;
        }

        current_position.1 -= 1;
        *current_jet += 1;
    }

    // println!();

}

fn emprint(
    tower: &mut Tower,
    shape: &Shape,
    position: Position,
    lower_bound: i64
) {

    for y in 0..4 {
        for x in 0..4 {
            let tower_row = (y + position.1 - lower_bound) as usize;
            let mut position = &mut tower[tower_row][x + position.0];
            if *position == State::Air && shape[y as usize][x] == State::Rock {
                *position = State::Rock;
            }
        }
    }
}

fn try_move(
    tower: &Tower,
    shape: &Shape,
    from: Position,
    lower_bound: i64,
    down: bool,
    left: bool
) -> bool {

    if down {
        if from.1 == 0 {
            return false;
        }
        // print!("down ");
        return !are_overlaping(
            tower,
            shape,
            (from.0, from.1 - 1),
            lower_bound
        );
    }
    if !left {
        // print!("right ");
        return !are_overlaping(
            tower,
            shape,
            (from.0 + 1, from.1),
            lower_bound
        );
    }
    if from.0 == 0 {
        return false;
    }
    // print!("left ");
    return !are_overlaping(
        tower,
        shape,
        (from.0 - 1, from.1),
        lower_bound
    );
}

fn are_overlaping(
    tower: &Tower,
    shape: &Shape,
    position: Position,
    lower_bound: i64
) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            let tower_row = (y + position.1 - lower_bound) as usize;
            if tower[tower_row][x + position.0] == State::Rock
            && shape[y as usize][x] == State::Rock {
                return true;
            }
        }
    }
    false
}
