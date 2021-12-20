use std::env;
use std::fs;
mod octopuses;
use crate::octopuses::OctopusField;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a_small();
    local_test_a_big();
    solution_a();
    local_test_b();
    solution_b();
}

fn local_test_a_small() {
    let input_file = ".\\test_input_small.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut octopus_field = OctopusField::parse_input(input);
    println! {"TEST A Small> Input:\n{}", octopus_field};

    // Perform step 1
    octopus_field.step();
    let exp_octo_step_1 = vec![
        vec![3, 4, 5, 4, 3],
        vec![4, 0, 0, 0, 4],
        vec![5, 0, 0, 0, 5],
        vec![4, 0, 0, 0, 4],
        vec![3, 4, 5, 4, 3],
    ];
    let exp_flashes_1 = 9;
    let octo_step_1 = octopus_field.get_octopuses();
    let flashes_1 = octopus_field.get_flashes();
    assert_eq! {exp_octo_step_1, octo_step_1, "TEST A Small> Octopuses mismatch after step 1"};
    assert_eq! {exp_flashes_1, flashes_1, "TEST A Small> Flashes mismatch after step 1"};

    // Perform step 2
    octopus_field.step();
    let exp_octo_step_2 = vec![
        vec![4, 5, 6, 5, 4],
        vec![5, 1, 1, 1, 5],
        vec![6, 1, 1, 1, 6],
        vec![5, 1, 1, 1, 5],
        vec![4, 5, 6, 5, 4],
    ];
    let exp_flashes_2 = 9;
    let octo_step_2 = octopus_field.get_octopuses();
    let flashes_2 = octopus_field.get_flashes();
    assert_eq! {exp_octo_step_2, octo_step_2, "TEST A Small> Octopuses mismatch after step 2"};
    assert_eq! {exp_flashes_2, flashes_2, "TEST A Small> Flashes mismatch after step 2"};
}

