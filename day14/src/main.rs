use std::env;
use std::fs;
mod polymerization;
use crate::polymerization::PolymerTemplate;
use std::collections::BTreeMap;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a();
    solution_a();
    local_test_b();
    solution_b();
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
        // "NBBBCNCCNBBNBNBBCHBHHBCHB",
        // "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
    ];

    let expected_step_states_2 = vec![
        BTreeMap::from([(('N', 'N'), 1), (('N', 'C'), 1), (('C', 'B'), 1)]),
        BTreeMap::from([
            (('N', 'C'), 1),
            (('C', 'N'), 1),
            (('N', 'B'), 1),
            (('B', 'C'), 1),
            (('C', 'H'), 1),
            (('H', 'B'), 1),
        ]),
        BTreeMap::from([
            (('N', 'B'), 2),
            (('B', 'C'), 2),
            (('C', 'C'), 1),
            (('C', 'N'), 1),
            (('B', 'B'), 2),
            (('C', 'B'), 2),
            (('B', 'H'), 1),
            (('H', 'C'), 1),
        ]),
    ];

    for (i, step) in expected_step_states.iter().enumerate() {
        // Prepare expected step data for comparison
        let mut count_nums: BTreeMap<char, usize> = BTreeMap::new();
        let mut pair_nums: BTreeMap<(char, char), usize> = BTreeMap::new();
        let step_chars = step.chars().collect::<Vec<char>>();
        step_chars
            .iter()
            .for_each(|c| *count_nums.entry(*c).or_insert(0) += 1);
        step_chars
            .windows(2)
            .for_each(|window| *pair_nums.entry((window[0], window[1])).or_insert(0) += 1);

        // Verify that the preparation worked
        assert_eq! {pair_nums, expected_step_states_2[i], "Pair nums mismatch in step {}", i};

        // Verify that the expected states match actual state
        assert_eq! {count_nums, polymer_template.counts, "Polymer states in step {} mismatch", i};

        // Evolute the polymer for next test
        polymer_template.build_polymer();
        println! {"DEBUG> Polymer after step {}:\n  counts={:?}\n  pairs={:?}", i, polymer_template.counts, polymer_template.pairs};
    }

    let expected_result = 1588;
    polymer_template.build_polymer_n_times(10 - expected_step_states.len());
    let result = polymer_template.score_of_last();
    assert_eq! {expected_result, result, "TEST A> Result of last polymer score mismatches"};
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
    assert_eq! {expected_result, result, "TEST B> Result of last polymer score mismatches"};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    let mut polymer_template = PolymerTemplate::parse_input(&input);
    println! {"SOLUTION B> Input:\n{}", polymer_template};
    polymer_template.build_polymer_n_times(40);
    println! {"SOLUTION B> After 40 steps:\n{}", polymer_template};

    let result = polymer_template.score_of_last();
    println! {"Solution A> Score of last polymer {}", result};
}
