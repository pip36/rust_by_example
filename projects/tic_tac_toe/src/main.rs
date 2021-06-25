mod game;
use std::io;

fn main() {
    println!("Tic Tac Toe");
    println!("1) Play vs Human");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");
}
