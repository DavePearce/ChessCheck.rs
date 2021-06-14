use super::piece::*;
use super::board::Board;
use super::squares::Square;

/**
 * Represents a given move in the game, which could be moving a piece,
 * taking another piece and/or putting the opposition in Check.
 */
pub trait Move {
    /**
     * Apply a given move to the board, either producing an updated
     * board (success) or nothing (failure).
     */   
    fn apply(&mut self, board: Board) -> Option<Board>; 
}

/**
 * A physical movement on the board.
 */
struct PhysicalMove {
    piece: Piece,
    from: Square,
    to: Square
}

impl Move for PhysicalMove {
    fn apply(&mut self, board: Board) -> Option<Board> {
        None
    }
}