fn local_test_a_big() {
    let input_file = ".\\test_input_big.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut octopus_field = OctopusField::parse_input(input);
    println! {"TEST A Big> Input:\n{}", octopus_field};

    // Perform step 1
    println! {"TEST A Big> Perform step 1"};
    octopus_field.step();
    let exp_octo_step_1 = vec![
        vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
        vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
        vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
        vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
        vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
        vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
        vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
        vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
        vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
        vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
    ];
    let exp_flashes_1 = 0;
    let octo_step_1 = octopus_field.get_octopuses();
    let flashes_1 = octopus_field.get_flashes();
    assert_eq! {exp_octo_step_1, octo_step_1, "TEST A Big> Octopuses mismatch after step 1"};
    assert_eq! {exp_flashes_1, flashes_1, "TEST A Big> Flashes mismatch after step 1"};
    println! {"TEST A Big> After step 1:\n{}", octopus_field};

    // Perform step 2
    println! {"TEST A Big> Perform step 2"};
    octopus_field.step();
    let exp_octo_step_2 = vec![
        vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
        vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
        vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
        vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
        vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
        vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
        vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
        vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
        vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
        vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
    ];
    let exp_flashes_2 = 35;
    let octo_step_2 = octopus_field.get_octopuses();
    let flashes_2 = octopus_field.get_flashes();
    assert_eq! {exp_octo_step_2, octo_step_2, "TEST A Big> Octopuses mismatch after step 2"};
    assert_eq! {exp_flashes_2, flashes_2, "TEST A Big> Flashes mismatch after step 2"};
    println! {"TEST A Big> After step 2:\n{}", octopus_field};

    // Test after 10 steps
    println! {"TEST A Big> Perform steps to 10"};
    let prev_steps = 2;
    octopus_field.step_x_times(10 - prev_steps);
    let exp_octo_step_10 = vec![
        vec![0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
        vec![0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
        vec![0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
        vec![0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
        vec![0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
        vec![0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
        vec![0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
        vec![5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
        vec![0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
        vec![0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
    ];
    let exp_flashes_10 = 204;
    let octo_step_10 = octopus_field.get_octopuses();
    let flashes_10 = octopus_field.get_flashes();
    assert_eq! {exp_octo_step_10, octo_step_10, "TEST A Big> Octopuses mismatch after step 10"};
    assert_eq! {exp_flashes_10, flashes_10, "TEST A Big> Flashes mismatch after step 10"};
    println! {"TEST A Big> After step 10:\n{}", octopus_field};

    // Test after 100 steps
    println! {"TEST A Big> Perform steps to 100"};
    let prev_steps = 10;
    octopus_field.step_x_times(100 - prev_steps);
    let exp_octo_step_100 = vec![
        vec![0, 3, 9, 7, 6, 6, 6, 8, 6, 6],
        vec![0, 7, 4, 9, 7, 6, 6, 9, 1, 8],
        vec![0, 0, 5, 3, 9, 7, 6, 9, 3, 3],
        vec![0, 0, 0, 4, 2, 9, 7, 8, 2, 2],
        vec![0, 0, 0, 4, 2, 2, 9, 8, 9, 2],
        vec![0, 0, 5, 3, 2, 2, 2, 8, 7, 7],
        vec![0, 5, 3, 2, 2, 2, 2, 9, 6, 6],
        vec![9, 3, 2, 2, 2, 2, 8, 9, 6, 6],
        vec![7, 9, 2, 2, 2, 8, 6, 8, 6, 6],
        vec![6, 7, 8, 9, 9, 9, 8, 7, 6, 6],
    ];
    let exp_flashes_100 = 1656;
    let octo_step_100 = octopus_field.get_octopuses();
    let flashes_100 = octopus_field.get_flashes();
    assert_eq! {exp_octo_step_100, octo_step_100, "TEST A Big> Octopuses mismatch after step 100"};
    assert_eq! {exp_flashes_100, flashes_100, "TEST A Big> Flashes mismatch after step 100"};
    // println!{"TEST A Big> After step 100:\n{}", octopus_field};

    println! {"TEST A Big> Final octopus field:\n{}", octopus_field};
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut octopus_field = OctopusField::parse_input(input);
    println! {"Solution A> Input:\n{}", octopus_field};
    octopus_field.step_x_times(100);
    println! {"SOLUTION A> Result: {}", octopus_field};
}

fn local_test_b() {
    let input_file = ".\\test_input_big.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut octopus_field = OctopusField::parse_input(input.clone());
    println! {"TEST B Big> Input:\n{}", octopus_field};

    // Perform steps to 193
    println! {"TEST B Big> Perform steps to 193"};
    octopus_field.step_x_times(193);
    let exp_octo_step_193 = vec![
        vec![5, 8, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![8, 8, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
        vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7],
    ];
    let octo_step_193 = octopus_field.get_octopuses();
    assert_eq! {exp_octo_step_193, octo_step_193, "TEST B Big> Octopuses mismatch after step 193"};
    println! {"TEST B Big> After step 193:\n{}", octopus_field};

    // Perform step 194
    println! {"TEST B Big> Perform step 194"};
    octopus_field.step();
    let exp_octo_step_194 = vec![
        vec![6, 9, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![9, 9, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
        vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
    ];
    let octo_step_194 = octopus_field.get_octopuses();
    assert_eq! {exp_octo_step_194, octo_step_194, "TEST B Big> Octopuses mismatch after step 194"};
    println! {"TEST B Big> After step 194:\n{}", octopus_field};

    // Perform step 195 which syncs
    println! {"TEST B Big> Perform steps to 195"};
    octopus_field.step();
    let exp_octo_step_195 = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let octo_step_195 = octopus_field.get_octopuses();
    assert_eq! {exp_octo_step_195, octo_step_195, "TEST B Big> Octopuses mismatch after step 195"};
    println! {"TEST B Big> After step 195:\n{}", octopus_field};

    // Test step until all flashed at once
    println! {"TEST B Big> Perform steps until all flashed at once"};
    octopus_field = OctopusField::parse_input(input);
    octopus_field.step_until_all_flash();
    let exp_octo_step_all_flash = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let exp_step_all_flashed = 195;
    let octo_step_all_flash = octopus_field.get_octopuses();
    let step_all_flashed = octopus_field.get_step();
    assert_eq! {exp_octo_step_all_flash, octo_step_all_flash, "TEST B Big> Octopuses mismatch in step all flashed"};
    assert_eq! {exp_step_all_flashed, step_all_flashed, "TEST B Big> First step in which all flashed mismatches"};

    println! {"TEST B Big> Final octopus field:\n{}", octopus_field};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut octopus_field = OctopusField::parse_input(input);
    println! {"SOLUTION B> Input:\n{}", octopus_field};

    octopus_field.step_until_all_flash();

    println! {"SOLUTION B> Final octopus field:\n{}", octopus_field};
}
