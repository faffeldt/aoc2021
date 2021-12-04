use std::env;
use std::fmt;

pub fn main() {
    println!("CWD: {:?}", env::current_dir());
    local_test();
    solution();
}

fn local_test() {
    let items: Vec<Command> = include_str!("..\\test_input.txt")
        .lines()
        .map(|i| i.split(" ").collect())
        .map(|i: Vec<_>| Command::new(i[0], i[1].parse::<isize>().unwrap()))
        .collect();
    println!("> Test input ({}): {:?}", items.len(), items);
    let position = calculate_position(items);
    let result = position.product();
    let expected_position = Position::new(15, 60, 10);
    let expected_result = 900;
    println! {"> expected_result={}, got={}; expected_pos={}, got={}", expected_result, result, expected_position, position}
    if result != expected_result {
        panic! {"> Solution does not match expected result"};
    }
}

fn solution() {
    let items: Vec<Command> = include_str!("..\\solution_input.txt")
        .lines()
        .map(|i| i.split(" ").collect())
        .map(|i: Vec<_>| Command::new(i[0], i[1].parse::<isize>().unwrap()))
        .collect();
    println!("> Solution input ({}): {:?}", items.len(), items);
    let position = calculate_position(items);
    let result = position.product();
    println!("> Solution result: {}", result);
}

struct Position {
    horizontal_pos: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    fn new(horizontal_pos: isize, depth: isize, aim: isize) -> Position {
        Position {
            horizontal_pos: horizontal_pos,
            depth: depth,
            aim: aim,
        }
    }

    fn product(&self) -> isize {
        self.horizontal_pos * self.depth
    }

    fn up(&mut self, x: isize) {
        self.aim = self.aim - x;
    }

    fn down(&mut self, x: isize) {
        self.aim = self.aim + x;
    }

    fn forward(&mut self, x: isize) {
        self.horizontal_pos = self.horizontal_pos + x;
        self.depth = self.depth + self.aim * x;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x: {}, y: {}, aim: {}",
            self.horizontal_pos, self.depth, self.aim
        )
    }
}

enum Command {
    Down(isize),
    Up(isize),
    Forward(isize),
}

impl Command {
    fn new(command: &str, amount: isize) -> Command {
        match command {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => panic! {"Cannot create command from: {} {}", command, amount},
        }
    }

    fn apply(&self, position: &mut Position) {
        match *self {
            Command::Down(amount) => position.down(amount),
            Command::Up(amount) => position.up(amount),
            Command::Forward(amount) => position.forward(amount),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Down(amount) => write!(f, "down {}", amount),
            Command::Up(amount) => write!(f, "up {}", amount),
            Command::Forward(amount) => write!(f, "forward {}", amount),
        }
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Down(amount) => write!(f, "down {}", amount),
            Command::Up(amount) => write!(f, "up {}", amount),
            Command::Forward(amount) => write!(f, "forward {}", amount),
        }
    }
}

fn calculate_position(commands: Vec<Command>) -> Position {
    let mut position = Position::new(0, 0, 0);
    for command in commands {
        command.apply(&mut position);
    }
    position
}
