use std::env;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test();
    solution();
}

fn local_test() {
    let measurements: Vec<usize> = include_str!("..\\test_input.txt")
        .lines()
        // .inspect(|i| println!{"{} -> {:#14b}", i, usize::from_str_radix(i, 2).unwrap()})
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .collect();
    println!("> Test input ({}): {:?}", measurements.len(), measurements);
    let num_bits = 5;
    let oxygen_generator_rating = calculate_oxygen_generator_rating(&measurements, num_bits);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&measurements, num_bits);
    let result = oxygen_generator_rating * co2_scrubber_rating;

    let expected_oxygen_generator_rating = 0b10111; // 23
    let expected_co2_scrubber_rating = 0b01010; // 10
    let expected_result = 230;
    println! {"> exp:\t{:#14b},\t{:#14b},\t{}\n  got:\t{:#14b},\t{:#14b},\t{}", expected_oxygen_generator_rating, expected_co2_scrubber_rating, expected_result, oxygen_generator_rating, co2_scrubber_rating, result}
    if result != expected_result {
        panic! {"> Solution does not match expected result"};
    }
}

fn solution() {
    let measurements: Vec<usize> = include_str!("..\\solution_input.txt")
        .lines()
        // .inspect(|i| println!{"{} -> {:#14b}", i, usize::from_str_radix(i, 2).unwrap()})
        .map(|i| usize::from_str_radix(i, 2).unwrap())
        .collect();
    println!(
        "> Solution input ({}): {:?}",
        measurements.len(),
        measurements
    );
    let num_bits = 12;
    let oxygen_generator_rating = calculate_oxygen_generator_rating(&measurements, num_bits);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&measurements, num_bits);
    let result = oxygen_generator_rating * co2_scrubber_rating;
    println! {"> Solution:\t{} ({:#14b})\t*\t{} ({:#14b})\t-> {}", oxygen_generator_rating, oxygen_generator_rating, co2_scrubber_rating, co2_scrubber_rating, result}
}

fn count_bits_at_idx(measurements: &Vec<usize>, num_bits: usize, target_bit: usize) -> usize {
    let mut counter: usize = 0;
    // Count the 1 in ith bit position of each measurement
    for measurement in measurements {
        let bit_mask = 1 << (num_bits - 1) - target_bit;
        // println!{"DEBUG> [{:2}] m={:#014b} m&bm={:#014b} -> {}", target_bit, measurement, measurement & bit_mask, (measurement & bit_mask != 0)}
        // println!{"              {:#014b}", bit_mask}
        counter += (measurement & bit_mask != 0) as usize;
    }
    counter
}

fn calculate_oxygen_generator_rating(measurements: &Vec<usize>, num_bits: usize) -> usize {
    println! {">>> Starting O2 Generator rating with {} measurements <<<", measurements.len()}
    let mut filtered_measurements = measurements.to_vec();
    for target_bit in 0..num_bits {
        println! {"\n>>> O2 Iteration {}/{}: <<<", target_bit, num_bits-1}
        let counter = count_bits_at_idx(&filtered_measurements, num_bits, target_bit);
        // Get if 0 or 1 are the most significant bit this run
        let orig_filtered_measurements_len = filtered_measurements.len();
        let significant_bit =
            counter >= (orig_filtered_measurements_len / 2) + orig_filtered_measurements_len % 2;
        println! {"target_bit={}: {} >= {} -> {}", target_bit, counter, (orig_filtered_measurements_len / 2) + orig_filtered_measurements_len % 2, significant_bit}
        // Prepare the bitmask to grab only the targeted bit in this run
        let bit_mask = 1 << (num_bits - 1) - target_bit;
        // Filter out all measurements whose target_bit equals most significant bit
        filtered_measurements = filtered_measurements
            .into_iter()
            .inspect(|measurement| println!{"{:#014b} -> {:#014b} -> {} -> {}", measurement, (measurement & bit_mask), ((measurement & bit_mask) != 0), ((measurement & bit_mask) != 0) == significant_bit})
            .filter(|measurement| ((measurement & bit_mask) != 0) == significant_bit)
            .collect::<Vec<usize>>();

        println! {"filtered_measurements: target_bit={} len_orig={} len_filtered={} counter={} significant_bit={}\n{:?}", target_bit, orig_filtered_measurements_len, filtered_measurements.len(), counter, significant_bit as usize, filtered_measurements};
        if filtered_measurements.len() == 1 {
            break;
        }
    }
    if filtered_measurements.len() != 1 {
        panic! {"Something went wrong filtering for most significant bits: {:?}", filtered_measurements}
    }
    filtered_measurements[0]
}

fn calculate_co2_scrubber_rating(measurements: &Vec<usize>, num_bits: usize) -> usize {
    println! {">>> Starting CO2 Scrubber rating with {} measurements <<<", measurements.len()}
    let mut filtered_measurements = measurements.to_vec();
    for target_bit in 0..num_bits {
        println! {"\n>>> CO2 Iteration {}/{}: <<<", target_bit, num_bits-1}
        let counter = count_bits_at_idx(&filtered_measurements, num_bits, target_bit);
        // Get if 0 or 1 are the most significant bit this run
        let orig_filtered_measurements_len = filtered_measurements.len();
        let significant_bit =
            counter >= (orig_filtered_measurements_len / 2) + orig_filtered_measurements_len % 2;
        println! {"target_bit={}: {} >= {} -> {}", target_bit, counter, (orig_filtered_measurements_len / 2) + orig_filtered_measurements_len % 2, significant_bit}
        // Prepare the bitmask to grab only the targeted bit in this run
        let bit_mask = 1 << (num_bits - 1) - target_bit;
        // Filter out all measurements whose target_bit does not equal most significant bit
        filtered_measurements = filtered_measurements
            .into_iter()
            .inspect(|measurement| println!{"{:#014b} -> {:#014b} -> {} => {}", measurement, (measurement & bit_mask), ((measurement & bit_mask) != 0) as usize, ((measurement & bit_mask) != 0) != significant_bit})
            .filter(|measurement| ((measurement & bit_mask) != 0) != significant_bit)
            .collect::<Vec<usize>>();

        println! {"filtered_measurements: target_bit={} len_orig={} len_filtered={} counter={} significant_bit={}\n{:?}", target_bit, orig_filtered_measurements_len, filtered_measurements.len(), counter, !significant_bit as usize, filtered_measurements};
        if filtered_measurements.len() == 1 {
            break;
        }
    }
    if filtered_measurements.len() != 1 {
        panic! {"Something went wrong filtering for least significant bits: {:?}", filtered_measurements}
    }
    filtered_measurements[0]
}
