use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    solve_day_4_phase_1();
    // solve_day_4_phase_2();
}

struct Color(String);
struct PassportID(String);
struct CountryID(String);
struct Year(usize);
struct Height(String);

struct Passport {
    byr: Option<Year>,
    iyr: Option<Year>,
    eyr: Option<Year>,
    hgt: Option<Height>,
    hcl: Option<Color>,
    ecl: Option<Color>,
    pid: Option<PassportID>,
    cid: Option<CountryID>,
}

impl Passport {
    fn parse(passport: &str) -> Passport {
        Passport {
            byr: parse_year(passport, "byr:", " "),
            iyr: parse_year(passport, "iyr:", " "),
            eyr: parse_year(passport, "eyr:", " "),
            hgt: parse_height(passport, "hgt:", " "),
            hcl: parse_color(passport, "hcl:", " "),
            ecl: parse_color(passport, "ecl:", " "),
            pid: parse_passport_id(passport, "pid:", " "),
            cid: parse_country_id(passport, "cid:", " "),
        }
    }

    fn valid(self, phase: i32) -> bool {
        return self.valid_phase_1();
    }

    fn valid_phase_1(self) -> bool {
        return
            self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some();
    }

    fn valid_phase_2(self) -> bool {
        return false;
    }
}

fn parse_value(passport: &str, left: &str, right: &str) -> Option<String> {
    passport.split(left).nth(1).map(|yr| yr.split(right).nth(0).map(|s| s.to_string())).flatten()
}

fn parse_year(passport: &str, left: &str, right: &str) -> Option<Year> {
    parse_value(passport, left, right).map(|value| {
        Year(value.parse::<usize>().unwrap())
    })
}

fn parse_color(passport: &str, left: &str, right: &str) -> Option<Color> {
    parse_value(passport, left, right).map(|value| Color(value))
}

fn parse_passport_id(passport: &str, left: &str, right: &str) -> Option<PassportID> {
    parse_value(passport, left, right).map(|value| PassportID(value))
}

fn parse_country_id(passport: &str, left: &str, right: &str) -> Option<CountryID> {
    parse_value(passport, left, right).map(|value| CountryID(value))
}

fn parse_height(passport: &str, left: &str, right: &str) -> Option<Height> {
    parse_value(passport, left, right).map(|value| Height(value))
}

fn solve_day_4_phase_2() -> usize {
    solve_day_4(2)
}

fn solve_day_4_phase_1() -> usize {
    solve_day_4(1)
}

fn solve_day_4(phase: i32) -> usize {
    let file = read_file();

    let valid_passports = file.iter()
        .filter(|line| Passport::parse(line).valid(phase) )
        .count();

    println!("Day 4 Phase {} => Valid passports {}", phase, valid_passports);
    return valid_passports;
}


fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("day_4/src/day_4_input.txt").unwrap());

    let mut list = Vec::new();
    let mut current: String = String::from(" ");
    for (line) in file.lines() {
        let item = line.unwrap();
        if item.is_empty() {
            current.push_str(&" ".to_string());
            list.push(current);
            current = String::from(" ");
        } else {
            current.push_str(&" ".to_string());
            current.push_str(&item);
        }
    }

    current.push_str(&" ".to_string());
    list.push(current); 

    return list;
}