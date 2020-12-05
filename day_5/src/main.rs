use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let data = read_file();

    let passes: HashSet<usize> = data.iter()
        .map(|string| seat_id(parse_boarding_pass(string.to_string())))
        .collect::<HashSet<usize>>();

    let max_seat_id = passes.iter().max().unwrap();
    println!("Day 5 Phase 1. Max Boarding Pass = {}", max_seat_id);

    for possible_seat_id in 32..*max_seat_id {
        if !passes.contains(&possible_seat_id) {
            println!("Day 5 Phase 2. Your Seat ID = {}", possible_seat_id);
        }
    }
}

// BP = [lower_row, upper_row, lower_column, upper_column]
fn parse_boarding_pass(string: String) -> [usize; 4] {
    string.chars().fold([0, 127, 0, 7], |val, c| {
        match c {
            'F' => [val[0], half(val[0], val[1]), val[2], val[3]],
            'B' => [half(val[0], val[1]) + 1, val[1], val[2], val[3]],
            'R' => [val[0], val[1], half(val[2], val[3]) + 1, val[3]],
            'L' => [val[0], val[1], val[2], half(val[2], val[3])],
            _ => panic!("Whaaaat {}", c)
        }
    })
}

fn half(lower: usize, upper: usize) -> usize {
    (upper + 1 - lower) / 2 + lower - 1
}

fn seat_id(boarding_pass: [usize; 4]) -> usize {
    return boarding_pass[0] * 8 + boarding_pass[2];
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("day_5/src/day_5_input.txt").unwrap());
    file.lines().map(|l| l.unwrap()).collect()
}