use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let (bag_contains, bag_contained_in) = create_graph();
    phase_1(bag_contained_in);
    phase_2(bag_contains);
}

type Graph = HashMap<String, HashMap<String, usize>>;

fn phase_1(bag_contained_in: Graph) {
    let bags = traverse(&bag_contained_in, vec!["shiny gold"], HashSet::new());
    println!("Day 7 Phase 1: {}", bags.len() - 1);
}

fn traverse(graph: &Graph, to_add: Vec<&str>, mut visited: HashSet<String>) -> HashSet<String> {
    let new_nodes: Vec<&str> = to_add.iter().filter(|x| !visited.contains(**x)).map(|x| *x).collect();
    
    if new_nodes.is_empty() {
        return visited;
    }

    let next_to_add: Vec<&str> = new_nodes.iter()
        .flat_map(|new_node| graph.get(*new_node).unwrap().keys())
        .map(|x| x.as_str())
        .collect();
    to_add.iter().for_each(|x| { visited.insert(x.to_string()); });

    return traverse(graph, next_to_add, visited)
}

fn phase_2(bag_contains: Graph) {
    let how_many_bags = open(&bag_contains, &bag_contains.get("shiny gold").unwrap());
    println!("Day 7 Phase 2: {}", how_many_bags);
}

fn open(bag_contains: &Graph, bag: &HashMap<String, usize>) -> usize {
    if bag.is_empty() { return 0; }

    bag.iter()
        .map(|(contained_bag, times)| times + times * open(bag_contains, &bag_contains.get(contained_bag).unwrap()))
        .sum()
}

fn create_graph() -> (Graph, Graph) {
    let mut bag_contains: Graph = HashMap::new();
    let mut bag_contained_in: Graph = HashMap::new();
    let contains_regex = Regex::new(r"(\d+) ([a-z ]+) bags?.?").unwrap();

    read_file().iter().for_each(|rule| {
        let bag = rule.split("bags").nth(0).unwrap().trim();
        let contains_str: Vec<&str> = rule.split("contain").nth(1).unwrap()
            .split(',')
            .map(|x| x.trim())
            .collect();

        bag_contains.insert(bag.to_string(), HashMap::new());
        if !bag_contained_in.contains_key(bag) {
            bag_contained_in.insert(bag.to_string(), HashMap::new());
        }

        contains_str.iter().for_each(|contains_bag_str| { // N {bag_name} bag(s.)
            if contains_bag_str.eq_ignore_ascii_case("no other bags.") {
                return;
            }
            let caps = contains_regex.captures(contains_bag_str).unwrap();
            let count = caps.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let bag_name = caps.get(2).map_or("", |m| m.as_str()).trim();

            if !bag_contained_in.contains_key(bag_name) {
                bag_contained_in.insert(bag_name.to_string(), HashMap::new());
            }

            bag_contains.get_mut(&bag.to_string()).unwrap().insert(bag_name.to_string(), count);
            bag_contained_in.get_mut(&bag_name.to_string()).unwrap().insert(bag.to_string(), count);
        });
    });

    return (bag_contains, bag_contained_in);
}


fn read_file() -> Vec<String> {
    let file = BufReader::new(File::open("day_7/src/day_7_input.txt").unwrap());
    file.lines().map(|l| l.unwrap()).collect()
}