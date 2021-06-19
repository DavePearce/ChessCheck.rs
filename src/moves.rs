use std::fmt;
use super::piece::*;
use super::board::Board;
use super::squares::Square;

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
 * Repreresents the phhysical move of a piece on the board, such as "Bb1-e5".
 */
struct PhysicalMove {
    piece: Piece,
    from: Square,
    to: Square
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
        write!(f,"Xyy-zz")
    }
}
