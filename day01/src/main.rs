use std::env;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a();
    solution_a();
    local_test_b();
    solution_b();
}

fn local_test_a() {
    let items: Vec<usize> = include_str!("..\\test_input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    println!("A> Test input ({}): {:?}", items.len(), items);
    let result = number_of_times_depth_increases(items);
    let expected_result = 7;
    println! {"A> expected={}, got={}", expected_result, result}
    if result != expected_result {
        panic! {"A> Solution does not match expected result"};
    }
}

fn solution_a() {
    let items: Vec<usize> = include_str!("..\\solution_input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    println!("A> Solution input ({})", items.len());
    let result = number_of_times_depth_increases(items);
    println!("A> Solution: {}", result);
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

fn local_test_b() {
    let items: Vec<usize> = include_str!("..\\test_input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    println!("B> Test input ({}): {:?}", items.len(), items);
    let result = number_of_times_depth_increases_3window(items);
    let expected_result = 5;
    println! {"B> expected={}, got={}", expected_result, result}
    if result != expected_result {
        panic! {"B> Solution does not match expected result"};
    }
}

fn solution_b() {
    let items: Vec<usize> = include_str!("..\\solution_input.txt")
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    println!("B> Solution input ({})", items.len());
    let result = number_of_times_depth_increases_3window(items);
    println!("B> Solution: {}", result);
}

fn number_of_times_depth_increases_3window(depths: Vec<usize>) -> usize {
    let result = depths
        .windows(3)
        .zip(depths.windows(3).skip(1))
        // .inspect(|(w1, w2)| println!{"w1: {:?} ({}), w2: {:?} ({})", w1, w1.iter().sum::<usize>(), w2, w2.iter().sum::<usize>()})
        .filter(|(w1, w2)| w1.iter().sum::<usize>() < w2.iter().sum::<usize>())
        .collect::<Vec<_>>();
    // println!("Result: length={} {:?}", result.len(), result);
    result.len()
}
