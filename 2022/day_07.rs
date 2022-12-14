#![allow(unused)]
use std::{fs::File, io::{BufRead, BufReader}, collections::HashMap};

#[derive(Debug)]
struct TFile {
    is_dir: bool,
    path: String,
    parent: usize,
    children: Vec<usize>,
    size: i32,
}
impl TFile {
    fn new(is_dir: bool, path: String, parent: usize, size: i32) -> TFile {
        TFile {
            is_dir,
            path,
            parent,
            children: vec![],
            size,
        }
    }
}

struct TFileSystem {
    file_tree: Vec<TFile>,
    file_path_indeces: HashMap<String, usize>,
}

const PATH: &str = "input.txt";
const TOTAL_SYSTEM_SPACE: i32 = 70_000_000;
const NEEDED_FREE_SPACE: i32 = 30_000_000;

fn main() {
    solve();
}

fn solve() {
    let file = File::open(PATH).unwrap();
    
    let mut tfile_system = create_tfsystem(file);
    populate_folder_values(&mut tfile_system);
    solve_part_one(&mut tfile_system);
    solve_part_two(&mut tfile_system);
}

fn solve_part_one(tfile_system: &mut TFileSystem) {

    let mut answer: i32 = 0;
    for tfile in &tfile_system.file_tree {
        answer += match tfile.is_dir && tfile.size < 100_000 {
            true => tfile.size,
            false => 0,
        };
    }

    println!("{}", answer);
}

fn solve_part_two(tfile_system: &mut TFileSystem) {
    
    let free_space = TOTAL_SYSTEM_SPACE - tfile_system.file_tree[0].size;
    let mut answer = TOTAL_SYSTEM_SPACE;

    for tfile in &tfile_system.file_tree {
        let eligible = tfile.is_dir
            && free_space + tfile.size >= NEEDED_FREE_SPACE
            && answer > tfile.size;
        
        if eligible { answer = tfile.size; }
    }

    println!("{}", answer);
}

fn create_tfsystem(file: File) -> TFileSystem {

    let mut file_tree: Vec<TFile> = vec![];
    file_tree.push(TFile::new(true, String::from("~"), 0, 0));
    
    let mut path_to_index: HashMap<String, usize> = HashMap::new();
    path_to_index.insert("~".to_string(), 0);

    let mut current_dir: usize = 0;
    let mut current_path = String::from("~");
    let mut current_index: usize = 0;

    let mut space_index: usize;
    let mut option: &str;
    let mut new_file_path: String;

    let lines = BufReader::new(file).lines();
    let mut line : String;
    for option_line in lines {
        
        line = option_line.expect("Error extracting a line from Option!");

        if line == "$ ls" {
            continue;
        }

        if line == "$ cd /" {
            current_dir = 0;
            current_path = String::from("~");
            continue;
        }

        if line == "$ cd .." {
            current_dir = file_tree[current_dir].parent;
            current_path = String::from(&file_tree[current_dir].path);
            continue;
        }

        space_index = line.find(' ')
            .expect("In this case the line should always contain a space!");
        option = &line[..space_index];

        if option == "$" {
            option = &line[line.rfind(' ').unwrap()+1..];
            current_dir = *path_to_index
                .get(&format!("{}/{}", &current_path, option))
                .expect("Directory does not exist!");
            current_path = String::from(&file_tree[current_dir].path);
            continue;
        }

        new_file_path = format!("{}/{}", &current_path, &line[space_index+1..]);
        
        if path_to_index.contains_key(&new_file_path) {
            continue;
        }

        current_index += 1;
        path_to_index.insert(String::from(&new_file_path), current_index);

        file_tree.push(
            TFile::new(
                option == "dir",
                new_file_path,
                current_dir,
                match option == "dir" {
                    true => 0,
                    false => option.parse().expect("In this case the string should always be parseable!"),
                },
            )
        );

        file_tree[current_dir].children.push(current_index);
    }    

    TFileSystem { file_tree, file_path_indeces: path_to_index }
}

fn populate_folder_values(tfile_system: &mut TFileSystem) {

    let mut are_children_processed: HashMap<usize, bool> = HashMap::new();
    let mut depth_first_search: Vec<usize> = vec![];
    let mut current_index: usize = 0;

    for child in &tfile_system.file_tree[0].children {
        depth_first_search.push(*child);
    }

    while depth_first_search.len() != 0 {

        current_index = *depth_first_search
            .last()
            .expect("Stack should always have at least one entry!");

        if (!tfile_system.file_tree[current_index].is_dir || *are_children_processed.entry(current_index).or_insert(false)) {
            
            let parent_index = tfile_system.file_tree[current_index].parent;
            tfile_system.file_tree[parent_index].size += tfile_system.file_tree[current_index].size;
            
            depth_first_search.pop();
            continue;
        }
        
        *are_children_processed.entry(current_index).or_insert(true) = true;
        for child in &tfile_system.file_tree[current_index].children {
            depth_first_search.push(*child);
        }
    }

}
