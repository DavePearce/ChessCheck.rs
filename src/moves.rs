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
pub trait Move: fmt::Display + str::FromStr {
    /**
     * Apply a given move to the board, either producing an updated
     * board (success) or nothing (failure).
     */
    fn apply(&mut self, board: Board) -> Option<Board>;
}

pub fn from_str(s:&str) -> Result<PhysicalMove,()> {
    if s.len() < 5 || s.len() > 6 {
        Err(())
    } else if s.len() == 5 {
        let mut f = squares::from_str(&s[0..2])?;
        let mut t = squares::from_str(&s[3..5])?;
        // Done
        Ok(PhysicalMove {
            piece: WHITE_PAWN,
            from: f,
            to: t,
        })
    } else {
        let mut p = piece::from_str(&s[0..1])?;
        let mut f = squares::from_str(&s[1..3])?;
        let mut t = squares::from_str(&s[4..6])?;
        // Done
        Ok(PhysicalMove {
            piece: p,
            from: f,
            to: t,
        })
    }
}

/**
 * Repreresents the phhysical move of a piece on the board, such as "Bb1-e5".
 */
pub struct PhysicalMove {
    piece: Piece,
    from: Square,
    to: Square,
}

/**
 * Logic for deciding whether or not a physical move can be applied.
 * If the move is invalid, then None is returned.
 */
impl Move for PhysicalMove {
    fn apply(&mut self, board: Board) -> Option<Board> {
        None
    }
}

impl fmt::Display for PhysicalMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}{}-{}",self.piece,self.from,self.to)
    }
}

/**
 * Parse a physical move of the from Xyy-zz (e.g. "Ba1-e5", etc).  Since we cannot 
 * determine from a given move string whether or not the move is for
 * White or Black, we always return a move for White.
 */
impl str::FromStr for PhysicalMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        self::from_str(s)
    }
}
