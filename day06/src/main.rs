use std::env;
use std::fs;
mod lanternfish;
use crate::lanternfish::Fish;

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
    
    let population = Fish::parse_input(input);
    let max_days_1 = 18;
    let expected_population_1 = 26;
    let max_days_2 = 80;
    let expected_population_2 = 5934;
    println!("A> Test input:\n\tmax_days={}\n\t{:?}", max_days_2, population);

    let result_1 = Fish::sim(population.clone(), max_days_1);
    assert_eq!{result_1, expected_population_1, "A> Fish population does not match after {}: {} == {}", max_days_1, result_1, expected_population_1};

    let result_2 = Fish::sim(population.clone(), max_days_2);
    assert_eq!{result_2, expected_population_2, "A> Fish population does not match after {}: {} == {}", max_days_2, result_2, expected_population_2};

    println!{"A> Test result after {} days: {}", max_days_2, result_2};
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    
    let population = Fish::parse_input(input);
    let max_days = 80;
    println!("A> Solution input:\n\tmax_days={}\n\t{:?}", max_days, population);

    let result = Fish::sim(population, max_days);
    assert_eq!{result, 391671, "A> Fish population does not match after {}: {} == {}", max_days, result, 391671};

    println!{"A> Solution result after {} days: {}", max_days, result};
}

fn local_test_b() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    
    let population = Fish::parse_input(input);
    let max_days = 256;
    let expected_population = 26984457539;
    println!("B> Test input:\n\tmax_days={}\n\t{:?}", max_days, population);

    let result = Fish::sim(population, max_days);
    assert_eq!{result, expected_population, "B> Fish population does not match after {} days: {} == {}", max_days, result, expected_population};

    println!{"B> Test result after {} days: {}", max_days, result};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file).unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    
    let population = Fish::parse_input(input);
    let max_days = 256;
    println!("B> Solution input:\n\tmax_days={}\n\t{:?}", max_days, population);

    let result = Fish::sim(population, max_days);

    println!{"B> Solution result after {} days: {}", max_days, result};
}

