use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub fn solve_problem_1_phase_1() -> i32 {
    let input = read_input();

    for (num, count) in input.iter() {
        let remaining = 2020 - num;
        if input.contains_key(&remaining) && *num != remaining {
            println!("Result (phase1) is {} * {} = {}", num, remaining, num * remaining);
            return num * remaining;
        } else if *num == 1010 && *count > 1 {
            println!("Result (phase1) is {} * {} = {}", num, num, num * num);
            return num * num;
        }
    }

    return -1;
}

pub fn solve_problem_1_phase_2() -> i32 {
    let input = read_input();
    println!("{:?}", input);

    for (num1, countNum1) in input.iter() {
        // 3a = 2020
        if *countNum1 >= 3 && num1 * 3 == 2020 {
            println!("Result (phase2) is {} * {} * {} = {}", num1, num1, num1, num1 * num1 * num1);
            return num1 * num1 * num1;
        }

        // a+a+remaining=2020
        if *countNum1 >= 2 {
            let remaining = 2020 - 2 * num1;
            if input.contains_key(&remaining) {
                println!("Result (phase2) is {} * {} * {} = {}", num1, num1, remaining, num1 * num1 * remaining);
                return num1 * num1 * remaining;
            }
        }

        // a+b+remaining=2020
        for (num2, _) in input.iter() {
            if *num1 != *num2 {
                let remaining = 2020 - num1 - num2;
                if input.contains_key(&remaining) {
                    println!("Result (phase2) is {} * {} * {} = {}", num1, num2, remaining, num1 * num2 * remaining);
                    return num1 * num2 * remaining;
                }
            }
        }
    }

    return -1;
}

fn read_input() -> HashMap<i32, i32> {
    let path = Path::new("src/problem_1/phase_1_input.txt");
    let file = BufReader::new(File::open(&path).unwrap());

    let mut expenses = HashMap::new();
    for line in file.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        if let Some(x) = expenses.get_mut(&num) {
            *x = *x + 1;
        } else {
            expenses.insert(num, 1);
        }
    }

    return expenses;
}

