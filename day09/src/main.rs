use std::env;
use std::fs;
mod heightmap;
use crate::heightmap::HeightMap;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    // local_test_a();
    // solution_a();
    local_test_b();
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

fn local_test_b() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut heightmap = HeightMap::parse_input(input);
    println!{"TEST B> Input:\n{}", heightmap};

    //  X 0 1 2 3 4 5 6 7 8 9
    // Y
    // 0  2 1 9 9 9 4 3 2 1 0
    // 1  3 9 8 7 8 9 4 9 2 1
    // 2  9 8 5 6 7 8 9 8 9 2
    // 3  8 7 6 7 8 9 6 7 8 9
    // 4  9 8 9 9 9 6 5 6 7 8
    let expected_basins_sizes = vec![3, 9, 14, 9];
    let basins = heightmap.find_basins();
    println!{"TEST B> After basins:\n{}", heightmap};
    // let basins_sizes: Vec<usize> = basins.iter().map(|b| b.len()).collect();
    assert_eq!{expected_basins_sizes, basins, "TEST B> Basin sizes mismatch: {:?} == {:?}", expected_basins_sizes, basins};

    let expected_result = 1134;
    // Multiply largest 3 basins
    let result = basins.iter()
        .rev()
        .take(3)
        .product::<usize>();
    assert_eq!{expected_result, result, "TEST B> Total result mismatches: {} == {}", expected_result, result};

    println!{"Test B> Total result: {}", result};
}

// fn solution_b() {
//     let input_file = ".\\solution_input.txt";
//     let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
//     let heightmap = HeightMap::parse_input(input);
//     println!{"SOLUTION B> Input:\n{}", heightmap};

//     let result = heightmap.result();
//     println!{"SOLUTION B> Total result: {}", result};
//     3780
// }

