use packet_decoder::Packet;
use std::env;
use std::fs;
mod packet_decoder;

fn main() {
    println!("CWD: {:?}", env::current_dir());
    solution_a();
    // solution_b();
}

fn solution_a() {
    let input_file = "./solution_input.txt";
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Error reading file {}", input_file));
    let packet = Packet::from_hex_str(&input);
    let packet_version_sum = packet.version_sum();
    println! {"Solution A: {:?}\n\t{:?}", packet_version_sum, packet};
}
