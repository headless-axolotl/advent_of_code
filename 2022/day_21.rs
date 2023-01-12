#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write, Read},
    collections::HashMap,
};

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

#[derive(Debug)]
#[derive(Clone, Copy)]
enum Monkey {
    Value(i64),
    Operation(usize, usize, i8),
}

const HUMN: usize = 54;

impl Monkey {

    fn evaluate(tree: &Tree, calculated: &mut HashMap<usize, i64>, index: usize) -> i64 {

        if let Some(&value) = calculated.get(&index) {
            return value;
        }

        let monkey = tree[index];
        let mut answer: i64 = 0;
        if let Monkey::Value(value) = monkey {
            answer = value;
        }
        if let Monkey::Operation(
            left_index,
            right_index,
            operation
        ) = monkey {
            let left  = Self::evaluate(tree, calculated, left_index);
            let right = Self::evaluate(tree, calculated, right_index);
            answer = Self::execute_operation(left, right, operation);
        }

        calculated.insert(index, answer);
        answer
    }

    fn execute_operation(left: i64, right: i64, operation: i8) -> i64 {
        match operation {
            0 => left + right,
            1 => left - right,
            2 => left * right,
            3 => left / right,
            _ => 0,
        }
    }

    fn to_operation(self) -> (usize, usize, i8) {
        if let Self::Operation(left, right, operation) = self {
            return (left, right, operation);
        }
        panic!("not an operation!");
    }

}

type Tree = Vec<Monkey>;
type MonkeyMap = HashMap<String, usize>;

fn main() {
    // solve_part_one();
    solve_part_two();
}

fn solve_part_one() {
    
    let mut file = File::open(PATH).unwrap();
    
    let (mut tree, mut monkey_map) = process_input(file);
    let mut calculated: HashMap<usize, i64> = HashMap::new();
    let answer = Monkey::evaluate(&tree, &mut calculated, monkey_map["root"]);
    println!("{}", answer);
}

fn solve_part_two() {

    let mut file = File::open(PATH).unwrap();

    let (mut tree, mut monkey_map) = process_input(file);
    let mut calculated: HashMap<usize, i64> = HashMap::new();
    let root_index = monkey_map["root"];
    let (left_check, right_check, _) = tree[root_index].to_operation();
    
    // checked that the Monkey::evaluate(left_check) is
    // monotonously decreasing i.e. we can do a binary search
    
    // Monkey::evalueate(right_check) seems to be constant: 21_973_580_688_943
    let searched_value: i64 = 21_973_580_688_943;
    
    // Monkey::evaluate(left_check)  = -22632809552529,
    // when tree[HUMN] = 9_999_999_999_999 i.e. that is the upper bound

    // left is the highest value where when (tree[HUMN] = left)
    // evaluate(l_c) >= searched_value
    let mut left : i64 = 0;
    let mut right: i64 = 9_999_999_999_999;
    let mut middle: i64 = 0;

    while left < right - 1 {
        middle = (left + right) >> 1;

        calculated.clear();
        tree[HUMN] = Monkey::Value(middle);
        let guess = Monkey::evaluate(&tree, &mut calculated, left_check);

        if guess < searched_value {
            right = middle;
        }
        else {
            left = middle;
        }
    }

    calculated.clear();
    tree[HUMN] = Monkey::Value(left);
    let guess = Monkey::evaluate(&tree, &mut calculated, left_check);

    println!("{} {}", left, guess); // 3_916_491_093_818

    // 3_916_491_093_817 was the answer (it had to be the smallest possible number i guess)

}

fn process_input(mut file: File) -> (Tree, MonkeyMap) {
    
    let mut tree: Tree = vec![];
    let mut monkey_map: MonkeyMap = MonkeyMap::new();

    let mut buf = String::new();
    file.read_to_string(&mut buf);
    let mut lines: Vec<&str> = buf.lines().collect();

    generate_keys(&lines, &mut monkey_map);

    for i in 0..lines.len() {
        
        let split: Vec<_> = lines[i].split(' ').collect();
        if split.len() == 2 {
            tree.push(Monkey::Value(split[1].parse().unwrap()));
            continue;
        }
        
        let left = monkey_map[split[1]];
        let right = monkey_map[split[3]];
        let operation = match split[2] {
            "+" => 0,
            "-" => 1,
            "*" => 2,
            "/" => 3,
            _   => 0,
        };

        tree.push(Monkey::Operation(left, right, operation));
    }

    (tree, monkey_map)
}

fn generate_keys(lines: &Vec<&str>, monkey_map: &mut MonkeyMap) {
    for i in 0..lines.len() {
        monkey_map.insert(String::from(&lines[i][0..4]), i);
    }
}
