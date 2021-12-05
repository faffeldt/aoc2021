use std::env;
mod vent_map;
use crate::vent_map::VentMap;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    // local_test_a();
    // solution_a();
    local_test_b();
    solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let vent_map = VentMap::from_file(input_file);
    println!("> Test input:\n{}", vent_map);
    vent_map.draw();
    let num_overlapping_lines = vent_map.number_of_overlapping_lines();
    let expected_num_overlapping_lines = 5;
    assert_eq!{
        num_overlapping_lines, 
        expected_num_overlapping_lines, 
        "Number of overlapping lines does not match: {} == {},",
        num_overlapping_lines,
        expected_num_overlapping_lines
    }
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let vent_map = VentMap::from_file(input_file);
    println!("> Test input:\n{}", vent_map);
    // vent_map.draw();
    let num_overlapping_lines = vent_map.number_of_overlapping_lines();
    println!{"> Solution: Number of overlapping lines: {}", num_overlapping_lines}
    
}

fn local_test_b() {
    let input_file = ".\\test_input.txt";
    let vent_map = VentMap::from_file(input_file);
    println!("> Test input:\n{}", vent_map);
    vent_map.draw();
    let num_overlapping_lines = vent_map.number_of_overlapping_lines();
    let expected_num_overlapping_lines = 12;
    assert_eq!{
        num_overlapping_lines, 
        expected_num_overlapping_lines, 
        "Number of overlapping lines does not match: {} == {},",
        num_overlapping_lines,
        expected_num_overlapping_lines
    }
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let vent_map = VentMap::from_file(input_file);
    println!("> Test input:\n{}", vent_map);
    // vent_map.draw();
    let num_overlapping_lines = vent_map.number_of_overlapping_lines();
    println!{"> Solution: Number of overlapping lines: {}", num_overlapping_lines}
    
}

