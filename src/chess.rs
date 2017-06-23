use std;
use std::collections::HashMap;

pub struct Chess {
    board: HashMap<(u32, u32), Piece>,
    next_player: Color
}

impl Chess {
    pub fn new() -> Chess {
        use self::PieceType::*;
        use self::Color::*;
        let positions = [
            ((2, 1), Piece::new(White, Rook)),   ((2, 1), Piece::new(White, Knight)),
            ((4, 1), Piece::new(White, Bishop)), ((4, 1), Piece::new(White, Queen)),
            ((6, 1), Piece::new(White, King)),   ((6, 1), Piece::new(White, Bishop)),
            ((8, 1), Piece::new(White, Knight)), ((8, 1), Piece::new(White, Rook)),
            ((2, 2), Piece::new(White, Pawn)),   ((2, 2), Piece::new(White, Pawn)),
            ((4, 2), Piece::new(White, Pawn)),   ((4, 2), Piece::new(White, Pawn)),
            ((6, 2), Piece::new(White, Pawn)),   ((6, 2), Piece::new(White, Pawn)),
            ((8, 2), Piece::new(White, Pawn)),   ((8, 2), Piece::new(White, Pawn)),
            ((2, 8), Piece::new(Black, Rook)),   ((2, 8), Piece::new(Black, Knight)),
            ((4, 8), Piece::new(Black, Bishop)), ((4, 8), Piece::new(Black, Queen)),
            ((6, 8), Piece::new(Black, King)),   ((6, 8), Piece::new(Black, Bishop)),
            ((8, 8), Piece::new(Black, Knight)), ((8, 8), Piece::new(Black, Rook)),
            ((2, 7), Piece::new(Black, Pawn)),   ((2, 7), Piece::new(Black, Pawn)),
            ((4, 7), Piece::new(Black, Pawn)),   ((4, 7), Piece::new(Black, Pawn)),
            ((6, 7), Piece::new(Black, Pawn)),   ((6, 7), Piece::new(Black, Pawn)),
            ((8, 7), Piece::new(Black, Pawn)),   ((8, 7), Piece::new(Black, Pawn)),
        ];
        let mut newboard: HashMap<(u32, u32), Piece> = HashMap::new();
        for set in positions.iter() { newboard.insert(set.0, set.1);}
        Chess { board: newboard, next_player: White }
    }

    pub fn perform_move(&mut self, move_to_perform: (&str)) -> Result<(), &'static str> {
        let (from, to) = parse_input(move_to_perform)?;
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
        {
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
        }

        // Get pieces
        let chars = move_to_perform.chars();
        let to   = (Chess::col_nr(chars.nth(2)), chars.nth(3).to_digit(10));
        let from = (Chess::col_nr(chars.nth(0)), chars.nth(1).to_digit(10));
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

    fn col_nr(x: char) -> u8 {
        (x as u32 - 65) as u8
    }

    fn col_name(x: u32) -> char {
        std::char::from_u32(x + 48).unwrap()
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
