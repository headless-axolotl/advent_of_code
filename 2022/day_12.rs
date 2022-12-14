#![allow(unused)]
use std::{
    fs::File, 
    io::{BufRead, BufReader},
    collections::HashSet,
    collections::BinaryHeap,
};

const TEST: &str = "test.txt";
const PATH: &str = "input.txt";

type Position = (usize, usize);
type HeightMap = Vec<Vec<i8>>;
type NodeInfo = Vec<Vec<(i32, Position)>>;

fn main() {
    solve();
}

fn solve() {
    let mut file = File::open(PATH).expect("File should exist.");

    let (mut height_map, start, end) = generate_height_map(file);
    // let mut node_info = generate_node_info(&height_map, start);
    // dijkstra(&height_map, &mut node_info, start, end);
    // println!("{}", node_info[end.0][end.1].0); // part one
    
    let mut new_node_info = generate_node_info(&height_map, end);
    let answer = dijkstra_part_two(&height_map, &mut new_node_info, end);
    println!("{}", answer);
}

fn generate_height_map(file: File) -> (HeightMap, Position, Position) {

    let mut height_map: HeightMap = vec![];
    let mut row_index: usize = 1;
    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);

    let mut option_lines: Vec<_> = BufReader::new(file).lines().collect();
    let line_length = option_lines[0].as_ref().expect("Line should be Ok.").len();
    
    height_map.push(vec![127; line_length+2]);
    for option_line in option_lines {
        
        let line = option_line.expect("Line should be Ok.");
        
        height_map.push(vec![127]);
        for character in line.chars() {
            
            let value = generate_height_from_char(character);
            match value {
                0 => start = (row_index, height_map[row_index].len()),
                27 => end = (row_index, height_map[row_index].len()),
                _ => {}
            }

            height_map[row_index].push(value);
        }
        height_map[row_index].push(127);
        row_index += 1;
    }
    height_map.push(vec![127; line_length+2]);

    (height_map, start, end)
}

fn generate_node_info(height_map: &HeightMap, start: Position) -> NodeInfo {
    let rows = height_map.len();
    let mut node_info: NodeInfo = vec![
        vec![(i32::MAX, (0, 0)); height_map[0].len()]; 
        rows
    ];
    node_info[start.0][start.1] = (0, start);
    node_info
}

fn eligible_neigbours(height_map: &HeightMap, given: Position) -> Vec<Position> {
    
    let mut neighbours: Vec<Position> = vec![];
    
    let given_height = height_map[given.0][given.1];
    if given_height == 127 {
        return neighbours;
    }

    let tested_indeces = [
        (given.0 + 1, given.1), 
        (given.0 - 1, given.1), 
        (given.0, given.1 + 1), 
        (given.0, given.1 - 1)
    ];

    for tested_index in tested_indeces {
        push_if_eligible(
            height_map, 
            given_height, 
            tested_index, 
            &mut neighbours
        );
    }

    neighbours
}

fn push_if_eligible(
    height_map: &HeightMap,
    from_height: i8,
    to: Position,
    neighbours: &mut Vec<Position>
) {
    // if from_height + 1 < height_map[to.0][to.1] {
    //     return;
    // }
    if 27 - (from_height) + 1 < 27 - height_map[to.0][to.1] {
        return;
    }
    neighbours.push(to);
}

fn generate_height_from_char(character: char) -> i8 {
    if character.is_ascii_lowercase() {
        return (character as i8) - ('a' as i8) + 1;
    }
    match character {
        'S' => 0,
        'E' => 27,
        _ => 127
    }
}

fn dijkstra(
    height_map: &HeightMap,
    node_info: &mut NodeInfo,
    start: Position,
    end: Position
) {
    let mut heap: BinaryHeap<(i32, Position)> = BinaryHeap::new();

    heap.push((0, start));
    
    while !heap.is_empty() {
        let top = heap.pop().expect("Pop should be Ok.");
        
        if top.1 == end {
            break;
        }
        
        for neighbour in eligible_neigbours(height_map, top.1) {
            let new_distance = node_info[top.1.0][top.1.1].0 + 1;
            if new_distance < node_info[neighbour.0][neighbour.1].0 {
                node_info[neighbour.0][neighbour.1].0 = new_distance;
                node_info[neighbour.0][neighbour.1].1 = top.1;
                heap.push((-new_distance, neighbour));
            }
        }
    }
}

fn dijkstra_part_two(
    height_map: &HeightMap,
    node_info: &mut NodeInfo,
    start: Position,
) -> i32 {
    let mut heap: BinaryHeap<(i32, Position)> = BinaryHeap::new();

    heap.push((0, start));
    
    while !heap.is_empty() {
        let top = heap.pop().expect("Pop should be Ok.");
        
        if height_map[top.1.0][top.1.1] == 1 {
            return node_info[top.1.0][top.1.1].0;
        }
        
        for neighbour in eligible_neigbours(height_map, top.1) {
            let new_distance = node_info[top.1.0][top.1.1].0 + 1;
            if new_distance < node_info[neighbour.0][neighbour.1].0 {
                node_info[neighbour.0][neighbour.1].0 = new_distance;
                node_info[neighbour.0][neighbour.1].1 = top.1;
                heap.push((-new_distance, neighbour));
            }
        }
    }

    -1
}
