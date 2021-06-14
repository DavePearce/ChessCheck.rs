use std::fmt;
use super::piece::*;

/**
 * Define Chess board representation.
 */
pub struct Board {
    squares: [Piece;64]
}

/**
 * Constant defining the starting board of
 * a chess game.
 */
pub const INITIAL: Board = Board {
    squares: [
        WHITE_ROOK, WHITE_KNIGHT, WHITE_BISHOP, WHITE_QUEEN, WHITE_KING, WHITE_BISHOP, WHITE_KNIGHT, WHITE_ROOK,
        WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN, WHITE_PAWN,
        BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK,
        BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK,
        BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK,
        BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK, BLANK,         
        BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN, BLACK_PAWN,
        BLACK_ROOK, BLACK_KNIGHT, BLACK_BISHOP, BLACK_QUEEN, BLACK_KING, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK
    ]
};

/** 
 * Provide textual representation of Board
 */
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..8).rev() {
            // write line
            write!(f,"{}",y+1)?;            
            // write pieces
            for x in 0..8 {
                write!(f,"|{}",self.squares[x + (y*8)])?;
            }
            writeln!(f,"|")?;            
        }
        // write bottome line
        let mut c = 'a';
        write!(f,"  a b c d e f g h")?;
        Ok(())
    }
}
