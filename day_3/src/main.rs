use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    solve_day_3_phase_1();
    solve_day_3_phase_2();   
}

fn solve_day_3_phase_1() {
    let trees =  count_trees(&parse_file(), 3, 1);
    println!("Day 3 Phase 1 => Trees {}", trees);
}

fn solve_day_3_phase_2() {
    let map = parse_file();
    let result = 
        count_trees(&map, 1, 1) *
        count_trees(&map, 3, 1) *
        count_trees(&map, 5, 1) *
        count_trees(&map, 7, 1) *
        count_trees(&map, 1, 2);

    println!("Day 3 Phase 2 => Trees {}", result);
}

fn count_trees(map: &Vec<String>, x_slope: usize, y_slope: usize) -> usize {
    map
        // Remove lines by "y" jump
        .iter()
        .enumerate()
        .filter(|&(idx, _)| idx % y_slope == 0)
        .map(|(_, tile)| tile)
        .collect::<Vec<&String>>()
        // Remove non-tree lines by "x" jump
        .iter()
        .enumerate()
        .filter(|&(idx, tile)| is_tree(tile, idx * x_slope % tile.len()))
        // Count trees
        .count()
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