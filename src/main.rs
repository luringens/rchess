#![allow(dead_code)]
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to rchess!");
    let mut game = Chess::new();

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

struct Chess {
    board: HashMap<String, Piece>
}

impl Chess {
    pub fn new() -> Chess {
        use Piece::*;
        let positions = [
            ("A1", WRook),   ("B1", WKnight), ("C1", WBishop), ("D1", WQueen),
            ("E1", WKing),   ("F1", WBishop), ("G1", WKnight), ("H1", WRook),
            ("A2", WPawn),   ("B2", WPawn),   ("C2", WPawn),   ("D2", WPawn),
            ("E2", WPawn),   ("F2", WPawn),   ("G2", WPawn),   ("H2", WPawn),
            ("A8", BRook),   ("B8", BKnight), ("C8", BBishop), ("D8", BQueen),
            ("E8", BKing),   ("F8", BBishop), ("G8", BKnight), ("H8", BRook),
            ("A7", BPawn),   ("B7", BPawn),   ("C7", BPawn),   ("D7", BPawn),
            ("E7", BPawn),   ("F7", BPawn),   ("G7", BPawn),   ("H7", BPawn)
        ];

        let mut newboard: HashMap<String, Piece> = HashMap::new();
        for set in positions.iter() {
            newboard.insert(String::from(set.0), set.1);
        }

        Chess { board: newboard }
    }

    pub fn perform_move(&mut self, move_to_perform: &str) -> Result<(), &'static str> {
        if !self.is_valid_move(move_to_perform) {
            return Result::Err("Invalid move");
        }

        let (from, to) = move_to_perform.split_at(2);
        let piece = self.board.get(from).unwrap().clone();
        self.board.insert(String::from(to), piece);
        self.board.remove(from);
        Ok(())
    }

    fn is_valid_move(&self, move_to_perform: &str) -> bool {
        // TODO: Check rules
        move_to_perform.len() == 4
    }
}

#[allow(unused_variables)]
impl std::fmt::Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        for x in 1..9 {
            print!("{} ", x);
            for letter in letters.iter() {
                print!("{} ", match self.board.get(&format!("{}{}", letter, x)) {
                    Some(t) => t.letter(),
                    None => "."
                });
            }
            println!();
        }
        println!("  A B C D E F G H");
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    BKing,
    BQueen,
    BRook,
    BBishop,
    BKnight,
    BPawn,
    WKing,
    WQueen,
    WRook,
    WBishop,
    WKnight,
    WPawn
}

impl Piece {
    fn letter(&self) -> &'static str {
        use Piece::*;
        match *self {
            BKing   => "♚",
            BQueen  => "♛",
            BRook   => "♜",
            BBishop => "♝",
            BKnight => "♞",
            BPawn   => "♟",
            WKing   => "♔",
            WQueen  => "♕",
            WRook   => "♖",
            WBishop => "♗",
            WKnight => "♘",
            WPawn   => "♙"
        }
    }
}
