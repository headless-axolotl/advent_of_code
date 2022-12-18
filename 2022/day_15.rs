#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    fmt::Debug, collections::HashSet
};

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

type Point  = (i32, i32);
type Range  = (i32, i32);
type Sensor = (Point, i32);

fn main() {
    
    solve();
}

fn solve() {

    let file = File::open(PATH).expect("File should exist");
    // let file = File::open(TEST).expect("File should exist");

    let mut sensors: Vec<Sensor> = vec![];

    let mut max_possible_x: i32 = 0;
    let mut min_possible_x: i32 = 0;

    let mut beacons: HashSet<Point> = HashSet::new();

    for option_line in BufReader::new(file).lines() {
        let line = option_line.expect("Line should be Ok.");
        let sensor = parse_line(&line, &mut beacons);
        max_possible_x =
            i32::max(max_possible_x, sensor.0.0 + sensor.1);
        min_possible_x =
            i32::min(min_possible_x, sensor.0.0 - sensor.1);
        sensors.push(sensor);
    }

    solve_part_one(&sensors, &beacons);
    solve_part_two(&sensors, &beacons);

}

fn solve_part_one(
    sensors: &Vec<Sensor>, 
    beacons: &HashSet<Point>
) {

    const GIVEN_ROW: i32 = 2_000_000;
    let ranges = probe_row(
        GIVEN_ROW,
        &sensors
    );

    let mut answer: i32 = 0;
    for range in ranges {
        answer += range.1 - range.0 + 1;
    }
    for beacon in beacons {
        if beacon.1 == GIVEN_ROW {
            answer -= 1;
        }
    }
    
    println!("{}", answer);
}

// slow -- potential optimization might be implemented in the future
// 3040754 [(-1192342, 2889464), (2889466, 4276084)] -> 2889465, 3040754 -> 11_557_863_040_754
fn solve_part_two(
    sensors: &Vec<Sensor>,
    beacons: &HashSet<Point>
) {
    for y in 0 ..= 4_000_000 {
        let ranges = probe_row(y, sensors);
        if ranges.len() > 1 {
            println!("{} {:?}", y, ranges);
            return;
        }
    }
}

fn manhatan_distance(
    point_a: Point, 
    point_b: Point
) -> i32{
    (point_a.0 - point_b.0).abs()
        + (point_a.1 - point_b.1).abs()
}

fn parse_line(
    line: &str, 
    beacons: &mut HashSet<Point>
) -> Sensor {

    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let first_equal : usize = line.find('=').unwrap();
    let second_equal: usize = first_equal  + 1 + line[first_equal  + 1..].find("=").unwrap();
    let third_equal : usize = second_equal + 1 + line[second_equal + 1..].find("=").unwrap();
    let fourth_equal: usize = third_equal  + 1 + line[third_equal  + 1..].find("=").unwrap();

    let first_comma : usize = line.find(',').unwrap();
    let second_comma: usize = first_comma  + 1 + line[first_comma + 1 ..].find(',').unwrap();

    let colon       : usize = line.find(':').unwrap();
    
    let sensor_x: i32 = line[first_equal  + 1.. first_comma].parse().unwrap();
    let sensor_y: i32 = line[second_equal + 1..       colon].parse().unwrap();
    let beacon_x: i32 = line[third_equal  + 1..second_comma].parse().unwrap();
    let beacon_y: i32 = line[fourth_equal + 1..            ].parse().unwrap();

    
    let sensor = (sensor_x, sensor_y);
    let beacon = (beacon_x, beacon_y);
    beacons.insert(beacon);
    (sensor, manhatan_distance(sensor, beacon))
}

fn probe_row(
    given_row: i32, 
    sensors: &Vec<Sensor>
) -> Vec<Range> {
    
    let mut ranges: Vec<Range> = vec![];
    for sensor in sensors {
        if let Some(range) = generate_range(given_row, sensor) {
            ranges.push(range);
        }
    }

    let mut answer: i32 = 0;
    ranges.sort();
    collapse_ranges(ranges)
}

fn generate_range(
    given_row: i32, 
    sensor: &Sensor
) -> Option<(i32, i32)> {
    let mut leftover_horizontal =
        sensor.1 - (sensor.0.1 - given_row).abs();
    if leftover_horizontal >= 0 {
        return Some((
            sensor.0.0 - leftover_horizontal, 
            sensor.0.0 + leftover_horizontal
        ));
    }
    None
}

fn collapse_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    
    if ranges.len() == 0 {
        return ranges;
    }
    
    while try_collapse(&mut ranges) { }
    ranges
}

fn try_collapse(ranges: &mut Vec<Range>) -> bool {

    let mut processed: Vec<Range> = vec![];
    let mut current = ranges[0];
    for i in 1..ranges.len() {

        if let Some(collapsed) = try_collapse_single(current, ranges[i]) {
            current = collapsed;
        }
        else {
            processed.push(current);
            current = ranges[i];
        }
    }

    processed.push(current);

    if processed.len() == ranges.len() {
        return false;
    }

    *ranges = processed;
    true
}

fn try_collapse_single(
    range_a: Range, 
    range_b: Range
) -> Option<Range> {

    if overlap(range_a, range_b.0) || overlap(range_a, range_b.1)
    || overlap(range_b, range_a.0) || overlap(range_b, range_a.1) {
        return Some((
            i32::min(range_a.0, range_b.0),
            i32::max(range_a.1, range_b.1),
        ));
    }
    None
}

fn overlap(range: Range, point: i32) -> bool {
    range.0 <= point && point <= range.1
}
