#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

// Used some help from the advent of code subreddit

mod recursion;

use recursion::Solver;

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

const MAX_ROOMS: usize = 26 * 26 + 1;
const MAXN: i32 = 26 * 26 + 1;
type Label = usize;
type Graph = Vec<Vec<Label>>;
type Indeces = [usize; MAX_ROOMS];
type AdjacencyMatrix = Vec<Vec<i32>>;

fn main() {
    solve();
}

fn solve() {

    let file = File::open(PATH).expect("File should exist.");
    
    let mut node_count: usize = 0;
    let mut graph: Graph = vec![];
    let mut indeces: Indeces = [MAX_ROOMS-1; MAX_ROOMS];
    let mut matrix: AdjacencyMatrix = vec![];
    let mut pressures: Vec<i32> = vec![];

    for option_line in BufReader::new(file).lines() {
        
        let line = option_line.expect("Line should be Ok.");
        
        graph.push(vec![]);
        parse_line(
            &line,
            &mut graph,
            &mut indeces,
            &mut pressures,
            node_count
        );
        
        node_count += 1;
    }

    rehash_graph(&mut graph, &indeces);
    populate_adjacency_matrix(
        &mut matrix,
        &graph,
        node_count
    );
    floyd_warshall(&mut matrix, node_count);

    solve_parts(indeces[0], matrix, pressures);
    
}

fn parse_line(
    line: &str,
    graph: &mut Graph,
    indeces: &mut Indeces,
    pressures: &mut Vec<i32>,
    current_node: usize
) {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    let equal_sign = line.find('=').unwrap();
    let semi_colon = line.find(';').unwrap();
    let pressure: i32 = line[equal_sign+1..semi_colon].parse().unwrap();
    pressures.push(pressure);

    let room_label = string_to_usize_label(&line[6..8]);
    indeces[room_label] = current_node;

    let mut current_space = line.rfind(' ').unwrap();
    
    // the 'v' from the word "valve(s)"
    while &line[current_space+1..=current_space+1] != "v" {
        
        let neighbour_label = string_to_usize_label(
            &line[current_space + 1..=current_space + 2]
        );

        graph[current_node].push(neighbour_label);
        
        current_space = line[..current_space].rfind(' ').unwrap();
    }
}

fn string_to_usize_label(label: &str) -> usize {
    let chars: Vec<_> = label.chars().collect();
    let a_code = 'A' as usize;
    return (
        26 * ((chars[0] as usize) - a_code) 
        + ((chars[1] as usize) - a_code)
    );
}

fn rehash_graph(
    graph: &mut Graph, 
    indeces: &Indeces
) {
    for i in 0..graph.len() {
        for item in &mut graph[i] {
            *item = indeces[*item];
        }
    }
}

fn populate_adjacency_matrix(
    matrix: &mut AdjacencyMatrix,
    graph: &Graph,
    node_count: usize
) {

    for i in 0..node_count{
        matrix.push(vec![MAXN; node_count]);
    }

    for i in 0..node_count {
        for node in &graph[i] {
            matrix[i][*node] = 1;
        }
    }
}

fn floyd_warshall(matrix: &mut AdjacencyMatrix, node_count: usize) {
    for k in 0..node_count {
        matrix[k][k] = 0;
        for i in 0..node_count {
            for j in 0..node_count {
                matrix[i][j] = i32::min(
                    matrix[i][j], 
                    matrix[i][k] + matrix[k][j]
                );
            }
        }
    }
}

fn solve_parts(start: usize, matrix: AdjacencyMatrix, pressures: Vec<i32>) {

    let mut working_valves: Vec<usize> = vec![];
    for i in 0..pressures.len() {
        if pressures[i] != 0 {
            working_valves.push(i);
        }
    }

    let mut solver = Solver::new(start, matrix, pressures);
    let mut answer: i32 = 0;
    (answer, solver) = Solver::solve(solver, &working_valves);
    println!("{answer}"); // part 1

    let mut progress_counter = 0;
    let mut progress = 0;

    // only 15 pipes are useful (check with input)
    for subset_mask in 1..(1<<15) {

        let unit_1_subset = generate_subset(&working_valves, subset_mask);
        let unit_2_subset = generate_subset(&working_valves, !subset_mask);

        let mut answer_1: i32 = 0;
        let mut answer_2: i32 = 0;

        (answer_1, solver) = Solver::solve(solver, &unit_1_subset);
        (answer_2, solver) = Solver::solve(solver, &unit_2_subset);

        answer = i32::max(answer, answer_1 + answer_2);

        progress_counter+=1;
        if progress_counter >= 100 {
            progress_counter = 0;
            progress += 1;
            println!("progress: {progress}");
        }
    }

    println!("{}", answer);
}

fn generate_subset(working_valves: &Vec<usize>, bitmask: i32) -> Vec<usize> {

    let mut subset: Vec<usize> = vec![];
    for i in 0..15 {
        if (1<<i) & bitmask != 0 {
            subset.push(working_valves[i]);
        }
    }
    subset
}
