#![allow(unused)]
use std::{
    fs::File, 
    io::{BufRead, BufReader}, 
    collections::HashSet, str::FromStr,
    fmt::Debug
};

#[derive(Clone, Copy)]
#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}
#[derive(Clone, Copy)]
#[derive(Debug)]
enum Parameter {
    Given,
    Const (i64),
}

impl Parameter {
    fn evaluate(&self, given: i64) -> i64 {
        match *self {
            Self::Given => given,
            Self::Const(value) => value
        }
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Equation(Parameter, Operation, Parameter);
impl Equation {
    fn evaluate(&self, given: i64, modulo: i64) -> i64 {
        match self.1 {
            Operation::Add =>
                (self.0.evaluate(given) + self.2.evaluate(given)) % modulo,
            Operation::Mul =>
                (self.0.evaluate(given) * self.2.evaluate(given)) % modulo,
        }
    }
}

type Monkey = (Equation, i64, usize, usize);
type Monkeys = (Vec<Monkey>, Vec<Vec<i64>>);

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    solve();
}

fn solve() {    
    let mut file = File::open(PATH).unwrap();
    
    let mut modulo: i64 = 1;
    let mut monkeys = generate_monkeys(file, &mut modulo);
    let mut score: Vec<i64> = vec![0; monkeys.0.len()];

    for i in 0..10000 {
        generate_round(&mut monkeys, &mut score, modulo);
    }

    println!("{:#?}", score);
    score.sort();

    let mut answer: i64 = score[score.len()-2]*score[score.len()-1];
    println!("{}", answer);
}

fn generate_monkeys(file: File, modulo: &mut i64) -> Monkeys {

    const INPUT_GROUP_LENGTH: usize = 7;

    let mut lines: Vec<_> = BufReader::new(file).lines().collect();

    let monkey_count = (lines.len()+1)/INPUT_GROUP_LENGTH;

    let mut monkey_data: Vec<Monkey> = vec![];
    let mut current_monkey: Monkey;
    let mut current_equation: Equation;
    let mut monkey_items: Vec<Vec<i64>> = vec![];

    *modulo = 1;

    for i in 0..monkey_count {
        
        let starting_items_line = lines[i * INPUT_GROUP_LENGTH + 1].as_ref().unwrap();
        monkey_items.push(
            get_starting_values(&starting_items_line)
        );
        
        let equation_line = lines[i * INPUT_GROUP_LENGTH + 2].as_ref().unwrap();
        current_equation = parse_equation(&equation_line);

        let divisible_check_line = lines[i * INPUT_GROUP_LENGTH + 3].as_ref().unwrap();
        let true_monkey_line = lines[i * INPUT_GROUP_LENGTH + 4].as_ref().unwrap();
        let false_monkey_line = lines[i * INPUT_GROUP_LENGTH + 5].as_ref().unwrap();

        let divisibility_check: i64 = get_number_from_end_of_line(divisible_check_line);
        *modulo *= divisibility_check;

        monkey_data.push((
            current_equation,
            divisibility_check,
            get_number_from_end_of_line(true_monkey_line),
            get_number_from_end_of_line(false_monkey_line)
        ));
    }

    (monkey_data, monkey_items)
}

fn get_starting_values(line: &str) -> Vec<i64> {

    let colon = line.find(":").expect("Items line should contain a colon.");
    let mut parsed_values: Vec<i64> = vec![];

    for unparsed in line[colon+2..].split(", ") {
        parsed_values.push(
            unparsed
                .parse()
                .expect("Segment should be parseable")
        );
    }

    parsed_values
}

fn parse_equation(line: &str) -> Equation {

    let equal_sign = line
        .find('=')
        .expect("Operation line should contain an equal sign.");

    let space_1 = equal_sign + 2 + line[equal_sign + 2..]
        .find(' ')
        .expect("Operation should contain a space");
    let space_2 = space_1    + 1 + line[space_1    + 1..]
        .find(' ')
        .expect("Operation should contain a space");

    Equation (
        parse_parameter(&line[equal_sign+2..space_1]), 
        match &line[space_1+1..space_2] {
            "+" => Operation::Add,
            _ => Operation::Mul
        },
        parse_parameter(&line[space_2+1..])
    )
}

fn parse_parameter(value: &str) -> Parameter {
    if value == "old" {
        return Parameter::Given;
    }
    Parameter::Const(value.parse().expect("Parameter value should be parseable."))
}

fn get_number_from_end_of_line<T>(line: &str) -> T
where T : FromStr, <T as FromStr>::Err: Debug {
    let last_space = line.rfind(" ").expect("Line should contain a space.");
    line[last_space + 1..].parse().expect("Value should be parseable.")
}

fn generate_round(
    monkeys: &mut Monkeys, 
    score: &mut Vec<i64>, 
    modulo: i64
) {
    
    for monkey_index in 0..monkeys.0.len() {
        let monkey_score = 
            process_single_monkey(
                &monkeys.0[monkey_index], 
                monkey_index, 
                &mut monkeys.1,
                modulo
            );
        score[monkey_index] += monkey_score;
    }
}

fn process_single_monkey(
    monkey: &Monkey, 
    monkey_index: usize, 
    items: &mut Vec<Vec<i64>>, 
    modulo: i64
) -> i64 {

    let mut temporary: Vec<i64> = vec![];
    for item in &items[monkey_index] {
        // temporary.push(monkey.0.evaluate(*item)/3);
        temporary.push(monkey.0.evaluate(*item, modulo));
    }
    for item in temporary {
        if item % monkey.1 == 0{
            items[ monkey.2 ].push(item);
        }
        else {
            items[ monkey.3 ].push(item);
        }
    }
    let inspected_item_count: i64 = items[monkey_index].len().try_into().unwrap();
    items[monkey_index].clear();
    inspected_item_count
}

