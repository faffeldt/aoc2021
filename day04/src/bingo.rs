use std::fmt;
use std::fs;

pub struct Bingo {
    draw_numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new(draw_numbers: Vec<usize>, boards: Vec<Board>) -> Bingo {
        Bingo {
            draw_numbers: draw_numbers,
            boards: boards,
        }
    }

    pub fn from_file(filename: &str) -> Bingo {
        let newline = "\r\n";
        let double_newline = "\r\n\r\n";
        let width = 5;
        let height = 5;

        println! {">>> Starting to parse file {} <<<", filename}
        let raw_data = fs::read_to_string(filename).unwrap();
        let data_chunks: Vec<&str> = raw_data.split(double_newline).collect();

        let raw_draw_numbers = data_chunks[0];
        let draw_numbers: Vec<usize> = raw_draw_numbers
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let raw_boards = &data_chunks[1..];
        let mut boards = Vec::new();
        for board in raw_boards {
            let mut cells: Vec<Vec<Cell>> = Vec::new();
            let raw_rows: Vec<&str> = board.split(newline).collect();
            for row in raw_rows {
                // Split row values by whitespace and create Vec of Cells
                let row_elements: Vec<Cell> = row
                    .split_whitespace()
                    .map(|e| e.parse::<usize>().unwrap())
                    .map(|e| Cell::new(e))
                    .collect();
                cells.push(row_elements);
            }
            boards.push(Board::new(width, height, cells));
        }

        Bingo::new(draw_numbers, boards)
    }

    pub fn play(&mut self) -> (usize, usize, usize) {
        let mut board_has_won = false;
        let mut last_drawn_number = 0;
        for draw_number in &self.draw_numbers {
            for board in &mut self.boards {
                board.check_number(*draw_number);
                board_has_won |= board.has_won();
                if board_has_won {
                    break;
                }
            }
            if board_has_won {
                last_drawn_number = *draw_number;
                break;
            }
            println! {"lastDrew {}, Result:\n{}", draw_number, self}
        }

        let mut winner_board_idx = 0;
        for i in 0..self.boards.len() {
            if self.boards[i].has_won() {
                winner_board_idx = i;
                break;
            }
        }

        println! {"last_drawn_number {}\nwinner:\n{}\nResult:\n{}", last_drawn_number, self.boards[winner_board_idx], self}
        (
            winner_board_idx,
            self.boards[winner_board_idx].sum_of_all_unmarked_cells(),
            last_drawn_number,
        )
    }

    pub fn play_for_last_board(&mut self) -> (usize, usize, usize) {
        let mut last_drawn_number = 0;
        let mut last_board_idx = 0;
        for draw_number in &self.draw_numbers {
            let mut all_boards_won = true;
            for i in 0..self.boards.len() {
                self.boards[i].check_number(*draw_number);
                let board_status = self.boards[i].has_won();
                all_boards_won &= board_status;
                // Remember the board that is not solved yet
                if !board_status {
                    last_board_idx = i;
                }
            }
            if all_boards_won {
                last_drawn_number = *draw_number;
                break;
            }
            println! {"lastDrew {}, Result:\n{}", draw_number, self}
        }

        println! {"last_drawn_number {}\nResult:\n{}\n\nLast board:\n{}", last_drawn_number, self, self.boards[last_board_idx]}
        (
            last_board_idx,
            self.boards[last_board_idx].sum_of_all_unmarked_cells(),
            last_drawn_number,
        )
    }
}

impl fmt::Display for Bingo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "Draw numbers ({}): {:?}\n", self.draw_numbers.len(), self.draw_numbers}
            .unwrap();
        for board in &self.boards {
            let board_seperator = "====================\n";
            write! {f, "{}{}{}", board_seperator, board, board_seperator}.unwrap();
        }
        write! {f, ""}
    }
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(width: usize, height: usize, cells: Vec<Vec<Cell>>) -> Board {
        Board {
            width: width,
            height: height,
            cells: cells,
        }
    }

    fn check_number(&mut self, number: usize) {
        for row in &mut self.cells {
            for cell in row {
                if cell.value == number {
                    cell.mark();
                    break;
                }
            }
        }
    }

    fn sum_of_all_unmarked_cells(&self) -> usize {
        self.cells.iter().fold(0, |mut sum, row| {
            sum += row.iter().filter(|c| !c.marked).fold(0, |mut sum, c| {
                sum += c.value;
                sum
            });
            sum
        })
    }

    fn has_won(&self) -> bool {
        let mut won = false;

        // Check columns and rows
        for j in 0..self.height {
            let mut row_check = true;
            let mut col_check = true;
            for i in 0..self.width {
                row_check &= self.cells[j][i].marked;
                col_check &= self.cells[i][j].marked;
            }
            if row_check || col_check {
                won = true;
                break;
            }
        }

        won
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "width: {}, height: {}, sum_of_unmarked: {}\n", self.width, self.height, self.sum_of_all_unmarked_cells()}.unwrap();
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

struct Cell {
    value: usize,
    marked: bool,
}

impl Cell {
    fn new(value: usize) -> Cell {
        Cell {
            value: value,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn representation(&self) -> String {
        if self.marked {
            format! {"-{:02}-", self.value}
        } else {
            format! {" {:02} ", self.value}
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "{}", self.representation()}
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write! {f, "{}", self.representation()}
    }
}
