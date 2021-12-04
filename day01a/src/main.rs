use std::env;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test();
    solution();
}

fn local_test() {
    let items: Vec<usize> = include_str!("..\\test_input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    println!("Test input ({}): {:?}", items.len(), items);
    let result = number_of_times_depth_increases(items);
    let expected_result = 7;
    println! {"> expected={}, got={}", expected_result, result}
    if result != expected_result {
        panic! {"Solution does not match expected result"};
    }
}

fn solution() {
    let items: Vec<usize> = include_str!("..\\solution_input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    println!("Solution input ({})", items.len());
    let result = number_of_times_depth_increases(items);
    println!("> Solution: {}", result);
}

fn number_of_times_depth_increases(depths: Vec<usize>) -> usize {
    let result = depths
        .windows(2)
        // .inspect(|w| println!{"a: {}, b: {}, increased? {}", w[0], w[1], w[0] < w[1]})
        .filter(|w| w[0] < w[1])
        .collect::<Vec<_>>();
    // println!("Result: length={} {:?}", result.len(), result);
    result.len()
}
