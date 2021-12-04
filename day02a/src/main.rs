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
    let expected_position = Position::new(15, 10);
    let expected_result = 150;
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
    x: isize,
    y: isize,
}

impl Position {
    fn new(horizontal_pos: isize, depth: isize) -> Position {
        Position {
            x: horizontal_pos,
            y: depth,
        }
    }

    fn product(&self) -> isize {
        self.x * self.y
    }

    fn update(&mut self, dx: isize, dy: isize) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
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
            Command::Forward(amount) => position.update(1 * amount, 0),
            Command::Down(amount) => position.update(0, 1 * amount),
            Command::Up(amount) => position.update(0, -1 * amount),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Forward(amount) => write!(f, "forward {}", amount),
            Command::Down(amount) => write!(f, "down {}", amount),
            Command::Up(amount) => write!(f, "up {}", amount),
        }
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Command::Forward(amount) => write!(f, "forward {}", amount),
            Command::Down(amount) => write!(f, "down {}", amount),
            Command::Up(amount) => write!(f, "up {}", amount),
        }
    }
}

fn calculate_position(commands: Vec<Command>) -> Position {
    let mut position = Position::new(0, 0);
    for command in commands {
        command.apply(&mut position);
    }
    position
}
