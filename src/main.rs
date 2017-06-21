#![allow(dead_code)]
use std::io;

mod chess;

fn main() {
    println!("Welcome to rchess!");
    let mut game = chess::Chess::new();

    let mut running = true;
    while running {
        println!("{}", game);
        print!("{}[2J", 27 as char); // Clear screen

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        match input.as_ref() {
            "exit" => running = false,
            _ => match game.perform_move(&input) {
                Err(a) => println!("{}", a),
                Ok(_) => println!("Done!")
            }
        }
    }
}
