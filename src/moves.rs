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
pub trait Move : fmt::Display {
    /**
     * Apply a given move to the board, either producing an updated
     * board (success) or nothing (failure).
     */
    fn apply(&mut self, board: Board) -> Option<Board>;
}

/**
 * Parse a given string into a Move.  If the string is invalid, then
 * an error is returned.
 */
pub fn from_str(s:&str) -> Result<Box<dyn Move>,()> {
    parse_move(s)
}

fn parse_move(s1:&str) -> Result<Box<dyn Move>,()> {
    // Parse piece (if exists)
    let (piece,s2) = parse_piece(s1,true);
    // Parse origin
    let (from, s3) = parse_square(s2)?;
    // Check whether this is a take or not
    let (kind, s4) = parse_kind(s3);    
    // Parse piece (if exists)
    let (taken,s5) = parse_piece(s4,false);
    // Parse destiation
    let (to,  s6) = parse_square(s5)?;
    // Create appropriate move
    let m : Box<dyn Move> = if kind {
	Box::new(SimpleTake{piece,from,to,taken})	
    } else {
	Box::new(SimpleMove{piece,from,to})
    };
    // Done
    Ok(m)
}

/**
 * Parse a single character piece (e.g. "Q", "K", "B", etc).  If no
 * valid character piece exists, then assume its a pawn.
 */
fn parse_piece(mut s:&str, w:bool) -> (Piece,&str) {
    let p : Piece;
    // Parse piece (if exists)       
    if piece::is_char(&s[0..1]) {
	// Piece given
	p = piece::from_str(&s[0..1]).unwrap();
	s = skip(s,1);
    } else if w {
	p = WHITE_PAWN;
    } else {
	p = BLACK_PAWN;
    }
    (p,s)
}

/**
 * Parse a square (e.g. "a5", "b3", etc).  This can produce an error
 * as a location can be incorrectly specified.
 */
fn parse_square(s:&str) -> Result<(Square,&str),()> {
    // Parse square
    let sq = squares::from_str(&s[0..2])?;
    // Done
    Ok((sq,skip(s,2)))
}

/**
 * Parse the kind of a move.
 */
fn parse_kind(s:&str) -> (bool,&str) {
    let b = &s[0..1] == "x";
    //
    (b,skip(s,1))
}

/**
 * Method for moving through a given string slice
 */
fn skip(s:&str,i:usize) -> &str {
    &s[i..]
}

// ================================================================
// Simple Move
// ================================================================

/**
 * Repreresents the movement of a piece on the board, such as "Bb1-e5"
 * or "b1-e5".  This is the simplest of all moves in the game.
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

// ================================================================
// Simple Take
// ================================================================

/**
 * Repreresents a move of a piece on the board which takes another,
 * such as "Bb1xe5" or "Bb1xQe5".
 */
pub struct SimpleTake {
    // piece doing the taking (the taker)
    piece: Piece,
    // location of taker
    from: Square,
    // location of takee
    to: Square,
    // piece being taken (the takee)    
    taken: Piece
}

/**
 * Logic for deciding whether or not a physical move can be applied.
 * If the move is invalid, then None is returned.
 */
impl Move for SimpleTake {
    fn apply(&mut self, board: Board) -> Option<Board> {
        None
    }
}

/**
 * Generic debugging output.
 */
impl fmt::Display for SimpleTake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match (self.piece.kind,self.taken.kind) {
	    (Kind::Pawn,Kind::Pawn) => {
		write!(f,"{}x{}",self.from,self.to)
	    }
	    (Kind::Pawn,_) => {
		write!(f,"{}x{}{}",self.from,self.taken,self.to)
	    }
	    (_,Kind::Pawn) => {
		write!(f,"{}{}x{}",self.piece,self.from,self.to)
	    }	    
	    _ => {
		write!(f,"{}{}x{}{}",self.piece,self.from,self.taken,self.to)
	    }
	}        
    }
}
