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
        // .inspect(|i| println!{"{} -> {:#14b}", i, usize::from_str_radix(i, 2).unwrap()})
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .collect();
    println!("> Test input ({}): {:?}", items.len(), items);
    let (gamma_rate, epsilon_rate) = calculate_rates(items, 5);
    let result = gamma_rate * epsilon_rate;
    
    let expected_gamma_rate = 0b10110;  // 22
    let expected_epsilon_rate = 0b01001;  // 9
    let expected_result = 198;
    println! {"> exp:\t{:#14b},\t{:#14b},\t{}\n  got:\t{:#14b},\t{:#14b},\t{}", expected_gamma_rate, expected_epsilon_rate, expected_result, gamma_rate, epsilon_rate, result}
    if result != expected_result {
        panic! {"> Solution does not match expected result"};
    }
}

fn solution() {
    let items: Vec<usize> = include_str!("..\\solution_input.txt")
        .lines()
        // .inspect(|i| println!{"{} -> {:#14b}", i, usize::from_str_radix(i, 2).unwrap()})
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .collect();
    println!("> Solution input ({}): {:?}", items.len(), items);
    let (gamma_rate, epsilon_rate) = calculate_rates(items, 12);
    let result = gamma_rate * epsilon_rate;
    println! {"> Solution:\t{} ({:#14b})\t*\t{} ({:#14b})\t-> {}", gamma_rate, gamma_rate, epsilon_rate, epsilon_rate, result}
}

fn calculate_rates(measurements: Vec<usize>, num_bits: usize) -> (usize, usize){
    let mut counters: Vec<usize> = vec![0; num_bits];
    // Count the 1 in ith bit position of each measurement
    for measurement in &measurements {
        for i in 0..num_bits {
            let bit_mask = 1 << (num_bits - 1) - i;
            println!{"DEBUG> [{:2}] m={:#14b} m&bm={:#14b} -> {}", i, measurement, measurement & bit_mask, (measurement & bit_mask != 0)}
            println!{"           -> {:#14b}", bit_mask}
            counters[i] = counters[i] + ((measurement & bit_mask != 0) as usize);
        }
        println!{"DEBUG> Counters: {:?}", counters }
    }
    // Check each counter if it is greater than half of the measurements
    let mut gamma_rate = 0b0;
    for c in 0..counters.len() {
        println!{"DEBUG> [{}] {} > {}\t{}\tBit which would be set {:#16b}", c, counters[c], (measurements.len() / 2), counters[c] > (measurements.len() / 2), (1 << (num_bits - c))}
        if counters[c] > (measurements.len() / 2) {
            gamma_rate = gamma_rate | (1 << ((num_bits - 1) - c))
        }
    }
    println!{"Gamma rate: {:#16b} {:?}", gamma_rate, counters}
    let epsilon_rate = gamma_rate ^ (usize::MAX >> (usize::BITS as usize - num_bits));

    (gamma_rate, epsilon_rate)
}
