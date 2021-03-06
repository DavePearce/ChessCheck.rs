use std::fmt;
use std::str;
use std::cmp;
use super::square::Square;
use super::board::Board;

// ==========================================================================
// Kind
// ==========================================================================

/**
 * The kind of a piece (e.g. kind, queen, etc)
 */
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    Blank,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Kind {
    /**
     * Convert a piece kind to a string for a given player.
     */
    pub fn to_string(self,p:Player) -> &'static str {
	match p {
	    Player::White => self.to_white_string(),
	    Player::Black => self.to_black_string()		
	}
    }

    /**
     * Convert a piece kind to a string for the white player.
     */    
    pub fn to_white_string(self) -> &'static str {
	match self {
            Kind::Blank => "_",
            Kind::Pawn => "P",
            Kind::Knight => "N",
            Kind::Bishop => "B",
            Kind::Rook => "R",
            Kind::Queen => "Q",
            Kind::King => "K"
        }
    }

    /**
     * Convert a piece kind to a string for the black player.
     */    
    pub fn to_black_string(self) -> &'static str {
	match self {
            Kind::Blank => "_",
            Kind::Pawn => "p",
            Kind::Knight => "n",
            Kind::Bishop => "b",
            Kind::Rook => "r",
            Kind::Queen => "q",
            Kind::King => "k"
        }
    }
}

// ==========================================================================
// Player
// ==========================================================================

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    White,
    Black
}

/**
 * Represents a player in the game (i.e. white or black).
 */
impl Player {
    /** 
     * Flip to other player.  For example, if this is white then it
     * becomes black, etc.
     */
    pub fn flip(&self) -> Player {
	match self {
	    Player::White => Player::Black,
	    Player::Black => Player::White		
	}
    }
}

// ==========================================================================
// Piece
// ==========================================================================

/**
 * Define a given piece on the board
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub player: Player,
    pub kind: Kind,
}

impl Piece {
    /**
     * Check whether this piece can move from one position on the
     * board to another.  There are few aspects to this: firstly, it
     * has to be a valid move for the piece (e.g. rooks cannot move
     * along a diagonal); secondly, the move needs to be unobstructed
     * (at least for some pieces).  Note, however, than the final
     * position is not considered here.  That is, we are not concerned
     * whether the final piece is obstructed or not (this is handled
     * elsewhere).    
     */
    pub fn can_move(&self, board: Board, from: Square, to: Square) -> bool {
	match self.kind {
	    Kind::Blank => false,
	    Kind::Pawn => can_pawn_move(board,self.player,from,to),
	    Kind::Knight => can_knight_move(board,from,to),
	    Kind::Bishop => can_bishop_move(board,from,to),
	    Kind::Rook => can_rook_move(board,from,to),
	    Kind::Queen => can_queen_move(board,from,to),
	    Kind::King => can_king_move(board,from,to)		
	}
    }
    
    /** 
     * Flip to other player.  For example, if this is white then it
     * becomes black, etc.
     */
    #[allow(dead_code)]
    pub fn flip(&self) -> Piece {
	Piece{player: self.player.flip(), kind: self.kind}
    }
}

/**
 * Convert a string representing a piece into an actual Piece (or an
 * error if invalid string).  In doing this, we must determine which
 * player the piece is for.
 */
pub fn from_str(s: &str, p:Player) -> Result<Piece, ()> {
    match s {
        "" => Ok(Piece{player: p,kind: Kind::Pawn}),
        "N" => Ok(Piece{player: p,kind: Kind::Knight}),
        "B" => Ok(Piece{player: p,kind: Kind::Bishop}),
        "R" => Ok(Piece{player: p,kind: Kind::Rook}),
        "Q" => Ok(Piece{player: p,kind: Kind::Queen}),
        "K" => Ok(Piece{player: p,kind: Kind::King}),
        _ => Err(()),
    }
}

/**
 * Test whether the first character of this string represents a kind
 * of piece.
 */
pub fn is_char(s: &str) -> bool {
    matches!(s,"N" | "B" | "R" | "Q" | "K")
}

/**
 * Determine whether a given pawn move is valid (or not).
 */
