use std::env;
mod lanternfish;
use crate::lanternfish::Population;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a();
    // solution_a();
    // local_test_b();
    // solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let mut initial_population = Population::from_file(input_file);
    let max_days = 18;
    println!("A> Test input:\n\tmax_days={}\n\t{}", max_days, initial_population);

    let expected_states = vec![
        ("Initial state: 3,4,3,1,2",
            vec![3,4,3,1,2]),
        ("After  1 days: 2,3,2,0,1",
            vec![2,3,2,0,1]),
        ("After  2 days: 1,2,1,6,0,8",
            vec![1,2,1,6,0,8]),
        ("After  3 days: 0,1,0,5,6,7,8",
            vec![0,1,0,5,6,7,8]),
        ("After  4 days: 6,0,6,4,5,6,7,8,8",
            vec![6,0,6,4,5,6,7,8,8]),
        ("After  5 days: 5,6,5,3,4,5,6,7,7,8",
            vec![5,6,5,3,4,5,6,7,7,8]),
        ("After  6 days: 4,5,4,2,3,4,5,6,6,7",
            vec![4,5,4,2,3,4,5,6,6,7]),
        ("After  7 days: 3,4,3,1,2,3,4,5,5,6",
            vec![3,4,3,1,2,3,4,5,5,6]),
        ("After  8 days: 2,3,2,0,1,2,3,4,4,5",
            vec![2,3,2,0,1,2,3,4,4,5]),
        ("After  9 days: 1,2,1,6,0,1,2,3,3,4,8",
            vec![1,2,1,6,0,1,2,3,3,4,8]),
        ("After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8",
            vec![0,1,0,5,6,0,1,2,2,3,7,8]),
        ("After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8",
            vec![6,0,6,4,5,6,0,1,1,2,6,7,8,8,8]),
        ("After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8",
            vec![5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8]),
        ("After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8",
            vec![4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8]),
        ("After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8",
            vec![3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8]),
        ("After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7",
            vec![2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7]),
        ("After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8",
            vec![1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8]),
        ("After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8",
            vec![0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8]),
        ("After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8",
            vec![6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8]),
    ];
    for day in 0..max_days {
        // assert_eq!{initial_population.fishes, expected_states[day].1,
        //     "Expected fish population on day {} does not match:\n{:?}\n{:?}",
        //     day, initial_population.fishes, expected_states[day].1};
        // assert_eq!{format!{"{}", initial_population}, expected_states[day].0,
        //     "Expected fish population on day {} repr does not match:\n{}\n{}",
        //     day, initial_population, expected_states[day].0};
        assert_eq!{initial_population.size(), expected_states[day].1.len(),
            "Expected fish population size on day {} does not match:\n{:?}\n{:?}",
            day, initial_population.size(), expected_states[day].1.len()};
        assert_eq!{initial_population.sum(), expected_states[day].1.iter().fold(0, |sum, i| sum + i),
            "Expected fish population sum on day {} does not match:\n{:?}\n{:?}",
            day, initial_population.sum(), expected_states[day].1.iter().fold(0, |sum, i| sum + i)};
        initial_population.evolute();
        // initial_population = initial_population.evolute_concurrently();
    }

    println!{"A> Test final state:\n\t{}\n\tSize: {}", initial_population, initial_population.size()};
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let mut initial_population = Population::from_file(input_file);
    let max_days = 80;
    println!("A> Solution input:\n\tmax_days={}\n\t{}", max_days, initial_population);
    for _ in 0..max_days {
        println!{"Days: {}, Size: {}", initial_population.current_day, initial_population.size()};
        initial_population.evolute();
        // initial_population = initial_population.evolute_concurrently();
    }
    
    println!{"A> Solution final state:\n\tSize: {}", initial_population.size()};
}

// fn local_test_b() {
//     let input_file = ".\\test_input.txt";
//     let mut initial_population = Population::from_file(input_file);
//     let max_days = 256;
//     println!("B> Test input:\n\tmax_days={}\n\t{}", max_days, initial_population);

//     for day in 0..max_days {
//         println!{"Days: {}, Size: {}", initial_population.current_day, initial_population.size()};
//         // initial_population.evolute();
//         initial_population = initial_population.evolute_concurrently();
//     }

//     let expected_total_size: usize = 26984457539;
//     assert_eq!{initial_population.size(), expected_total_size, "Total size after {} days mismatches: {} == {}", max_days, initial_population.size(), expected_total_size};

//     println!{"B> Test final state:\n\t{}\n\tSize: {}", initial_population, initial_population.size()};
// }

// fn solution_b() {
//     let input_file = ".\\solution_input.txt";
//     let mut initial_population = Population::from_file(input_file);
//     let max_days = 256;
//     println!("B> Solution input:\n\tmax_days={}\n\t{}", max_days, initial_population);
//     for _ in 0..max_days {
//         println!{"Days: {}, Size: {}", initial_population.current_day, initial_population.size()};
//         initial_population.evolute();
//     }
    
//     println!{"B> Solution final state:\n\tSize: {}", initial_population.size()};
// }

