use std::env;
use std::fs;
mod seven_segment_search;
use crate::seven_segment_search::Measurement;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    // local_test_a();
    // solution_a();
    local_test_b();
    solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let measurements = Measurement::parse_input(input);

    // println!{"Test A> Input:"};
    // for m in measurements {
    //     println!{"{}", m};
    // }
    let expected_numbers = vec![2, 3, 3, 1, 3, 4, 3, 1, 4, 2];
    let expected_numbers_sum = expected_numbers.iter().fold(0, |sum, i| sum + i);
    let expected_total = 26;
    assert_eq!{
        expected_numbers_sum,
        expected_total,
        "Test A> Expected numbers sum does not match expected total: {} == {}",
        expected_numbers_sum,
        expected_total
    };
    let mut result_numbers: Vec<usize> = Vec::new();
    for i in 0..measurements.len() {
        let result_num = measurements[i].count_1478_in_output();
        assert_eq!{
            expected_numbers[i],
            result_num,
            "Test A> Measurement {}: Expected numbers does not match expected: {} == {}",
            i,
            expected_numbers[i],
            result_num
        }
        result_numbers.push(result_num);
    }

    let result = result_numbers.iter().fold(0, |sum, i| sum + i);
    assert_eq!{
        expected_total,
        result,
        "Test A> Expected total does not match result: {} == {}",
        expected_total,
        result
    };
    
    println!{"Test A> Result {}: {:?}", result, result_numbers};
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let measurements = Measurement::parse_input(input);
    let result_numbers = measurements.iter().map(|m| m.count_1478_in_output()).collect::<Vec<usize>>();
    let result = result_numbers.iter().fold(0, |sum, i| sum + i);
    println!{"Solution A> Result {}: {:?}", result, result_numbers};
}

fn local_test_b() {
    // Perform tests with simple input string
    let test_input = String::from("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
    let test_measurements = Measurement::parse_input(test_input);
    let test_result_numbers = test_measurements.iter()
        .inspect(|m| println!{"DEBUG B> Test Measurement: {}", m})
        .map(|m| m.decode_output())
        .collect::<Vec<u32>>();
    // let result = result_numbers.iter().fold(0, |sum, i| sum + i);
    println!{"DEBUG B> Test result {:?}", test_result_numbers};
    let expected_test_result = 5353;
    assert_eq!{test_result_numbers[0], expected_test_result, "DEBUG B> Mismatched test result: {} == {}", test_result_numbers[0], expected_test_result}
    
    // Perform tests with test input file
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let measurements = Measurement::parse_input(input);
    let result_numbers = measurements.iter()
        // .inspect(|m| println!{"TEST B> Test Measurement: {}", m})
        .map(|m| m.decode_output())
        .collect::<Vec<u32>>();
    println!{"TEST B> Test result {:?}", result_numbers};
    
    let expected_result_numbers = vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];
    for (i, (res, exp)) in result_numbers.iter().zip(expected_result_numbers.iter()).enumerate() {
        assert_eq!{res, exp, "TEST B> Expected result number ({}) mismatched: {} == {}", i, res, exp};
    }
    let result = result_numbers.iter().fold(0, |acc, n| acc + n);
    let expected_result = 61229;
    assert_eq!{result, expected_result, "TEST B> Mismatched result: {} == {}", result, expected_test_result}
    println!{"TEST B> Result: {}", result};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let measurements = Measurement::parse_input(input);
    let result_numbers = measurements.iter()
        // .inspect(|m| println!{"SOLUTION B> Test Measurement: {}", m})
        .map(|m| m.decode_output())
        .collect::<Vec<u32>>();
    let result = result_numbers.iter().fold(0, |acc, n| acc + n);
    println!{"SOLUTION B> Result: {}", result};
}