pub fn can_pawn_move(board: Board, player: Player, from: Square, to: Square) -> bool {
    // Get direction of movement for given player
    let dir : i8 = match player { Player::White => 1, Player::Black => -1 };
    let old_row = from.row() as i8;
    let old_col = from.column() as i8;
    let new_row = to.row() as i8;
    let new_col = to.column() as i8;
    let target = board.get(to);
    //
    if (old_row+dir) == new_row {
	// Case where pawn advances one row
	if old_col == new_col {
	    // Simple move, not take
	    return target == BLANK;
	} else if (old_col-1) == new_col || (old_col+1) == new_col {
	    // Take move
	    return target != BLANK;
	}
    } else if (old_row+dir+dir) == new_row && (old_col == new_col) {
	// Case where pawn advances two rows.  For this to be
	// permited, pawn must be on starting row.
	if (dir == 1 && old_row == 1) || (dir == -1 && old_row == 6) {
	    // Furthermore, cannot be anything in the way.  So,
	    // compute the square inbetween.
	    let gap = Square::new(from.column(), (old_row+dir) as u8);
	    // Check nothing is in the way!
	    return board.get(gap) == BLANK && target == BLANK;
	}
    }
    // Fail
    false
}

/**
 * Determine whether a given knight move is valid (or not).
 */
pub fn can_knight_move(_board: Board, from: Square, to: Square) -> bool {
    // Determine absolute difference in column
    let dcol = abs_diff_column(from,to);
    // Determine absolute different in row
    let drow = abs_diff_row(from,to);
    //
    (dcol == 1 && drow == 2) || (dcol == 2 && drow == 1)
}

/**
 * Determine whether a given bishop move is valid (or not).
 */
pub fn can_bishop_move(board: Board, from: Square, to: Square) -> bool {
    clear_diagonal_inner(board,from,to)
}

/**
 * Determine whether a given rook move is valid (or not).
 */
pub fn can_rook_move(board: Board, from: Square, to: Square) -> bool {
    clear_straight_inner(board,from,to)
}

/**
 * Determine whether a given queen move is valid (or not).
 */
pub fn can_queen_move(board: Board, from: Square, to: Square) -> bool {
    clear_diagonal_inner(board,from,to) || clear_straight_inner(board,from,to)
}

/**
 * Determine whether a given king move is valid (or not).
 */
pub fn can_king_move(_board: Board, from: Square, to: Square) -> bool {
    // Determine absolute difference in column
    let dcol = abs_diff_column(from,to);
    // Determine absolute different in row
    let drow = abs_diff_row(from,to);
    //
    (dcol == 1 || drow == 1) && dcol <= 1 && drow <= 1
}

/**
 * Check whether a given diagonal in the board consists of internal
 * blanks.  That is, all positions are blank *except* the start and
 * end square.  Furthermore, if the path between the two points is not
 * a diagonal then return false.
 */
fn clear_diagonal_inner(board: Board, from: Square, to: Square) -> bool {
    let diff_col = abs_diff_column(from,to);
    let diff_row = abs_diff_row(from,to);
    // Sanity check is diagonal
    if diff_col != diff_row || diff_col == 0 {
	false
    } else {
	// Check inner all blanks
	clear_inner(board,from,to)
    }
}

/**
 * Check whether a given straight (i.e. row or column) in the board
 * consists of internal blanks.  That is, all positions are blank
 * *except* the start and end square.  Furthermore, if the path
 * between the two points is not straight then return false.
 */
fn clear_straight_inner(board: Board, from: Square, to: Square) -> bool {
    let diff_col = abs_diff_column(from,to);
    let diff_row = abs_diff_row(from,to);
    // Sanity check is straight
    if (diff_col != 0 && diff_row != 0) || diff_col == diff_row {
	false
    } else {
	// Check inner all blanks	
	clear_inner(board,from,to)
    }    
}

/**
 * Check whether a given straight or diagonal in the board consists of
 * internal blanks.  That is, all positions are blank *except* the
 * start and end square.  Note that if the path between the two points
 * is not a diagonal or straight, then this may loop indefinitely.
 */
fn clear_inner(board: Board, from: Square, to: Square) -> bool {
    // Extract markers
    let mut col : i8 = from.column() as i8;
    let mut row : i8 = from.row() as i8;
    let start_col : i8 = from.column() as i8;
    let start_row : i8 = from.row() as i8;
    let end_col : i8 = to.column() as i8;
    let end_row : i8 = to.row() as i8;
    let col_dir : i8 = cmp(col,end_col);
    let row_dir : i8  = cmp(row,end_row);
    //
    while row != end_row || col != end_col {
	if (row != start_row || col != start_col) && (row != end_row || col != end_col)
	    && board.get(Square::new(col as u8,row as u8)) != BLANK {
		return false;
	    }
	col += col_dir;
	row += row_dir;
    }
    // Success!
    true
}

