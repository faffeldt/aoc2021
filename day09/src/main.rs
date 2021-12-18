use std::env;
use std::fs;
mod heightmap;
use crate::heightmap::HeightMap;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a();
    solution_a();
    // local_test_b();
    // solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let heightmap = HeightMap::parse_input(input);
    println!{"TEST A> Input:\n{}", heightmap};

    let expected_low_points = vec![1, 0, 5, 5];
    let low_points = heightmap.low_points();
    assert_eq!{expected_low_points, low_points, "TEST A> Low points mismatch: {:?} == {:?}", expected_low_points, low_points};

    let expected_risks = vec![2, 1, 6, 6];
    let risks: Vec<usize> = low_points.iter()
        .map(|&p| HeightMap::risk_of_height(p))
        .collect();
    assert_eq!{expected_risks, risks, "TEST A> Risks mismatch: {:?} == {:?}", expected_risks, risks};

    let expected_total_risk = 15;
    // let total_risk = risks.iter().fold(0, |acc, r| acc + r);
    let total_risk = heightmap.total_risk();
    assert_eq!{expected_total_risk, total_risk, "TEST A> Total risk mismatches: {} == {}", expected_total_risk, total_risk};

    println!{"Test A> Total risk: {}", total_risk};
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let heightmap = HeightMap::parse_input(input);
    println!{"SOLUTION A> Input:\n{}", heightmap};

    let total_risk = heightmap.total_risk();
    println!{"SOLUTION A> Total risk: {}", total_risk};
}

// fn local_test_b() {
//     let input_file = ".\\test_input.txt";
//     let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
//     let heightmap = HeightMap::parse_input(input);
//     println!{"TEST B> Input:\n{}", heightmap};

//     let expected_low_points = vec![1, 0, 5, 5];
//     let low_points = heightmap.low_points();
//     assert_eq!{expected_low_points, low_points, "TEST B> Low points mismatch: {:?} == {:?}", expected_low_points, low_points};

//     let expected_risks = vec![2, 1, 6, 6];
//     let risks: Vec<usize> = low_points.iter()
//         .map(|&p| HeightMap::risk_of_height(p))
//         .collect();
//     assert_eq!{expected_risks, risks, "TEST B> Risks mismatch: {:?} == {:?}", expected_risks, risks};

//     let expected_total_risk = 15;
//     // let total_risk = risks.iter().fold(0, |acc, r| acc + r);
//     let total_risk = heightmap.total_risk();
//     assert_eq!{expected_total_risk, total_risk, "TEST B> Total risk mismatches: {} == {}", expected_total_risk, total_risk};

//     println!{"Test B> Total risk: {}", total_risk};
// }

// fn solution_b() {
//     let input_file = ".\\solution_input.txt";
//     let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
//     let heightmap = HeightMap::parse_input(input);
//     println!{"SOLUTION B> Input:\n{}", heightmap};

//     let total_risk = heightmap.total_risk();
//     println!{"SOLUTION B> Total risk: {}", total_risk};
// }

