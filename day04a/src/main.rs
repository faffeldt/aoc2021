use std::env;
mod bingo;
use crate::bingo::Bingo;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    // local_test_a();
    // solution_a();
    local_test_b();
    solution_b();
}

fn local_test_a() {
    let input_file = ".\\test_input.txt";
    let mut bingo = Bingo::from_file(input_file);
    println!("> Test input:\n{}", bingo);
    let (winner_board_idx, sum_of_all_unmarked_cells, last_drawn_number) = bingo.play();
    let result = sum_of_all_unmarked_cells * last_drawn_number;

    let expected_winning_board = 2;
    assert_eq! {winner_board_idx, expected_winning_board, "Winning Board Index mismatches: {} == {}", winner_board_idx, expected_winning_board}
    // let expected_marked_numbers = vec![14, 21, 17, 24, 4];
    let expected_sum_of_unmarked_numbers = 188;
    assert_eq! {sum_of_all_unmarked_cells, expected_sum_of_unmarked_numbers, "Sum of marked numbers mismatches: {} == {}", sum_of_all_unmarked_cells, expected_sum_of_unmarked_numbers}
    let expected_last_drawn_number = 24;
    assert_eq! {last_drawn_number, expected_last_drawn_number, "Last drawn number mismatches: {} == {}", last_drawn_number, expected_last_drawn_number}
    let expected_result = expected_sum_of_unmarked_numbers * expected_last_drawn_number; // 4512
    assert_eq! {result, expected_result, "Result does not match: {} == {}", result, expected_result};

    println! {">>> Test Result A:\n\twinning_board={}, sum_unmarked={}, last_drawn={}\n\t=> score={}", winner_board_idx, sum_of_all_unmarked_cells, last_drawn_number, result}
}

fn solution_a() {
    let input_file = ".\\solution_input.txt";
    let mut bingo = Bingo::from_file(input_file);
    println!("> Solution input:\n{}", bingo);
    let (winner_board_idx, sum_of_all_unmarked_cells, last_drawn_number) = bingo.play();
    let result = sum_of_all_unmarked_cells * last_drawn_number;

    println! {">>> Solution Result A:\n\twinning_board={}, sum_unmarked={}, last_drawn={}\n\t=> score={}", winner_board_idx, sum_of_all_unmarked_cells, last_drawn_number, result}
}

fn local_test_b() {
    let input_file = ".\\test_input.txt";
    let mut bingo = Bingo::from_file(input_file);
    println!("> Test input:\n{}", bingo);
    let (last_board_idx, sum_of_all_unmarked_cells, last_drawn_number) =
        bingo.play_for_last_board();
    let result = sum_of_all_unmarked_cells * last_drawn_number;

    let expected_last_board = 1;
    assert_eq! {last_board_idx, expected_last_board, "Last Board Index mismatches: {} == {}", last_board_idx, expected_last_board}
    // let expected_marked_numbers = vec![14, 21, 17, 24, 4];
    let expected_sum_of_unmarked_numbers = 148;
    assert_eq! {sum_of_all_unmarked_cells, expected_sum_of_unmarked_numbers, "Sum of marked numbers mismatches: {} == {}", sum_of_all_unmarked_cells, expected_sum_of_unmarked_numbers}
    let expected_last_drawn_number = 13;
    assert_eq! {last_drawn_number, expected_last_drawn_number, "Last drawn number mismatches: {} == {}", last_drawn_number, expected_last_drawn_number}
    let expected_result = expected_sum_of_unmarked_numbers * expected_last_drawn_number; // 1924
    assert_eq! {result, expected_result, "Result does not match: {} == {}", result, expected_result};

    println! {">>> Test Result B:\n\tlast_board={}, sum_unmarked={}, last_drawn={}\n\t=> score={}", last_board_idx, sum_of_all_unmarked_cells, last_drawn_number, result}
}

fn solution_b() {
    let input_file = ".\\solution_input.txt";
    let mut bingo = Bingo::from_file(input_file);
    println!("> Solution input:\n{}", bingo);
    let (last_board_idx, sum_of_all_unmarked_cells, last_drawn_number) =
        bingo.play_for_last_board();
    let result = sum_of_all_unmarked_cells * last_drawn_number;

    println! {">>> Solution Result B:\n\tlast_board={}, sum_unmarked={}, last_drawn={}\n\t=> score={}", last_board_idx, sum_of_all_unmarked_cells, last_drawn_number, result}
}
