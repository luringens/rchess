#![allow(dead_code)]
use std::io;

mod chess;

fn main() {
    let clear = format!("{}[2J", 27 as char);
    let mut game = chess::Chess::new();
    
    println!("Welcome to rchess!");
    println!("Enter the field to move from and to in the format 'A2A4'.");
    println!("Type 'quit' to stop playing.");
    
    let mut running = true;
    while running {
        println!("{}", game);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        print!("{}", clear);
        match input.as_ref() {
            "quit" => running = false,
            _ => match game.perform_move(&input) {
                Err(a) => println!("{}", a),
                Ok(_) => println!("Done!")
            }
        }
    }
}
