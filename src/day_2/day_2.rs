use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

struct PasswordPolicy {
    min: i32,
    max: i32,
    char: char,
}

impl PasswordPolicy {
    fn parse(policy_def: &str) -> PasswordPolicy {
        let range: Vec<&str> = policy_def[..policy_def.chars().count() - 2].split('-').collect();
        PasswordPolicy {
            min: range[0].trim().parse::<i32>().unwrap(),
            max: range[1].trim().parse::<i32>().unwrap(),
            char: policy_def.chars().last().unwrap()
        }
    }

    fn validate(self, password: &str) -> bool {
        let count: i32 = password.chars().map(|char| {
            return if char == self.char {
                1
            } else {
                0
            }
        }).sum();
        return count >= self.min && count <= self.max;
    }
}

pub fn solve_day_2_phase_1() -> i32 {
    let path = Path::new("src/day_2/day_2_input.txt");
    let file = BufReader::new(File::open(&path).unwrap());

    let valid_passwords = file.lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            let parsed_line: Vec<&str> = line.split(':').collect::<Vec<&str>>();
            let policy_def = parsed_line[0].trim();
            let password = parsed_line[1].trim();

            let policy = PasswordPolicy::parse(policy_def);
            return if policy.validate(password) {
                1
            } else {
                0
            }
        }).sum();

    println!("Valid passwords {}", valid_passwords);
    return valid_passwords;
}
