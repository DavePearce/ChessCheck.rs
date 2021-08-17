use super::moves;
use super::moves::Move;
use super::piece::Player;
use super::board::*;
use std::fmt;
use std::str;

pub struct Game {
    // Sequence of moves beggining with White's and alternating between White and Black.
    moves: Vec<Box<dyn Move>>,
}

pub fn from_str(s: &str) -> Result<Game,()> {
    let mut ms = Vec::<Box<dyn Move>>::new();
    // Read line-by-line
    for l in s.lines() {
        // Split moves
        let v: Vec<&str> = l.split(' ').collect();
        //
        match v.len() {
            1 => {
                // Append white's move
                ms.push(moves::from_str(v[0],Player::White)?);
            }
            2 => {
                // Append white's move
                ms.push(moves::from_str(v[0],Player::White)?);
                // Append black's move
                ms.push(moves::from_str(v[1],Player::Black)?);
            }
            _ => return Err(()),
        }
    }
    // Dummy for now
    Ok(Game { moves: ms })
}

impl Game {
    /**
     * Apply this game to a given board, producing a board
     * representing the state of the game after all the moves have
     * been applied.
     */
    pub fn apply(&self, mut board: Board) -> Option<Board> {
	for m in &self.moves {
	    board = m.apply(board)?;
	}
	return Some(board);
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = true;
        for m in &self.moves {
            if w {
                write!(f, "{} ", m)?;
            } else {
                writeln!(f, "{} ", m)?;
            }
            w = !w;
        }
        Ok(())
    }
}

// ======================================================
// Pawn Tests
// ======================================================

#[test]
fn test_pawn_01() {
    let ms = "a2-a3";
    // Check move sequence
    check2(ms,
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
 let ms = "a2-a3 b7-b6";
    // Check move sequence
    check2(ms,
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
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_11() {
    let ms = vec!["e2-e3","c7-c6",
		  "e3-e5"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_12() {
    let ms = vec!["e2-e4","e7-e5",
		  "e4-d5"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_13() {
    let ms = vec!["e2-e4","e7-e5",
		  "e4-e3"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_14() {
    let ms = vec!["a2-a3","d7-d5",
		  "a3-a4","d5-d4",
		  "a4-a5","d4-d3",
		  "d2-d4"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_15() {
    let ms = vec!["a2-a3","d7-d5",
		  "a3-a4","d5-d4",
		  "d2-d4"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_16() {
    let ms = vec!["e2-e4","e7-e5",
		  "e4xe5"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_17() {
    let ms = vec!["c2-c4","e7-e6",
		  "c4xe6"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_18() {
    let ms = vec!["c2-c4","d7-d6",
		  "c4xd6"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_19() {
    let ms = vec!["c2-c4","d7-d6",
		  "c4xd5"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
}

#[test]
fn test_pawn_20() {
    let ms = vec!["c2-c4","d7-d5",
		  "c4-c5","d5-d4",
		  "c5xd4"];
    // Check failed    
    assert!(moves::apply(ms,INITIAL).is_none());
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
fn check2(game: &str, expected: &str) {
    // Parse game string
    let g = from_str(game).unwrap();
    // Apply each move to initial board producing a potentially
    // updated board.    
    let obrd = g.apply(INITIAL);
    // Print expected board
    println!("Expected:\n{}\n",expected);
    //
    if obrd.is_some() {
	// Print actual board
	println!("Actual:\n{}",obrd.unwrap().to_string());
    }
    // Attempt to unwrap the board
    let brd = obrd.unwrap().to_string();
    // Check whether they match
    assert_eq!(brd,expected);   
}

fn check(moves: Vec<&str>, expected: &str) {
    // Apply each move to initial board.  We are not expecting this to
    // fail.
    let brd = moves::apply(moves,INITIAL).unwrap().to_string();
    // Print expected board
    println!("Expected:\n{}\n",expected);
    // Print actual board
    println!("Actual:\n{}",brd);
    // Check whether they match
    assert_eq!(brd,expected);   
}
