use std::env;
use std::fs;
mod crabs;
use crate::crabs::Crab;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a();
    solution_a();
    local_test_b();
    solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let crabs = Crab::parse_input(input);
    
    // let result = Crab::calculate_fuel_cost(crabs.clone(), 2);

    println!{"================"}

    // Test different positions
    let expected_h_pos_1 = 1;
    let expected_fuel_1 = 41;
    let result_fuel_1 = Crab::calculate_fuel_cost(crabs.clone(), expected_h_pos_1);
    assert_eq!{result_fuel_1, expected_fuel_1, "Test 1> Calculated fuel cost mismatches for h_pos {}: {} == {}", expected_h_pos_1, result_fuel_1, expected_fuel_1};
    println!{"================"}

    let expected_h_pos_2 = 3;
    let expected_fuel_2 = 39;
    let result_fuel_2 = Crab::calculate_fuel_cost(crabs.clone(), expected_h_pos_2);
    assert_eq!{result_fuel_2, expected_fuel_2, "Test 2> Calculated fuel cost mismatches for h_pos {}: {} == {}", expected_h_pos_2, result_fuel_2, expected_fuel_2};
    println!{"================"}

    let expected_h_pos_3 = 10;
    let expected_fuel_3 = 71;
    let result_fuel_3 = Crab::calculate_fuel_cost(crabs.clone(), expected_h_pos_3);
    assert_eq!{result_fuel_3, expected_fuel_3, "Test 3> Calculated fuel cost mismatches for h_pos {}: {} == {}", expected_h_pos_3, result_fuel_3, expected_fuel_3};
    println!{"================"}

    let expected_cheapest_h_pos = 2;
    let expected_cheapest_fuel = 37;
    let (result_cheapest_fuel, result_cheapest_h_pos) = Crab::find_cheapest_alignment_point(crabs.clone());
    assert_eq!{result_cheapest_fuel, expected_cheapest_fuel, "Test Cheap> Calculated fuel cost mismatches: {} == {}", result_cheapest_fuel, expected_cheapest_fuel};
    assert_eq!{result_cheapest_h_pos, expected_cheapest_h_pos, "Test Cheap> Calculated hpos cost mismatches: {} == {}", result_cheapest_h_pos, expected_cheapest_h_pos};
    println!{"A> Test result: Cheapest h_pos={} fuel_cost={}", result_cheapest_h_pos, result_cheapest_fuel};
    println!{"================"}
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let crabs = Crab::parse_input(input);

    let (result_cheapest_fuel, result_cheapest_h_pos) = Crab::find_cheapest_alignment_point(crabs.clone());

    println!{"A> Solution result: Cheapest h_pos={} fuel_cost={}", result_cheapest_h_pos, result_cheapest_fuel};
}

fn local_test_b() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let crabs = Crab::parse_input(input);

    println!{"================"}

    // Test different positions
    let expected_h_pos_1 = 2;
    let expected_fuel_1 = 206;
    let result_fuel_1 = Crab::calculate_fuel_cost_correct(crabs.clone(), expected_h_pos_1);
    assert_eq!{result_fuel_1, expected_fuel_1, "Test 1> Calculated correct fuel cost mismatches for h_pos {}: {} == {}", expected_h_pos_1, result_fuel_1, expected_fuel_1};
    println!{"================"}

    let expected_cheapest_h_pos = 5;
    let expected_cheapest_fuel = 168;
    let (result_cheapest_fuel, result_cheapest_h_pos) = Crab::find_cheapest_alignment_point_correct(crabs.clone());
    assert_eq!{result_cheapest_fuel, expected_cheapest_fuel, "Test Cheap> Calculated fuel cost mismatches: {} == {}", result_cheapest_fuel, expected_cheapest_fuel};
    assert_eq!{result_cheapest_h_pos, expected_cheapest_h_pos, "Test Cheap> Calculated hpos cost mismatches: {} == {}", result_cheapest_h_pos, expected_cheapest_h_pos};
    println!{"B> Test result: Cheapest h_pos={} fuel_cost={}", result_cheapest_h_pos, result_cheapest_fuel};
    println!{"================"}
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let crabs = Crab::parse_input(input);

    let (result_cheapest_fuel, result_cheapest_h_pos) = Crab::find_cheapest_alignment_point_correct(crabs.clone());

    println!{"B> Solution result: Cheapest h_pos={} fuel_cost={}", result_cheapest_h_pos, result_cheapest_fuel};
}

