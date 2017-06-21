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
        use PieceType::*;
        use Color::*;
        let positions = [
            ("A1", Piece{piecetype: Rook,   color: White}), ("B1", Piece{piecetype: Knight, color: White}),
            ("C1", Piece{piecetype: Bishop, color: White}), ("D1", Piece{piecetype: King,   color: White}),
            ("E1", Piece{piecetype: Queen,  color: White}), ("F1", Piece{piecetype: Bishop, color: White}),
            ("G1", Piece{piecetype: Knight, color: White}), ("H1", Piece{piecetype: Rook,   color: White}),
            ("A2", Piece{piecetype: Pawn,   color: White}), ("B2", Piece{piecetype: Pawn,   color: White}),
            ("C2", Piece{piecetype: Pawn,   color: White}), ("D2", Piece{piecetype: Pawn,   color: White}),
            ("E2", Piece{piecetype: Pawn,   color: White}), ("F2", Piece{piecetype: Pawn,   color: White}),
            ("G2", Piece{piecetype: Pawn,   color: White}), ("H2", Piece{piecetype: Pawn,   color: White}),
            ("A8", Piece{piecetype: Rook,   color: Black}), ("B8", Piece{piecetype: Knight, color: Black}),
            ("C8", Piece{piecetype: Bishop, color: Black}), ("D8", Piece{piecetype: King,   color: Black}),
            ("E8", Piece{piecetype: Queen,  color: Black}), ("F8", Piece{piecetype: Bishop, color: Black}),
            ("G8", Piece{piecetype: Knight, color: Black}), ("H8", Piece{piecetype: Rook,   color: Black}),
            ("A7", Piece{piecetype: Pawn,   color: Black}), ("B7", Piece{piecetype: Pawn,   color: Black}),
            ("C7", Piece{piecetype: Pawn,   color: Black}), ("D7", Piece{piecetype: Pawn,   color: Black}),
            ("E7", Piece{piecetype: Pawn,   color: Black}), ("F7", Piece{piecetype: Pawn,   color: Black}),
            ("G7", Piece{piecetype: Pawn,   color: Black}), ("H7", Piece{piecetype: Pawn,   color: Black})
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
struct Piece {
    pub color: Color,
    pub piecetype: PieceType
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White    
}

#[derive(Debug, Clone, Copy)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl Piece {
    fn letter(&self) -> &'static str {
        use PieceType::*;
        use Color::*;
        match (self.piecetype, self.color) {
            (King,   White) => "♚",
            (Queen,  White) => "♛",
            (Bishop, White) => "♝",
            (Knight, White) => "♞",
            (Rook,   White) => "♜",
            (Pawn,   White) => "♟",
            (King,   Black) => "♔",
            (Queen,  Black) => "♕",
            (Bishop, Black) => "♗",
            (Knight, Black) => "♘",
            (Rook,   Black) => "♖",
            (Pawn,   Black) => "♙"
        }
    }
}
