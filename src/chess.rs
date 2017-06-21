use std;
use std::collections::HashMap;

pub struct Chess {
    board: HashMap<String, Piece>,
    next_player: Color
}

impl Chess {
    pub fn new() -> Chess {
        use self::PieceType::*;
        use self::Color::*;
        let positions = [
            ("A1", Piece::new(White, Rook)),   ("B1", Piece::new(White, Knight)),
            ("C1", Piece::new(White, Bishop)), ("D1", Piece::new(White, Queen)),
            ("E1", Piece::new(White, King)),   ("F1", Piece::new(White, Bishop)),
            ("G1", Piece::new(White, Knight)), ("H1", Piece::new(White, Rook)),
            ("A2", Piece::new(White, Pawn)),   ("B2", Piece::new(White, Pawn)),
            ("C2", Piece::new(White, Pawn)),   ("D2", Piece::new(White, Pawn)),
            ("E2", Piece::new(White, Pawn)),   ("F2", Piece::new(White, Pawn)),
            ("G2", Piece::new(White, Pawn)),   ("H2", Piece::new(White, Pawn)),
            ("A8", Piece::new(Black, Rook)),   ("B8", Piece::new(Black, Knight)),
            ("C8", Piece::new(Black, Bishop)), ("D8", Piece::new(Black, Queen)),
            ("E8", Piece::new(Black, King)),   ("F8", Piece::new(Black, Bishop)),
            ("G8", Piece::new(Black, Knight)), ("H8", Piece::new(Black, Rook)),
            ("A7", Piece::new(Black, Pawn)),   ("B7", Piece::new(Black, Pawn)),
            ("C7", Piece::new(Black, Pawn)),   ("D7", Piece::new(Black, Pawn)),
            ("E7", Piece::new(Black, Pawn)),   ("F7", Piece::new(Black, Pawn)),
            ("G7", Piece::new(Black, Pawn)),   ("H7", Piece::new(Black, Pawn)),
        ];

        let mut newboard: HashMap<String, Piece> = HashMap::new();
        for set in positions.iter() {
            newboard.insert(String::from(set.0), set.1);
        }

        Chess { board: newboard, next_player: White }
    }

    pub fn perform_move(&mut self, move_to_perform: &str) -> Result<(), &'static str> {
        if !self.is_valid_move(move_to_perform) {
            return Result::Err("Invalid move");
        }

        let (from, to) = move_to_perform.split_at(2);
        let piece = self.board.get(from).unwrap().clone();
        self.board.insert(String::from(to), piece);
        self.board.remove(from);
        self.next_player = match self.next_player {
            Color::White => Color::Black,
            Color::Black => Color::White
        };
        Ok(())
    }

    fn is_valid_move(&self, move_to_perform: &str) -> bool {
        // Check length
        if move_to_perform.len() != 4 { return false };

        // Check the move is withing bounds
        let valid_letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        let valid_numbers = ['1', '2', '3', '4', '5', '6', '7', '8'];
        let mut valid = true;
        for (position, character) in move_to_perform.char_indices() {
            valid = valid && match position % 2{
                0 => valid_letters.contains(&character),
                1 => valid_numbers.contains(&character),
                _ => panic!("This can't happen.")
            }
        }
        if !valid { return false; }

        // Get pieces
        let (from, to) = move_to_perform.split_at(2);
        let piece_to_move = self.board.get(from);
        let piece_to_kill = self.board.get(to);

        // Check that we are moving a piece, and it's of the correct color        
        match piece_to_move {
            Some(piece) => if piece.color != self.next_player { return false; },
            None => return false
        }        
        
        // Check that if we are landing on another piece, it is hostile
        match piece_to_kill {
            Some(piece) => if piece.color == self.next_player { return false; },
            None => { }
        }

        // TODO: Check that the piece is allowed to move like this
        // TODO: Check for mate rules

        true
    }

    pub fn next_player(&self) -> Color {
        self.next_player
    }
}

#[allow(unused_variables)]
impl std::fmt::Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // Print top row
        println!("  A B C D E F G H");

        let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
        for x in 1..9 {

            // Print the row number first in each row.
            print!("{} ", x);

            // Print each slot. Print . if empty, print piece otherwise.
            for letter in letters.iter() {
                print!("{} ", match self.board.get(&format!("{}{}", letter, x)) {
                    Some(t) => t.letter(),
                    None => "."
                });
            }

            // ...and print the row number last as well.
            println!("{} ", x);
        }

        // Print bottom row
        println!("  A B C D E F G H");
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub piecetype: PieceType
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece {
            color: color,
            piecetype: piece_type
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White    
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match *self {
            Color::White => "white",
            Color::Black => "black"
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl Piece {
    fn letter(&self) -> &'static str {
        use self::PieceType::*;
        use self::Color::*;
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
