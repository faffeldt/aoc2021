use std::env;
use std::fs;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test();
    // solution();
}

fn local_test() {
    let input_file = ".\\test_input.txt";
    let bingo = Bingo::from_file(input_file);
    // println!("> Test input: {:?}", bingo);
    

    let result = 0;

    let expected_winning_board = 2;
    let expected_marked_numbers = vec![14, 21, 17, 24, 4];
    let expected_sum_of_unmarked_numbers = 188;
    let expected_last_drawn_number = 24;
    let expected_result = expected_sum_of_unmarked_numbers * expected_last_drawn_number;  // 4512
    println! {"> exp:\twb={}, marked_numbers={:?}, sum_unmarked={}, last_drawn={} => result={}", expected_winning_board, expected_marked_numbers, expected_sum_of_unmarked_numbers, expected_last_drawn_number, expected_result}
    println! {"> got: result={}", result};

    if result != expected_result {
        panic! {"> Solution does not match expected result"};
    }
}

// fn solution() {
//     let measurements: Vec<usize> = include_str!("..\\solution_input.txt")
//         .lines()
//         // .inspect(|i| println!{"{} -> {:#14b}", i, usize::from_str_radix(i, 2).unwrap()})
//         .map(|i| usize::from_str_radix(i, 2).unwrap())
//         .collect();
//     println!(
//         "> Solution input ({}): {:?}",
//         measurements.len(),
//         measurements
//     );
//     let num_bits = 12;
//     let oxygen_generator_rating = calculate_oxygen_generator_rating(&measurements, num_bits);
//     let co2_scrubber_rating = calculate_co2_scrubber_rating(&measurements, num_bits);
//     let result = oxygen_generator_rating * co2_scrubber_rating;
//     println! {"> Solution:\t{} ({:#14b})\t*\t{} ({:#14b})\t-> {}", oxygen_generator_rating, oxygen_generator_rating, co2_scrubber_rating, co2_scrubber_rating, result}
// }

struct Bingo {
    draw_numbers: Vec<usize>,
    // boards: Vec<Board>,
}

impl Bingo {
    fn new(draw_numbers: Vec<usize>) -> Bingo {
        Bingo {
            draw_numbers: draw_numbers,
            // boards: boards,
        }
    }

    fn from_file(filename: &str) -> Bingo {
        let NEWLINE = "\r\n";
        let DOUBLE_NEWLINE = "\r\n\r\n";
        let WIDTH = 5;
        let HEIGHT = 5;

        println!{">>> Starting to parse file {} <<<", filename}
        let raw_data = fs::read_to_string(filename).unwrap();
        let data_chunks: Vec<&str> = raw_data.split(DOUBLE_NEWLINE).collect();
        
        let raw_drawings = data_chunks[0];
        let drawings: Vec<&str> = raw_drawings.split(",").collect();
        println!{"Drawings ({}): {:?}", drawings.len(), drawings};

        let raw_boards = &data_chunks[1..];
        // let mut boards = vec![vec![0; WIDTH]; HEIGHT];
        let boards:Vec<Vec<usize>> = &raw_boards
            .into_iter()
            .map(|board| board.split(NEWLINE).collect())
            .map(|board:Vec<&str>| board.into_iter()
                .map(|row| row.split_whitespace().collect())
                .map(|row:Vec<&str>| row.into_iter()
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect()
                )
                .collect()
            )
            .collect();

        // for board in raw_boards {
        //     println!{"-------------"}
        //     let raw_rows: Vec<&str> = board.split(NEWLINE).collect();
        //     for row in raw_rows {
        //         let raw_row_elements: Vec<&str> = row.split_whitespace().collect();
        //         let row_elements: Vec<usize> = raw_row_elements
        //             .into_iter()
        //             .map(|e| e.parse::<usize>().unwrap())
        //             .collect();
        //         println!{"{:?}", row_elements};
        //     }
        //     println!{"-------------\n"}
        // }
        println!{"Boards ({}): {:?}", boards.len(), boards};

        let bingo = Bingo::new(vec![0]);
        bingo
    }
}

struct Board {
    width: usize,
    height: usize,
    fields: Vec<usize>,  // vec![vec![0; width]; height];
}