/**
 * Determine absolute different in column between two squares
 */
fn abs_diff_column(from: Square, to: Square) -> u8 {
    cmp::max(from.column(),to.column()) - cmp::min(from.column(),to.column())
}

/**
 * Determine absolute different in row between two squares
 */
fn abs_diff_row(from: Square, to: Square) -> u8 {
    cmp::max(from.row(),to.row()) - cmp::min(from.row(),to.row())
}

/**
 * Compare two items and return their relative sign.
 */
fn cmp(x: i8, y: i8) -> i8 {
    if x == y {
    	0
    } else if x < y {
    	1
    } else {
    	-1
    }
}

/**
 * provide textual representation of pieces, where white's pieces are
 * uppercase and black's are lowercase.
 */
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f,"{}",self.kind.to_string(self.player))
    }
}

// ======================================================
// Piece Constants
// ======================================================

pub const BLANK: Piece = Piece {
    player: Player::White,
    kind: Kind::Blank,
};

pub const WHITE_PAWN: Piece = Piece {
    player: Player::White,
    kind: Kind::Pawn,
};
pub const WHITE_KNIGHT: Piece = Piece {
    player: Player::White,
    kind: Kind::Knight,
};
pub const WHITE_BISHOP: Piece = Piece {
    player: Player::White,
    kind: Kind::Bishop,
};
pub const WHITE_ROOK: Piece = Piece {
    player: Player::White,
    kind: Kind::Rook,
};
pub const WHITE_QUEEN: Piece = Piece {
    player: Player::White,
    kind: Kind::Queen,
};
pub const WHITE_KING: Piece = Piece {
    player: Player::White,
    kind: Kind::King,
};
// Black's pieces
pub const BLACK_PAWN: Piece = Piece {
    player: Player::Black,
    kind: Kind::Pawn,
};
pub const BLACK_KNIGHT: Piece = Piece {
    player: Player::Black,
    kind: Kind::Knight,
};
pub const BLACK_BISHOP: Piece = Piece {
    player: Player::Black,
    kind: Kind::Bishop,
};
pub const BLACK_ROOK: Piece = Piece {
    player: Player::Black,
    kind: Kind::Rook,
};
pub const BLACK_QUEEN: Piece = Piece {
    player: Player::Black,
    kind: Kind::Queen,
};
pub const BLACK_KING: Piece = Piece {
    player: Player::Black,
    kind: Kind::King,
};

// ======================================================
// Tests
// ======================================================

#[test]
fn test_01() {
    assert_eq!(from_str("",Player::White).unwrap(), WHITE_PAWN);
}

#[test]
fn test_02() {
    assert_eq!(from_str("N",Player::White).unwrap(), WHITE_KNIGHT);
}

#[test]
fn test_03() {
    assert_eq!(from_str("B",Player::White).unwrap(), WHITE_BISHOP);
}

#[test]
fn test_04() {
    assert_eq!(from_str("R",Player::White).unwrap(), WHITE_ROOK);
}

#[test]
fn test_05() {
    assert_eq!(from_str("Q",Player::White).unwrap(), WHITE_QUEEN);
}

#[test]
fn test_06() {
    assert_eq!(from_str("K",Player::White).unwrap(), WHITE_KING);
}

#[test]
fn test_07() {
    assert_eq!(WHITE_PAWN.flip(), BLACK_PAWN);
}

#[test]
fn test_08() {
    assert_eq!(WHITE_KNIGHT.flip(), BLACK_KNIGHT);
}

#[test]
fn test_09() {
    assert_eq!(WHITE_BISHOP.flip(), BLACK_BISHOP);
}

#[test]
fn test_10() {
    assert_eq!(WHITE_ROOK.flip(), BLACK_ROOK);
}

#[test]
fn test_11() {
    assert_eq!(WHITE_QUEEN.flip(), BLACK_QUEEN);
}

#[test]
fn test_12() {
    assert_eq!(WHITE_KING.flip(), BLACK_KING);
}

#[test]
fn test_13() {
    assert!(from_str("e",Player::White).is_err());
}

#[test]
fn test_14() {
    assert!(from_str("p",Player::White).is_err());
}
