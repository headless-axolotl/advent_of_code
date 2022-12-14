use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    solve();
}

fn solve() {

    let path = "input.txt";
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    };

    let mut answer : [i32; 3] = [0, 0, 0];
    let mut current : i32 = 0;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(contents) = line {
            if contents.trim() == "" {
                push_value(&mut answer, current);
                current = 0;
            }
            else if let Ok(value) = contents.parse::<i32>() {
                current += value;
            }
        }
    }

    println!("individual max: {}", answer[0]);
    println!("top 3: {}", answer[0] + answer[1] + answer[2]);
}

fn push_value(array: &mut [i32; 3], value: i32) {
    
    if array[0] < value {
        (array[1], array[2]) = (array[0], array[1]);
        array[0] = value;
    }
    else if array[1] < value {
        array[2] = array[1];
        array[1] = value;
    }
    else if array[2] < value {
        array[2] = value;
    }
}
