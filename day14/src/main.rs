use std::env;
use std::fs;
mod polymerization;
use crate::polymerization::PolymerTemplate;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    // local_test_a();
    // solution_a();
    local_test_b();
    // solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut polymer_template = PolymerTemplate::parse_input(&input);
    println! {"TEST A> Input:\n{}", polymer_template};
    // polymer_template.build_polymer_n_times(10);
    // println! {"TEST A> After 10 steps:\n{}", polymer_template};

    let expected_step_states = vec![
        "NNCB",
        "NCNBCHB",
        "NBCCNBBBCBHCB",
        "NBBBCNCCNBBNBNBBCHBHHBCHB",
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
    ];
    for i in 0..expected_step_states.len() {
        assert_eq!{expected_step_states[i].chars().collect::<Vec<char>>(), polymer_template.states[0], "Polymer states in step {} mismatch", i};
        polymer_template.build_polymer();
    }

    let expected_result = 1588;
    polymer_template.build_polymer_n_times(10 - expected_step_states.len());
    let result = polymer_template.score_of_last();
    assert_eq!{expected_result, result, "TEST A> Result of last polymer score mismatches"};
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    let mut polymer_template = PolymerTemplate::parse_input(&input);
    println! {"SOLUTION A> Input:\n{}", polymer_template};
    polymer_template.build_polymer_n_times(10);
    println! {"SOLUTION A> After 10 steps:\n{}", polymer_template};
    
    let result = polymer_template.score_of_last();
    println! {"Solution A> Score of last polymer {}", result};
}

fn local_test_b() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut polymer_template = PolymerTemplate::parse_input(&input);
    println! {"TEST B> Input:\n{}", polymer_template};
    polymer_template.build_polymer_n_times(40);
    println! {"TEST B> After 40 steps:\n{}", polymer_template};

    let expected_result = 2188189693529;
    let result = polymer_template.score_of_last();
    assert_eq!{expected_result, result, "TEST B> Result of last polymer score mismatches"};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    let mut polymer_template = PolymerTemplate::parse_input(&input);
    println! {"SOLUTION A> Input:\n{}", polymer_template};
    polymer_template.build_polymer_n_times(40);
    // println! {"SOLUTION A> After 40 steps:\n{}", polymer_template};
    
    let result = polymer_template.score_of_last();
    println! {"Solution A> Score of last polymer {}", result};
}
