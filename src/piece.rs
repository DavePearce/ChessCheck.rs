use std::fmt;
use std::str;
use super::squares::Square;
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
    pub fn to_string(&self,p:Player) -> &str {
	match p {
	    Player::White => self.to_white_string(),
	    Player::Black => self.to_black_string()		
	}
    }

    pub fn to_white_string(&self) -> &str {
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

    pub fn to_black_string(&self) -> &str {
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
     * Set the side for a given piece (i.e. white or black).
     */
    pub fn to_side(&self, p:Player) -> Piece {
	return Piece{player: p, kind: self.kind};
    }

    /**
     * Check whether this piece can move from one position on the
     * board to another.  There are few aspects to this: firstly, it
     * has to be a valid move for the piece (e.g. rooks cannot move
     * along a diagonol); secondly, the move needs to be unobstructed
     * (at least for some pieces).  Note, however, than the final
     * position is not considered here.  That is, we are not concerned
     * whether the final piece is obstructed or not (this is handled
     * elsewhere).    
     */
    pub fn can_move(&self, board: Board, from: Square, to: Square) -> bool {
	match self.kind {
	    Pawn => {
		true
	    }
	    _ => {
		false
	    }
	}
    }
    
    /** 
     * Flip to other player.  For example, if this is white then it
     * becomes black, etc.
     */
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
    match s {
        "N" => true,
        "B" => true,
        "R" => true,
        "Q" => true,
        "K" => true,
        _ => false
    }    
}

/**
 * Provide textual representation of pieces, where white's pieces are
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
