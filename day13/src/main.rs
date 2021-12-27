use std::env;
use std::fs;
mod origami;
use crate::origami::Origami;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test_a_small();
    solution_a();
    solution_b();
}

fn local_test_a_small() {
    let input_file = ".\\test_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let mut origami = Origami::parse_input(&input);
    println! {"TEST A> Input:\n{}", origami};

    println! {"TEST A> Board draw:"};
    origami.draw();

    origami.fold();
    origami.draw();
    let expected_points_fold_0 = 17;
    let points_fold_0 = origami.num_of_points();
    assert_eq! {expected_points_fold_0, points_fold_0, "Points after first fold mismatch"};

    origami.execute_all_folds();
    // origami.draw();
    let expected_points_fold_all = 16;
    let points_fold_all = origami.num_of_points();
    assert_eq! {expected_points_fold_all, points_fold_all, "Points after all folds mismatch"};
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    let mut origami = Origami::parse_input(&input);
    println! {"Solution A> State before any fold:\n\t{}", origami};
    origami.fold();
    // origami.draw();
    println! {"Solution A> State after 1 fold:\n\t{}", origami};
    let points_fold_0 = origami.num_of_points();
    println! {"Solution A> Number of points after 1 fold: {}", points_fold_0};
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));

    let mut origami = Origami::parse_input(&input);
    println! {"Solution B> State before any fold:\n\t{}", origami};
    origami.execute_all_folds();
    origami.draw();
    println! {"Solution B> State after 1 fold:\n\t{}", origami};
}
