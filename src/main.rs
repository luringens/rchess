#![allow(dead_code)]
use std::io;

mod chess;

fn main() {
    let clear = format!("{}[2J", 27 as char);
    let mut game = chess::Chess::new();
    
    println!("Welcome to rchess!");
    println!("Enter the field to move from and to in the format 'A2A4'.");
    println!("Type 'quit' to stop playing.");
    
    loop {
        println!("{}", game);
        println!("It is {}'s turn.", game.next_player());
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        print!("{}", clear);
        match input.as_ref() {
            "quit" => return,
            _ => match game.perform_move(&input) {
                Err(a) => println!("{}", a),
                Ok(_) => println!("Done!")
            }
        }
    }
}
