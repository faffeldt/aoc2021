use std::env;
use std::fmt;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test();
    solution();
}

fn local_test() {
    let items: Vec<usize> = include_str!("..\\test_input.txt")
        .lines()
        // .inspect(|i| println!{"{} -> {:#b}", i, usize::from_str_radix(i, 2).unwrap()})
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .collect();
    println!("> Test input ({}): {:?}", items.len(), items);
    let (gamma_rate, epsilon_rate) = calculate_rates(items, 5);
    let result = gamma_rate * epsilon_rate;
    
    let expected_gamma_rate = 0b10110;  // 22
    let expected_epsilon_rate = 0b01001;  // 9
    let expected_result = 198;
    println! {"> exp:\t{:#b},\t{:#b},\t{}\n  got:\t{:#b},\t{:#b},\t{}", expected_gamma_rate, expected_epsilon_rate, expected_result, gamma_rate, epsilon_rate, result}
    if result != expected_result {
        panic! {"> Solution does not match expected result"};
    }
}

fn solution() {
    let items: Vec<usize> = include_str!("..\\solution_input.txt")
        .lines()
        // .inspect(|i| println!{"{} -> {:#b}", i, usize::from_str_radix(i, 2).unwrap()})
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .collect();
    println!("> Solution input ({}): {:?}", items.len(), items);
    let (gamma_rate, epsilon_rate) = calculate_rates(items, 12);
    let result = gamma_rate * epsilon_rate;
    println! {"> Solution:\t{} ({:#b})\t*\t{} ({:#b})\t-> {}", gamma_rate, gamma_rate, epsilon_rate, epsilon_rate, result}
}

fn calculate_rates(measurements: Vec<usize>, num_bits: usize) -> (usize, usize){
    let mut counters: Vec<usize> = vec![0; num_bits];  // usize::BITS as usize
    // Count the 1 in ith bit position of each measurement
    for m in &measurements {
        for i in 0..num_bits {
            counters[i] = counters[i] + ((m & (1 << num_bits - i) != 0) as usize);
        }
        println!{"DEBUG> {:#b}\t{:?}", m, counters }
    }
    // Check each counter if it is greater than half of the measurements
    let mut gamma_rate = 0b0;
    for c in 0..counters.len() {
        println!{"DEBUG> [{}] {} > {}\t{}\tBit which would be set {:#b}", c, counters[c], (measurements.len() / 2), counters[c] > (measurements.len() / 2), (1 << (num_bits - c))}
        if counters[c] > (measurements.len() / 2) {
            gamma_rate = gamma_rate | (1 << (num_bits - c))
        }
    }
    println!{"Gamma rate: {:#b} {:?}", gamma_rate, counters}
    let epsilon_rate = gamma_rate ^ (usize::MAX >> (usize::BITS as usize - num_bits));

    (gamma_rate, epsilon_rate)
}

