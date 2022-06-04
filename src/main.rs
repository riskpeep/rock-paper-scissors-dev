//
// Rock, Paper, Sissors
//
// A game by Robert Kerr

use std::io;

fn main() {
    println!("Hello, Lets play Rock, Paper, Sissors!");

    println!("Please input your move.");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read move");

    println!("You chose: {}", player_move);
}
