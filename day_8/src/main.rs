use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

type Arguments = i64;
type Instruction = (String, Arguments);
type Program = Vec<Instruction>;
type State = i64;

fn main() {
    phase_1();
}

fn phase_1() {
    let program = create_program();
    let result = run_program(program);
    println!("Day 8 Phase 1: {} ", result);
}

fn create_program() -> Program {
    read_file().iter().map(|line| {
        let instruction: Vec<&str> = line.split(' ').collect();
        ( 
            instruction.iter().nth(0).unwrap().to_string(), 
            instruction.iter().nth(1).unwrap().parse::<i64>().unwrap()
        )
    }).collect::<Vec<(String, Arguments)>>()
}

fn run_program(program: Program) -> State {
    let mut state: State = 0;
    let mut current_instruction_idx: usize = 0;
    let mut loc_covered: HashSet<usize> = HashSet::new();
    loop {
        if loc_covered.contains(&current_instruction_idx) { break; }

        loc_covered.insert(current_instruction_idx);
        let current_instruction = program.iter().nth(current_instruction_idx).unwrap();
        match current_instruction.0.as_str() {
            "jmp" => { 
                current_instruction_idx = jump(current_instruction_idx as i64, current_instruction.1) as usize;
            },
            "acc" => {
                state = acc(state, current_instruction.1);
                current_instruction_idx = current_instruction_idx + 1;
            } ,
            _ => current_instruction_idx = current_instruction_idx + 1,
        }
    }

    return state;
}

fn jump(current_instruction: i64, args: Arguments) -> i64 {
    current_instruction + args
}

fn acc(state: State, args: Arguments) -> i64 {
    state + args
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("day_8/src/day_8_input.txt").unwrap());
    file.lines().map(|l| l.unwrap()).collect()
}