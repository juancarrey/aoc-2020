use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    solve_day_2_phase_1();
    solve_day_2_phase_2();
}

struct PasswordPolicy {
    min: usize,
    max: usize,
    char: char,
}

impl PasswordPolicy {
    fn parse(policy_def: &str) -> PasswordPolicy {
        let range: Vec<&str> = policy_def[..policy_def.chars().count() - 2].split('-').collect();
        PasswordPolicy {
            min: range[0].trim().parse::<usize>().unwrap(),
            max: range[1].trim().parse::<usize>().unwrap(),
            char: policy_def.chars().last().unwrap()
        }
    }

    fn validate_phase_1(self, password: &str) -> bool {
        return within_range(password.chars().map(|char| {
            return if char == self.char {
                1
            } else {
                0
            }
        }).sum(), self.min, self.max);
    }

    fn validate_phase_2(self, password: &str) -> bool {
        let expected = Some(self.char);
        return xor(
            password.chars().nth(self.min - 1) == expected,
            password.chars().nth(self.max - 1) == expected);
    }
}

fn within_range(val: usize, min: usize, max: usize) -> bool {
    return val >= min && val <= max;
}

fn xor(a: bool, b: bool) -> bool {
    return (a || b) && !(a && b);
}

fn solve_day_2_phase_2() -> usize {
    solve_day_2(2)
}

fn solve_day_2_phase_1() -> usize {
    solve_day_2(1)
}

fn solve_day_2(phase: i32) -> usize {
    let path = Path::new("day_2/src/day_2_input.txt");
    let file = BufReader::new(File::open(&path).unwrap());

    let valid_passwords = file.lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            let parsed_line: Vec<&str> = line.split(':').collect::<Vec<&str>>();
            let policy_def = parsed_line[0].trim();
            let password = parsed_line[1].trim();

            let policy = PasswordPolicy::parse(policy_def);
            let valid =
                if phase == 1 { policy.validate_phase_1(password) }
                else { policy.validate_phase_2(password) };

            return if valid { 1 } else { 0 }
        }).sum();

    println!("Day 2 Phase {} => Valid passwords {}", phase, valid_passwords);
    return valid_passwords;
}
