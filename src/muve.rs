use super::board::*;
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
    fn apply(&self, board: Board) -> Option<Board>;
}

/**
 * Apply a sequence of zero more moves to a given board, producing a
 * potentially updated board.  If one of the moves is invalid, then
 * nothing is returned.
 */
pub fn apply(moves: Vec<&str>, mut board: Board) -> Option<Board> {
    for s in moves {
	// Parse move
	let m = from_str(s,Player::White).unwrap();
	// Apply move 
	board = m.apply(board)?;
    }
    return Some(board);
}

/**
 * Parse a given string into a Move.  If the string is invalid, then
 * an error is returned.
 */
pub fn from_str(s1:&str, p:Player) -> Result<Box<dyn Move>,()> {
    // Parse piece (if exists)
    let (piece,s2) = parse_piece(s1,p);
    // Parse origin
    let (from, s3) = parse_square(s2)?;
    // Check whether this is a take or not
    let (kind, s4) = parse_kind(s3);    
    // Parse piece (if exists)
    let (taken,s5) = parse_piece(s4,p.flip());
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
fn parse_piece(mut s:&str, p:Player) -> (Piece,&str) {
    let mut r : Piece;
    // Parse piece (if exists)       
    if piece::is_char(&s[0..1]) {
	// Piece given
	r = piece::from_str(&s[0..1],p).unwrap();
	s = skip(s,1);
    } else {
	r = match p {
	    Player::White => WHITE_PAWN,
	    Player::Black => BLACK_PAWN
	};	
    }
    // Convert to appropriate player
    (r,s)
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
    /**
     * Piece doing the move
     */
    piece: Piece,
    /**
     * Starting position of piece
     */
    from: Square,
    /**
     * Ending position of piece
     */
    to: Square
}

/**
 * Logic for deciding whether or not a physical move can be applied.
 * If the move is invalid, then None is returned.
 */
impl Move for SimpleMove {
    fn apply(&self, mut board: Board) -> Option<Board> {
	println!("*** {}",self);	
	// Read out piece at from position
        let p = board.get(self.from);
	// Read out piece at to position (should be blank)
	let t = board.get(self.to);
	// Check piece matches what is expected
	if p == self.piece && t == BLANK && p.can_move(board,self.from,self.to) {
	    // Remove piece from board
	    board = board.set(self.from,BLANK);
	    // Put piece at new position
	    board = board.set(self.to,p);
	    // Done
	    return Some(board);
	}
	// Failure
	return None;
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
    /**
     * Piece doing the take
     */
    piece: Piece,
    /**
     * Starting position of piece
     */
    from: Square,
    /**
     * Ending position of piece
     */
    to: Square,
    /**
     * Piece being taken
     */
    taken: Piece
}

/**
 * Logic for deciding whether or not a physical move can be applied.
 * If the move is invalid, then None is returned.
 */
impl Move for SimpleTake {
    fn apply(&self, mut board: Board) -> Option<Board> {
	// Read out piece at from position
        let p = board.get(self.from);
	// Read out piece at to position (should be blank)
	let t = board.get(self.to);
	// Check piece matches what is expected
	if p == self.piece && t == self.taken && p.can_move(board,self.from,self.to) {
	    // Remove piece from board
	    board = board.set(self.from,BLANK);
	    // Put piece at new position
	    board = board.set(self.to,p);
	    // Done
	    return Some(board);
	}
	// Failure
	return None;
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
