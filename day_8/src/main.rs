use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

type Arguments = i64;
type Instruction = (String, Arguments);
type Program = Vec<Instruction>;
type State = i64;
type ProgramResult = (State, bool, HashSet<usize>);

fn main() {
    phase_1();
    phase_2();
}

fn phase_1() {
    let program = create_program();
    let result = run_program(&program);
    println!("Day 8 Phase 1: {} ", result.0);
}

fn phase_2() {
    let mut program = create_program();
    let result = run_program_fixing(&mut program);
    println!("Day 8 Phase 2: {} ", result.0);
}

fn create_program() -> Program {
    read_file().iter().map(|line| {
        let instruction: Vec<&str> = line.split(' ').collect();
        println!("{:?}", instruction);
        ( 
            instruction.iter().nth(0).unwrap().to_string(), 
            instruction.iter().nth(1).unwrap().parse::<i64>().unwrap()
        )
    }).collect::<Vec<(String, Arguments)>>()
}

fn run_program_fixing(program: &mut Program) -> ProgramResult {
    let loc_covered_normal = run_program(program).2;

    // If the program "ends" on each line without modifying it
    let loc_ends_program: Vec<bool> = program.iter().enumerate().map(|(loc, _)| {
        continue_program(program, loc, &HashSet::new()).1
    }).collect();

    // Find instruction to be swapped:
    // It does NOT end the program
    // It does end if we swap (Next instruction ends without changing it.)
    // It is also covered on normal run (we can reach it without modifying the program)
    let loc_to_change: usize = *program
        .iter()
        .enumerate().filter(|(loc, instruction)| {
        if !loc_covered_normal.contains(loc) { return false; }
        if *loc_ends_program.iter().nth(*loc).unwrap() { return false; }
        if *loc >= program.len() - 1 { return false; }

        match instruction.0.as_str() {
            "jmp" => *loc_ends_program.iter().nth(loc + 1).unwrap(),
            "nop" => *loc_ends_program.iter().nth(jump(*loc, instruction.1)).unwrap(),
            _ => false
        }
    })
    .map(|(loc, _)| loc)
    .collect::<Vec<usize>>()
    .iter().nth(0).unwrap();

    // Fix wrong Instruction (Swap)
    let instruction = program.iter().nth(loc_to_change).unwrap();
    let swapped = match instruction.0.as_str() {
        "jmp" => (String::from("nop"), instruction.1),
        "nop" => (String::from("jmp"), instruction.1),
        _ => panic!("Whaat")
    };
    let range = std::ops::Range { start: loc_to_change, end: loc_to_change + 1 };
    program.splice(range, vec![swapped]);

    return run_program(program);
}

fn run_program(program: &Program) -> ProgramResult {
    return continue_program(program, 0 as usize, &HashSet::new());
}

fn continue_program(program: &Program, start_instruction_idx: usize, already_covered_lines: &HashSet<usize>) -> (State, bool, HashSet<usize>) {
    // println!("Running program");
    let mut state: State = 0;
    let mut current_instruction_idx: usize = start_instruction_idx;
    let mut loc_covered: HashSet<usize> = HashSet::new();
    already_covered_lines.iter().for_each(|loc| { loc_covered.insert(*loc); });

    loop {
        if loc_covered.contains(&current_instruction_idx) { break; }
        if current_instruction_idx >= program.len() { break; }

        loc_covered.insert(current_instruction_idx);
        let current_instruction = program.iter().nth(current_instruction_idx).unwrap();
        match current_instruction.0.as_str() {
            "jmp" => { 
                // println!("Jump {}", current_instruction.1);
                current_instruction_idx = jump(current_instruction_idx, current_instruction.1) as usize;
            },
            "acc" => {
                // println!("Acc {}", current_instruction.1);
                state = acc(state, current_instruction.1);
                current_instruction_idx = current_instruction_idx + 1;
            } ,
            _ => current_instruction_idx = current_instruction_idx + 1,
        }

        // println!("State {}, Next LOC= {}", state, current_instruction_idx);
    }

    return (state, current_instruction_idx >= program.len(), loc_covered);
}

fn next_line(current_instruction: usize) -> usize {
    current_instruction + 1
}

fn jump(current_instruction: usize, args: Arguments) -> usize {
    (current_instruction as i64 + args) as usize
}

fn acc(state: State, args: Arguments) -> i64 {
    state + args
}

fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("day_8/src/day_8_input.txt").unwrap());
    file.lines().map(|l| l.unwrap()).collect()
}