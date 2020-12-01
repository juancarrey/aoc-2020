use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn solve_problem_1_phase_1() -> i32 {
    let input = read_input();

    for num in input.iter() {
        let remaining = 2020 - num;
        if input.contains(&remaining) {
            println!("Result (phase1) is {} * {} = {}", num, remaining, num * remaining);
            return num * remaining;
        }
    }

    return -1;
}

pub fn solve_problem_1_phase_2() -> i32 {
    let input = read_input();

    for num1 in input.iter() {
        for num2 in input.iter() {
            if num1 != num2 {
                let remaining = 2020 - num1 - num2;
                if input.contains(&remaining) {
                    println!("Result (phase2) is {} * {} * {} = {}", num1, num2, remaining, num1 * num2 * remaining);
                    return num1 * num2 * remaining;
                }
            }
        }
    }

    return -1;
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

