use std::env;
use std::fs;
mod chiton;
// use crate::chiton::RiskFactorMap;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    solution_a();
    // local_test_b();
    // solution_b();
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    // let mut polymer_template = Caves::parse_input(&input);
    // println! {"SOLUTION A> Input:\n{}", polymer_template};
    // polymer_template.build_polymer_n_times(10);
    // println! {"SOLUTION A> After 10 steps:\n{}", polymer_template};

    // let result = polymer_template.score_of_last();
    // println! {"Solution A> Score of last polymer {}", result};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    //     let mut polymer_template = PolymerTemplate::parse_input(&input);
    //     println! {"SOLUTION B> Input:\n{}", polymer_template};
    //     polymer_template.build_polymer_n_times(40);
    //     println! {"SOLUTION B> After 40 steps:\n{}", polymer_template};

    //     let result = polymer_template.score_of_last();
    //     println! {"Solution A> Score of last polymer {}", result};
}
