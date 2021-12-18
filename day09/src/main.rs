use std::env;
use std::fs;
mod heightmap;
use crate::heightmap::HeightMap;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a();
    // solution_a();
    // local_test_b();
    // solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let heightmap = HeightMap::parse_input(input);
    println!{"TEST A> Input:\n{}", heightmap};

    
    
    // println!{"Test A> Result {}: {:?}", result, result_numbers};
}

// fn solution_a() {
//     let input_file = ".\\solution_input.txt";
//     let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    
//     println!{"Solution A> Result {}: {:?}", result, result_numbers};
// }

// fn local_test_b() {
//     // Perform tests with simple input string
//     let test_input = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
//     let test_measurements = Measurement::parse_input(test_input);
   
//     println!{"DEBUG B> Test result {:?}", test_result_numbers};
//     let expected_test_result = 5353;
//     assert_eq!{test_result_numbers[0], expected_test_result, "DEBUG B> Mismatched test result: {} == {}", test_result_numbers[0], expected_test_result}
    
//     // Perform tests with test input file
//     let input_file = ".\\test_input.txt";
//     let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
//     let measurements = Measurement::parse_input(input);
    
//     println!{"TEST B> Test result {:?}", result_numbers};
    
    
//     let result = result_numbers.iter().fold(0, |acc, n| acc + n);
//     let expected_result = 61229;
//     assert_eq!{result, expected_result, "TEST B> Mismatched result: {} == {}", result, expected_test_result}
//     println!{"TEST B> Result: {}", result};
// }

// fn solution_b() {
//     let input_file = ".\\solution_input.txt";
//     let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    
//     let result = result_numbers.iter().fold(0, |acc, n| acc + n);
//     println!{"SOLUTION B> Result: {}", result};
// }

