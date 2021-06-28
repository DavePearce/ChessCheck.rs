use super::board::Board;
use super::piece;
use super::piece::*;
use super::squares;
use super::squares::Square;
use std::fmt;
use std::str;

/**
 * Represents a given move in the game, which could be moving a piece,
 * taking another piece and/or putting the opposition in Check.
 */
pub trait Move: fmt::Display {
    /**
     * Apply a given move to the board, either producing an updated
     * board (success) or nothing (failure).
     */
    fn apply(&mut self, board: Board) -> Option<Board>;
}

pub fn from_str(s:&str) -> Result<Move,()> {
    parse_move(s)
}

fn parse_move(mut s:&str) -> Result<Move,()> {
    let piece : Piece;
    // Parse piece (if exists)       
    if piece::is_char(&s[0..1]) {
	// Piece given
	piece = piece::from_str(&s[0..1])?;
	s = &s[1..];
    } else {
	piece = WHITE_PAWN;
    }
    // Parse from
    let from = squares::from_str(&s[0..2])?;
    // Parse to
    let to = squares::from_str(&s[3..5])?;
    // Check whether this is a take or not
    if &s[2..3] == "x" {
	println!("MATCHED TAKE");
    }
    Ok(SimpleMove{piece,from,to})
}

/**
 * Repreresents the phhysical move of a piece on the board, such as "Bb1-e5".
 */
pub struct SimpleMove {
    piece: Piece,
    from: Square,
    to: Square,
}

/**
 * Logic for deciding whether or not a physical move can be applied.
 * If the move is invalid, then None is returned.
 */
impl Move for SimpleMove {
    fn apply(&mut self, board: Board) -> Option<Board> {
        None
    }
}

/**
 * Generic debugging output.
 */
impl fmt::Display for SimpleMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self.piece.kind {
	    Kind::Pawn => {
		write!(f,"{}-{}",self.from,self.to)
	    }
	    _ => {
		write!(f,"{}{}-{}",self.piece,self.from,self.to)
	    }
	}        
    }
}
