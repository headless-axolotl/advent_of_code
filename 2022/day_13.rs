#![allow(unused)]
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    cmp::Ordering, fmt::{Display, Debug},
};

// #[derive(Debug)]
#[derive(PartialEq)]
enum PacketBit {
    Raw(i32),
    Bracket(bool),
}

type Packet = Vec<PacketBit>;

const PATH: &str = "input.txt";
const TEST: &str = "test.txt";

fn main() {
    solve();
}

fn solve() {

    let mut file = File::open(PATH).expect("File is missing.");
    let mut output = match File::options().write(true).truncate(true).open("output.txt") {
        Ok(val) => val,
        Err(_) => File::create("output.txt").expect("Creation should be successful.")
    };
    // solve_part_one(file, output);
    solve_part_two(file);
}

fn solve_part_one(file: File, mut output_file: File) {

    let option_lines: Vec<_> = BufReader::new(file).lines().collect();
    
    let mut index: usize = 0;
    let mut left: Packet;
    let mut right: Packet;
    let mut answer: i32 = 0;

    while (index * 3) < option_lines.len() {

        left  = parse_packet(&option_lines[index * 3    ].as_ref().expect("Line should be Ok."));
        right = parse_packet(&option_lines[index * 3 + 1].as_ref().expect("Line should be Ok."));
        
        let comparison = compare_packets(&left, &right);
        writeln!(&mut output_file, "{}", match comparison {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1
        });
        writeln!(&mut output_file);
        writeln!(&mut output_file);
        if comparison == Ordering::Less {
            answer += (index + 1) as i32;
        }

        index += 1;
    }

    println!("{}", answer);
}

fn solve_part_two(file: File) {

    let mut packets: Vec<Packet> = vec![];

    let option_lines: Vec<_> = BufReader::new(file).lines().collect();

    let mut index: usize = 0;
    let mut left: Packet;
    let mut right: Packet;
    let mut answer: i32 = 0;

    while (index * 3) < option_lines.len() {

        left  = parse_packet(&option_lines[index * 3    ].as_ref().expect("Line should be Ok."));
        right = parse_packet(&option_lines[index * 3 + 1].as_ref().expect("Line should be Ok."));
        
        packets.push(left);
        packets.push(right);

        index += 1;
    }
    
    packets.push(parse_packet("[[2]]"));
    packets.push(parse_packet("[[6]]"));
    packets.sort_by(|a, b| compare_packets(a, b));

    let mut first: Packet = parse_packet("[[2]]");
    let mut second: Packet = parse_packet("[[6]]");

    let mut first_index: usize = 1;
    let mut second_index: usize = 1;

    for i in 0..packets.len() {
        if compare_packets(&first, &packets[i]) == Ordering::Equal {
            first_index = i + 1;
        }
        if compare_packets(&second, &packets[i]) == Ordering::Equal {
            second_index = i + 1;
        }
    }

    println!("{}", first_index * second_index);

}

fn parse_packet(line: &str) -> Packet {

    let mut packet: Packet = vec![];
    let mut sections: Vec<_> = line.split(',').collect();
    let mut index: usize = 0;
    let mut end_of_integer: usize = 0;

    for section in sections {

        index = 0;
        while index < section.len() {

            let section_at_index = &section[index..=index];
            if section_at_index != "[" && section_at_index != "]" {

                end_of_integer = get_end_of_integer(&section[index..], index);

                let parsed: i32 = section[index..end_of_integer].parse()
                    .expect("Subsection should be parseable");
                packet.push(PacketBit::Raw(parsed));
                
                index = end_of_integer;
                continue;
            }
            
            packet.push(PacketBit::Bracket(section_at_index == "["));
            index += 1;
        }
    }

    packet
}

fn get_end_of_integer(section: &str, integer_start: usize) -> usize {
    
    let opening_bracket = section.find('[')
        .or(Some(section.len())).expect("Given value is always Some.");
    let closing_bracket = section.find(']')
        .or(Some(section.len())).expect("Given value is always Some.");
    
    integer_start + usize::min(opening_bracket, closing_bracket)
}

enum PacketBitOption {
    Some(i32, i32, bool),
    None(i32)
}
type PBO = PacketBitOption;

fn get_next_packet_bit_option(
    packet: &Packet,
    index: &mut usize,
    depth: &mut i32
) -> PacketBitOption {
    
    while *index < packet.len() {

        if let PacketBit::Raw(value) = packet[*index] {
            
            *index += 1;
            return PBO::Some(
                value,
                *depth,
                packet[*index] == PacketBit::Bracket(false)
            );
        }
        if let PacketBit::Bracket(opening) = packet[*index] {
            
            *index += 1;
            if !opening {
                let previous_depth = *depth;
                *depth -= 1;

                return PBO::None(previous_depth);
            }
            
            *depth += 1;
        }
    }

    PBO::None(i32::MAX)
}

fn compare_packet_bit_option(
    option_left: PacketBitOption, 
    option_right: PacketBitOption
) -> Ordering {

    match (option_left, option_right) {
        (PBO::None(left_depth), PBO::None(right_depth))
            => return difference_to_ordering(left_depth - right_depth),
        
        (PBO::None(..), PBO::Some(..))
            => return Ordering::Less,
        
        (PBO::Some(..), PBO::None(..))
            => return Ordering::Greater,
        
        (PBO::Some(left_value, left_depth, left_is_last), PBO::Some(right_value, right_depth, right_is_last)) => {
            let difference = left_value - right_value;
            if difference != 0 {
                return difference_to_ordering(difference);
            }
            if left_depth == right_depth {
                return Ordering::Equal;
            }
            if left_is_last && right_is_last {
                return Ordering::Equal;
            }
            return difference_to_ordering(left_depth - right_depth);
        }
    }
}

fn compare_packets(left_packet: &Packet, right_packet: &Packet) -> Ordering {

    let mut left_index: usize = 0;
    let mut left_depth: i32 = 0;
    let mut right_index: usize = 0;
    let mut right_depth: i32 = 0;

    while left_index < left_packet.len() && right_index < right_packet.len() {
        
        let ordering = compare_packet_bit_option(
            get_next_packet_bit_option(
                left_packet, 
                &mut left_index, 
                &mut left_depth
            ),
            get_next_packet_bit_option(
                right_packet, 
                &mut right_index, 
                &mut right_depth
            )
        );

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    Ordering::Equal
}

fn difference_to_ordering(difference: i32) -> Ordering {
    if difference < 0 {
        return Ordering::Less;
    }
    if difference == 0 {
        return Ordering::Equal;
    }
    if difference > 0 {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}
