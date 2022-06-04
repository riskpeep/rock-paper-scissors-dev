//
// Rock, Paper, Sissors
//
// A game by Robert Kerr

use std::io;
use rand::Rng;

fn main() {
    println!("Hello, Lets play Rock, Paper, Sissors!");

    let comp_move = rand::thread_rng().gen_range(1..4);

    println!("My move is {}", comp_move);

    println!("Please input your move.");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read move");

    println!("You chose: {}", player_move);
}
