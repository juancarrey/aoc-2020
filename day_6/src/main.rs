use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    phase_1();
    phase_2();
}

fn phase_1() {
    let data = read_file();

    let total_yes_answers: usize = data.iter()
        .map(|line| line.chars().filter(|x| *x != ' ').collect::<HashSet<char>>().len()).sum();

    println!("Day 6 Phase 1 = {}", total_yes_answers);
}

fn phase_2() {
    let data = read_file();

    let total_yes_answers: usize = data.iter()
        .map(|line| 
            count_group_yes_answers(line.split(' ').filter(|l| !l.is_empty()).collect())
        )
        .sum();

    println!("Day 6 Phase 2 = {}", total_yes_answers);
}

fn count_group_yes_answers(group_answers: Vec<&str>) -> usize {
    let mut answers : HashMap<char, usize> = HashMap::new();
    group_answers.iter()
        .map(|answers| answers.chars().filter(|x| *x != ' '))
        .flatten()
        .collect::<Vec<char>>()
        .iter()
        .for_each(|ch| *answers.entry(*ch).or_insert(0) += 1);

    return answers.iter().filter(|(_, count)| **count == group_answers.len()).count();
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("day_6/src/day_6_input.txt").unwrap());

    let mut list = Vec::new();
    let mut current: String = String::from("");
    for line in file.lines() {
        let item = line.unwrap();
        if item.is_empty() {
            current.push_str(&" ".to_string());
            list.push(current);
            current = String::from("");
        } else {
            current.push_str(&item);
            current.push_str(&" ".to_string());
        }
    }

    current.push_str(&" ".to_string());
    list.push(current); 

    return list;
}