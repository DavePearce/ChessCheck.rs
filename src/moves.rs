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
	let m = from_str(s).unwrap();
	// Apply move 
	board = m.apply(board)?;
    }
    return Some(board);
}

/**
 * Parse a given string into a Move.  If the string is invalid, then
 * an error is returned.
 */
pub fn from_str(s1:&str) -> Result<Box<dyn Move>,()> {
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
    // Create simple move
    let parent = SimpleMove{piece,from,to};
    // Create appropriate move
    let m : Box<dyn Move> = if kind {
	Box::new(SimpleTake{parent,taken})	
    } else {
	Box::new(parent)
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
    fn apply(&self, mut board: Board) -> Option<Board> {
	// Read out piece at from position
        let p = board.get(self.from);
	// Remove piece from board
	board = board.set(self.from,BLANK);
	// Put piece at new position
	board = board.set(self.to,p);
	// Done
	return Some(board);
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
    // Underlying move
    parent: SimpleMove,
    // piece being taken (the takee)    
    taken: Piece
}

/**
 * Logic for deciding whether or not a physical move can be applied.
 * If the move is invalid, then None is returned.
 */
impl Move for SimpleTake {
    fn apply(&self, mut board: Board) -> Option<Board> {
	return self.parent.apply(board);
    }
}

/**
 * Generic debugging output.
 */
impl fmt::Display for SimpleTake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match (self.parent.piece.kind,self.taken.kind) {
	    (Kind::Pawn,Kind::Pawn) => {
		write!(f,"{}x{}",self.parent.from,self.parent.to)
	    }
	    (Kind::Pawn,_) => {
		write!(f,"{}x{}{}",self.parent.from,self.taken,self.parent.to)
	    }
	    (_,Kind::Pawn) => {
		write!(f,"{}{}x{}",self.parent.piece,self.parent.from,self.parent.to)
	    }	    
	    _ => {
		write!(f,"{}{}x{}{}",self.parent.piece,self.parent.from,self.taken,self.parent.to)
	    }
	}        
    }
}

// ======================================================
// Pawn Tests
// ======================================================

#[test]
fn test_pawn_01() {
    let ms = vec!["a2-a3"];
    // Check move sequence
    check(ms,
r#"8|r|n|b|q|k|b|n|r|
7|p|p|p|p|p|p|p|p|
6|_|_|_|_|_|_|_|_|
5|_|_|_|_|_|_|_|_|
4|_|_|_|_|_|_|_|_|
3|P|_|_|_|_|_|_|_|
2|_|P|P|P|P|P|P|P|
1|R|N|B|Q|K|B|N|R|
  a b c d e f g h"#);    
}

#[test]
fn test_pawn_02() {
 let ms = vec!["a2-a3","b7-b6"];
    // Check move sequence
    check(ms,
r#"8|r|n|b|q|k|b|n|r|
7|p|_|p|p|p|p|p|p|
6|_|p|_|_|_|_|_|_|
5|_|_|_|_|_|_|_|_|
4|_|_|_|_|_|_|_|_|
3|P|_|_|_|_|_|_|_|
2|_|P|P|P|P|P|P|P|
1|R|N|B|Q|K|B|N|R|
  a b c d e f g h"#);
}

#[test]
fn test_pawn_03() {
 let ms = vec!["a2-a4","b7-b5"];
    // Check move sequence
    check(ms,
r#"8|r|n|b|q|k|b|n|r|
7|p|_|p|p|p|p|p|p|
6|_|_|_|_|_|_|_|_|
5|_|p|_|_|_|_|_|_|
4|P|_|_|_|_|_|_|_|
3|_|_|_|_|_|_|_|_|
2|_|P|P|P|P|P|P|P|
1|R|N|B|Q|K|B|N|R|
  a b c d e f g h"#);
}

#[test]
fn test_pawn_04() {
    let ms = vec!["d2-d4","d7-d5",
		  "e2-e4","d5xe4"];
    // Check move sequence
    check(ms,
r#"8|r|n|b|q|k|b|n|r|
7|p|p|p|_|p|p|p|p|
6|_|_|_|_|_|_|_|_|
5|_|_|_|_|_|_|_|_|
4|_|_|_|P|p|_|_|_|
3|_|_|_|_|_|_|_|_|
2|P|P|P|_|_|P|P|P|
1|R|N|B|Q|K|B|N|R|
  a b c d e f g h"#);
}

#[test]
fn test_pawn_05() {
    let ms = vec!["d2-d3","d7-d5",
		  "e2-e4","d5xe4",
		  "d3xe4"];
    // Check move sequence
    check(ms,
r#"8|r|n|b|q|k|b|n|r|
7|p|p|p|_|p|p|p|p|
6|_|_|_|_|_|_|_|_|
5|_|_|_|_|_|_|_|_|
4|_|_|_|_|P|_|_|_|
3|_|_|_|_|_|_|_|_|
2|P|P|P|_|_|P|P|P|
1|R|N|B|Q|K|B|N|R|
  a b c d e f g h"#);
}

#[test]
fn test_pawn_10() {
    let ms = vec!["e2-e5"];
    // Check failed
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_11() {
    let ms = vec!["e2-e3","c7-c6",
		  "e3-e5"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_12() {
    let ms = vec!["e2-e4","e7-e5",
		  "e4-d5"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_13() {
    let ms = vec!["e2-e4","e7-e5",
		  "e4-e3"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_14() {
    let ms = vec!["a2-a3","d7-d5",
		  "a3-a4","d5-d4",
		  "a4-a5","d4-d3",
		  "d2-d4"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_15() {
    let ms = vec!["a2-a3","d7-d5",
		  "a3-a4","d5-d4",
		  "d2-d4"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_16() {
    let ms = vec!["e2-e4","e7-e5",
		  "e4xe5"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_17() {
    let ms = vec!["c2-c4","e7-e6",
		  "c4xe6"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_18() {
    let ms = vec!["c2-c4","d7-d6",
		  "c4xd6"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_19() {
    let ms = vec!["c2-c4","d7-d6",
		  "c4xd5"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_20() {
    let ms = vec!["c2-c4","d7-d5",
		  "c4-c5","d5-d4",
		  "c5xd4"];
    // Check failed    
    assert!(apply(ms,INITIAL).is_none());
}

// ======================================================
// Knight Tests
// ======================================================

#[test]
fn test_knight_01() {
    let ms = vec!["Nb1-a3"];
    // Check move sequence
    check(ms,
r#"8|r|n|b|q|k|b|n|r|
7|p|p|p|p|p|p|p|p|
6|_|_|_|_|_|_|_|_|
5|_|_|_|_|_|_|_|_|
4|_|_|_|_|_|_|_|_|
3|N|_|_|_|_|_|_|_|
2|P|P|P|P|P|P|P|P|
1|R|_|B|Q|K|B|N|R|
  a b c d e f g h"#);    
}

// ======================================================
// Helpers
// ======================================================

/**
 * Check that a given sequence of moves from the initial board end up
 * producing an expected board.
 */
fn check(moves: Vec<&str>, expected: &str) {
    // Apply each move to initial board.  We are not expecting this to
    // fail.    
    let brd = apply(moves,INITIAL).unwrap().to_string();
    // Print expected board
    println!("Expected:\n{}\n",expected);
    // Print actual board
    println!("Actual:\n{}",brd);
    // Check whether they match
    assert_eq!(brd,expected);   
}
