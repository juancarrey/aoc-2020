use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
 solve_day_3(1);   
}

fn solve_day_3(phase: i32) {
    let map: Vec<String> = parse_file();
    const slope: usize = 3;

    let trees = map
        .iter()
        .enumerate()
        .filter(|&(idx, tile)| is_tree(tile, idx * slope % tile.len()))
        .count();

    println!("Day 3 Phase {} => Trees {}", phase, trees);
}

fn is_tree(tile: &String, pos: usize) -> bool {
    tile.chars().nth(pos) == Some('#')
}


fn parse_file() -> Vec<String> {
    let path = Path::new("day_3/src/day_3_input.txt");
    let file = BufReader::new(File::open(&path).unwrap());

    return file.lines()
        .map(|line_result| {
            line_result.unwrap()
        }).collect::<Vec<String>>();
}