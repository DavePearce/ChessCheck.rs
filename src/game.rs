use super::muve;
use super::muve::Move;
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
        let v: Vec<&str> = l.split_ascii_whitespace().collect();
        //
        match v.len() {
            1 => {
                // Append white's move
                ms.push(muve::from_str(v[0],Player::White)?);
            }
            2 => {
                // Append white's move
                ms.push(muve::from_str(v[0],Player::White)?);
                // Append black's move
                ms.push(muve::from_str(v[1],Player::Black)?);
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
    #[allow(dead_code)]
    pub fn apply(&self, mut board: Board) -> Result<Board,Board> {
	for m in &self.moves {
	    let b = m.apply(board);
	    // Sanity check
	    if let Some(brd) = b {
		board = brd;
	    } else {
		return Err(board);
	    } 
	}
	Ok(board)
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
    check_valid("a2-a3",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|P|_|_|_|_|_|_|_|\n\
		 2|_|P|P|P|P|P|P|P|\n\
		 1|R|N|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");    
}

#[test]
fn test_pawn_02() {
    check_valid("a2-a3 b7-b6",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|_|p|p|p|p|p|p|\n\
		 6|_|p|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|P|_|_|_|_|_|_|_|\n\
		 2|_|P|P|P|P|P|P|P|\n\
		 1|R|N|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_pawn_03() {
    check_valid("a2-a4 b7-b5",
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|_|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|p|_|_|_|_|_|_|\n\
		 4|P|_|_|_|_|_|_|_|\n\
		 3|_|_|_|_|_|_|_|_|\n\
		 2|_|P|P|P|P|P|P|P|\n\
		 1|R|N|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_pawn_04() {
    check_valid("d2-d4 d7-d5\n\
		 e2-e4 d5xe4",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|P|p|_|_|_|\n\
		 3|_|_|_|_|_|_|_|_|\n\
		 2|P|P|P|_|_|P|P|P|\n\
		 1|R|N|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_pawn_05() {
    check_valid("d2-d3 d7-d5\n\
		 e2-e4 d5xe4\n\
		 d3xe4",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|P|_|_|_|\n\
		 3|_|_|_|_|_|_|_|_|\n\
		 2|P|P|P|_|_|P|P|P|\n\
		 1|R|N|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_pawn_10() {
    check_invalid("e2-e5");
}

#[test]
fn test_pawn_11() {
    check_invalid("e2-e3 c7-c6\n\
		   e3-e5");
}

#[test]
fn test_pawn_12() {
    check_invalid("e2-e4 e7-e5\n\
		   e4-d5");
}

#[test]
fn test_pawn_13() {
    check_invalid("e2-e4 e7-e5\n\
		   e4-e3");
}

#[test]
fn test_pawn_14() {
    check_invalid("a2-a3 d7-d5\n\
		   a3-a4 d5-d4\n\
		   a4-a5 d4-d3\n\
		   d2-d4");
}

#[test]
fn test_pawn_15() {
    check_invalid("a2-a3 d7-d5\n\
		   a3-a4 d5-d4\n\
		   d2-d4");
}

#[test]
fn test_pawn_16() {
    check_invalid("e2-e4 e7-e5\n\
		   e4xe5");
}

#[test]
fn test_pawn_17() {
    check_invalid("c2-c4 e7-e6\n\
		   c4xe6");
}

#[test]
fn test_pawn_18() {
    check_invalid("c2-c4 d7-d6\n\
		   c4xd6");
}

#[test]
fn test_pawn_19() {
    check_invalid("c2-c4 d7-d6\n\
		   c4xd5");
}

#[test]
fn test_pawn_20() {
    check_invalid("c2-c4 d7-d5\n\
		   c4-c5 d5-d4\n\
		   c5xd4");
}

// ======================================================
// Knight Tests
// ======================================================

#[test]
fn test_knight_01() {
    check_valid("Nb1-a3",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|N|_|_|_|_|_|_|_|\n\
		 2|P|P|P|P|P|P|P|P|\n\
		 1|R|_|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");    
}

#[test]
fn test_knight_02() {
    check_valid("Nb1-c3",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|N|_|_|_|_|_|\n\
		 2|P|P|P|P|P|P|P|P|\n\
		 1|R|_|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h"
    );
}

#[test]
fn test_knight_03() {
    check_valid("Nb1-c3 Ng8-f6",
		//
		"8|r|n|b|q|k|b|_|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|n|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|N|_|_|_|_|_|\n\
		 2|P|P|P|P|P|P|P|P|\n\
		 1|R|_|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_knight_04() {
    check_valid("Nb1-c3 Ng8-f6\n\
		 Ng1-f3 Nb8-c6\n",
		//
		"8|r|_|b|q|k|b|_|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|n|_|_|n|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|N|_|_|N|_|_|\n\
		 2|P|P|P|P|P|P|P|P|\n\
		 1|R|_|B|Q|K|B|_|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_knight_05() {
    check_valid("Ng1-f3 Ng8-f6\n\
		 Nf3-d4",
		//
		"8|r|n|b|q|k|b|_|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|n|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|N|_|_|_|_|\n\
		 3|_|_|_|_|_|_|_|_|\n\
		 2|P|P|P|P|P|P|P|P|\n\
		 1|R|N|B|Q|K|B|_|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_knight_06() {
    check_valid("Ng1-f3 Ng8-f6\n\
		 c2-c3 Nf6-e4",
		//
		"8|r|n|b|q|k|b|_|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|n|_|_|_|\n\
		 3|_|_|P|_|_|N|_|_|\n\
		 2|P|P|_|P|P|P|P|P|\n\
		 1|R|N|B|Q|K|B|_|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_knight_07() {
    check_valid("Nb1-c3 d7-d5\nNc3xd5",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		7|p|p|p|_|p|p|p|p|\n\
		6|_|_|_|_|_|_|_|_|\n\
		5|_|_|_|N|_|_|_|_|\n\
		4|_|_|_|_|_|_|_|_|\n\
		3|_|_|_|_|_|_|_|_|\n\
		2|P|P|P|P|P|P|P|P|\n\
		1|R|_|B|Q|K|B|N|R|\n\
		-|a b c d e f g h");
}

#[test]
fn test_knight_08() {
    check_valid("d2-d4 Nb8-c6\nNg1-f3 Nc6xd4",
		//
		"8|r|_|b|q|k|b|n|r|\n\
		7|p|p|p|p|p|p|p|p|\n\
		6|_|_|_|_|_|_|_|_|\n\
		5|_|_|_|_|_|_|_|_|\n\
		4|_|_|_|n|_|_|_|_|\n\
		3|_|_|_|_|_|N|_|_|\n\
		2|P|P|P|_|P|P|P|P|\n\
		1|R|N|B|Q|K|B|_|R|\n\
		-|a b c d e f g h"
    );
}

#[test]
fn test_knight_09() {
    check_valid("d2-d4 Nb8-c6\nNg1-f3 Nc6-e5\nNf3xNe5",
		//
		"8|r|_|b|q|k|b|n|r|\n\
		7|p|p|p|p|p|p|p|p|\n\
		6|_|_|_|_|_|_|_|_|\n\
		5|_|_|_|_|N|_|_|_|\n\
		4|_|_|_|P|_|_|_|_|\n\
		3|_|_|_|_|_|_|_|_|\n\
		2|P|P|P|_|P|P|P|P|\n\
		1|R|N|B|Q|K|B|_|R|\n\
		-|a b c d e f g h"
    );
}

#[test]
fn test_knight_10() {
    check_valid("Nb1-c3 a7-a6\nNc3-d5 a6-a5\nNd5xe7",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		7|_|p|p|p|N|p|p|p|\n\
		6|_|_|_|_|_|_|_|_|\n\
		5|p|_|_|_|_|_|_|_|\n\
		4|_|_|_|_|_|_|_|_|\n\
		3|_|_|_|_|_|_|_|_|\n\
		2|P|P|P|P|P|P|P|P|\n\
		1|R|_|B|Q|K|B|N|R|\n\
		-|a b c d e f g h"
    );
}

#[test]
fn test_knight_11() {
    check_valid("d2-d4 Nb8-c6\n\
		 e2-e4 Nc6xd4",
		//
		"8|r|_|b|q|k|b|n|r|\n\
		 7|p|p|p|p|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|n|P|_|_|_|\n\
		 3|_|_|_|_|_|_|_|_|\n\
		 2|P|P|P|_|_|P|P|P|\n\
		 1|R|N|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_knight_12() {
    check_valid("Nb1-c3 e7-e5\n\
		 e2-e3 e5-e4\n\
		 Nc3xe4",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|p|_|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|N|_|_|_|\n\
		 3|_|_|_|_|P|_|_|_|\n\
		 2|P|P|P|P|_|P|P|P|\n\
		 1|R|_|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_knight_13() {
    check_invalid("Nb1-b3");
}

#[test]
fn test_knight_14() {
    check_invalid("Nb1-c4");
}

#[test]
fn test_knight_15() {    
    check_invalid("Nb1-d2");
}

#[test]
fn test_knight_16() {    
    check_invalid("Nb1-c3 d7-d5\n\
		   Nc3-d5");
}

#[test]
fn test_knight_17() {    
    check_invalid("Nb1-c3 e7-e6\n\
		   Nc3-a3");
}

#[test]
fn test_knight_18() {
    check_invalid("Nb1xd2");
}

#[test]
fn test_knight_19() {
    check_invalid("Nb1xb7");
}

#[test]
fn test_knight_20() {
    check_invalid("Nb1-c3 e7-e6\n\
		   Nc3xe6");
}

#[test]
fn test_knight_21() {
    check_invalid("e2-e4 Nb8-c6\n\
		   e4-e5 Nc6xd4");
}

#[test]
fn test_knight_22() {
    check_invalid("d2-d4 Nb8-c6\n\
		   Ng1-f3 Nc6-e5\n\
		   Nf3xBe5");
}

// ======================================================
// Bishops
// ======================================================

#[test]
fn test_bishop_01() {
    check_valid("e2-e3 e7-e6\nBf1-d3",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|p|_|p|p|p|\n\
		 6|_|_|_|_|p|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|_|B|P|_|_|_|\n\
		 2|P|P|P|P|_|P|P|P|\n\
		 1|R|N|B|Q|K|_|N|R|\n\
		 -|a b c d e f g h");    
}

#[test]
fn test_bishop_02() {
    check_valid("d2-d3 d7-d6\nBc1-e3",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|p|_|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|_|P|B|_|_|_|\n\
		 2|P|P|P|_|P|P|P|P|\n\
		 1|R|N|_|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_03() {    
    check_valid("e2-e3 e7-e6\nBf1-d3 Bf8-d6",
		//
		"8|r|n|b|q|k|_|n|r|\n\
		 7|p|p|p|p|_|p|p|p|\n\
		 6|_|_|_|b|p|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|_|B|P|_|_|_|\n\
		 2|P|P|P|P|_|P|P|P|\n\
		 1|R|N|B|Q|K|_|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_04() {
    check_valid("d2-d3 d7-d6\nBc1-e3 Bc8-e6",
		//
		"8|r|n|_|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|p|b|_|_|_|\n\
		 5|_|_|_|_|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|_|P|B|_|_|_|\n\
		 2|P|P|P|_|P|P|P|P|\n\
		 1|R|N|_|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_05() {    
    check_valid("d2-d3 d7-d5\n\
		 Bc1-g5",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|p|_|_|B|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|_|P|_|_|_|_|\n\
		 2|P|P|P|_|P|P|P|P|\n\
		 1|R|N|_|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_06() {  
    check_valid("d2-d3 d7-d5\n\
		 Bc1-g5 Bc8-g4\n",
		//
		"8|r|n|_|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|p|_|_|B|_|\n\
		 4|_|_|_|_|_|_|b|_|\n\
		 3|_|_|_|P|_|_|_|_|\n\
		 2|P|P|P|_|P|P|P|P|\n\
		 1|R|N|_|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_07() {  
    check_valid("d2-d3 d7-d5\n\
		 Bc1-g5 Bc8-g4\n\
		 Bg5-h4\n",
		//
		"8|r|n|_|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|p|_|_|_|_|\n\
		 4|_|_|_|_|_|_|b|B|\n\
		 3|_|_|_|P|_|_|_|_|\n\
		 2|P|P|P|_|P|P|P|P|\n\
		 1|R|N|_|Q|K|B|N|R|\n\
		 -|a b c d e f g h");   
}

#[test]
fn test_bishop_08() {  
    check_valid("d2-d3 d7-d5\n\
		 Bc1-g5 Bc8-g4\n\
		 Bg5xe7\n",
		//
		"8|r|n|_|q|k|b|n|r|\n\
		 7|p|p|p|_|B|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|p|_|_|_|_|\n\
		 4|_|_|_|_|_|_|b|_|\n\
		 3|_|_|_|P|_|_|_|_|\n\
		 2|P|P|P|_|P|P|P|P|\n\
		 1|R|N|_|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_09() {      
    check_valid("d2-d3 d7-d5\n\
		 Bc1-g5 Bc8-g4\n\
		 Bg5-h4 Bg4xe2\n",
		//
		"8|r|n|_|q|k|b|n|r|\n\
		 7|p|p|p|_|p|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|p|_|_|_|_|\n\
		 4|_|_|_|_|_|_|_|B|\n\
		 3|_|_|_|P|_|_|_|_|\n\
		 2|P|P|P|_|b|P|P|P|\n\
		 1|R|N|_|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_10() {
    check_valid("e2-e3 f7-f5\n\
		 Bf1-d3 g7-g6\n\
		 Bd3xf5",
		//
		"8|r|n|b|q|k|b|n|r|\n\
		 7|p|p|p|p|p|_|_|p|\n\
		 6|_|_|_|_|_|_|p|_|\n\
		 5|_|_|_|_|_|B|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|_|_|P|_|_|_|\n\
		 2|P|P|P|P|_|P|P|P|\n\
		 1|R|N|B|Q|K|_|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_11() {
    check_valid("c2-c4 e7-e5\n\
		 c4-c5 Bf8xc5",
		//
		"8|r|n|b|q|k|_|n|r|\n\
		 7|p|p|p|p|_|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|b|_|p|_|_|_|\n\
		 4|_|_|_|_|_|_|_|_|\n\
		 3|_|_|_|_|_|_|_|_|\n\
		 2|P|P|_|P|P|P|P|P|\n\
		 1|R|N|B|Q|K|B|N|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_12() {    
    check_valid("f2-f4 e7-e5\n\
		 g2-g3 Bf8-c5\n\
		 Nb1-c3 Bc5xNg1",
		//
		"8|r|n|b|q|k|_|n|r|\n\
		 7|p|p|p|p|_|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|p|_|_|_|\n\
		 4|_|_|_|_|_|P|_|_|\n\
		 3|_|_|N|_|_|_|P|_|\n\
		 2|P|P|P|P|P|_|_|P|\n\
		 1|R|_|B|Q|K|B|b|R|\n\
		 -|a b c d e f g h");
}

#[test]
fn test_bishop_13() {    
    check_valid("e2-e4 e7-e5\n\
		 Bf1-a6 Bf8-a3\n\
		 Ba6xb7 Ba3xb2",
		//
		"8|r|n|b|q|k|_|n|r|\n\
		 7|p|B|p|p|_|p|p|p|\n\
		 6|_|_|_|_|_|_|_|_|\n\
		 5|_|_|_|_|p|_|_|_|\n\
		 4|_|_|_|_|P|_|_|_|\n\
		 3|_|_|_|_|_|_|_|_|\n\
		 2|P|b|P|P|_|P|P|P|\n\
		 1|R|N|B|Q|K|_|N|R|\n\
		 -|a b c d e f g h");
}

#[test] fn test_bishop_14() {
    check_invalid("Bc1-c3");
}
#[test] fn test_bishop_15() {
    check_invalid("Bc1-e3");
}
#[test] fn test_bishop_16() {
    check_invalid("Bc1-b3");
}
#[test] fn test_bishop_17() {
    check_invalid("c2-c3 e7-e6\n\
		   Bc1-c2");
}
#[test] fn test_bishop_18() {
    check_invalid("c2-c3 Bc8-c6");
}
#[test] fn test_bishop_19() {
    check_invalid("c2-c3 Bc8-e6");
}
#[test] fn test_bishop_20() {
    check_invalid("c2-c3 e7-e6\n\
		   d2-d4 Bf8-c5\n\
		   Bc1-d2 Bc5-e3");
}
#[test] fn test_bishop_21() {
    check_invalid("Bc1xc7");
}
#[test] fn test_bishop_22() {
    check_invalid("Bc1xh6");
}
#[test] fn test_bishop_23() {
    check_invalid("Bc1xd2");
}
#[test] fn test_bishop_24() {
    check_invalid("d2-d3 e7-e6\n\
		   Bc1-f4 f7-f6\n\
		   Bf4xNb8");
}
#[test] fn test_bishop_25() {
    check_invalid("c2-c3 e7-e6\n\
		   d2-d4 Bf8-c5\n\
		   Bc1-d2 Bc5xf2");
}



// ======================================================
// Helpers
// ======================================================

/**
 * Check that a given game (i.e. sequence of moves) produce an
 * expected board when applied to the initial board.
 */
#[cfg(test)]
fn check_valid(game: &str, expected: &str) {
    // Parse game string
    let g = from_str(game).unwrap();
    // Apply each move to initial board producing a potentially
    // updated board.    
    let obrd = g.apply(INITIAL);
    // Print game
    println!("Game:\n{}\n",game);
    // Print expected board
    println!("Expected:\n{}\n",expected);
    // Extract actual board
    let brd = match obrd {
	Ok(b) => b,
	Err(b) => b
    };
    // Print actual board
    println!("Actual:\n{}",brd.to_string());
    // Check whether they match
    assert!(brd.to_string() == expected);       
}

/**
 * Check that a given game (i.e. sequence of moves) produce an error
 * when applied to the initial board.
 */
#[cfg(test)]
fn check_invalid(game: &str) {
    // Parse game string
    let g = from_str(game).unwrap();
    // Apply each move to initial board producing a potentially
    // updated board.    
    let obrd = g.apply(INITIAL);
    // Expect this to have failed
    assert!(obrd.is_err());
}
