use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn solve_problem_1_phase_1() {
    let input = read_input();

    for num in input.iter() {
        let remaining = 2020 - num;
        if input.contains(&remaining) {
            println!("Result is {} * {} = {}", num, remaining, num * remaining);
            break;
        }
    }
}

fn read_input() -> HashSet<i32> {
    let path = Path::new("src/problem_1/phase_1_input.txt");
    let file = BufReader::new(File::open(&path).unwrap());

    let mut expenses = HashSet::new();
    for line in file.lines() {
        expenses.insert(line.unwrap().parse::<i32>().unwrap());
    }

    return expenses;
}

