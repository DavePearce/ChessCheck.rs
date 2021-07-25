use std::fmt;
use std::str;

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

/**
 * Define a given piece on the board
 */
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    pub white: bool,
    pub kind: Kind,
}

/**
 * Set the side for a given piece (i.e. white or black).
 */
pub fn to_side(b: bool, p: Piece) -> Piece {
    return Piece{white: b, kind: p.kind};
}

/**
 * Convert a string representing a piece into an actual
 * Piece (or an error if invalid string).
 */
pub fn from_str(s: &str) -> Result<Piece, ()> {
    match s {
        "" => Ok(WHITE_PAWN),
        "N" => Ok(WHITE_KNIGHT),
        "B" => Ok(WHITE_BISHOP),
        "R" => Ok(WHITE_ROOK),
        "Q" => Ok(WHITE_QUEEN),
        "K" => Ok(WHITE_KING),
        _ => Err(()),
    }
}

/**
 * Test whether the first character of this string represents a kind
 * of pice.
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
        match self.kind {
            Kind::Blank => write!(f, " "),
            Kind::Pawn => {
                if self.white {
                    write!(f, "P")
                } else {
                    write!(f, "p")
                }
            }
            Kind::Knight => {
                if self.white {
                    write!(f, "N")
                } else {
                    write!(f, "n")
                }
            }
            Kind::Bishop => {
                if self.white {
                    write!(f, "B")
                } else {
                    write!(f, "b")
                }
            }
            Kind::Rook => {
                if self.white {
                    write!(f, "R")
                } else {
                    write!(f, "r")
                }
            }
            Kind::Queen => {
                if self.white {
                    write!(f, "Q")
                } else {
                    write!(f, "q")
                }
            }
            Kind::King => {
                if self.white {
                    write!(f, "K")
                } else {
                    write!(f, "k")
                }
            }
        }
    }
}

// ======================================================
// Piece Constants
// ======================================================

pub const BLANK: Piece = Piece {
    white: true,
    kind: Kind::Blank,
};

pub const WHITE_PAWN: Piece = Piece {
    white: true,
    kind: Kind::Pawn,
};
pub const WHITE_KNIGHT: Piece = Piece {
    white: true,
    kind: Kind::Knight,
};
pub const WHITE_BISHOP: Piece = Piece {
    white: true,
    kind: Kind::Bishop,
};
pub const WHITE_ROOK: Piece = Piece {
    white: true,
    kind: Kind::Rook,
};
pub const WHITE_QUEEN: Piece = Piece {
    white: true,
    kind: Kind::Queen,
};
pub const WHITE_KING: Piece = Piece {
    white: true,
    kind: Kind::King,
};
// Black's pieces
pub const BLACK_PAWN: Piece = Piece {
    white: false,
    kind: Kind::Pawn,
};
pub const BLACK_KNIGHT: Piece = Piece {
    white: false,
    kind: Kind::Knight,
};
pub const BLACK_BISHOP: Piece = Piece {
    white: false,
    kind: Kind::Bishop,
};
pub const BLACK_ROOK: Piece = Piece {
    white: false,
    kind: Kind::Rook,
};
pub const BLACK_QUEEN: Piece = Piece {
    white: false,
    kind: Kind::Queen,
};
pub const BLACK_KING: Piece = Piece {
    white: false,
    kind: Kind::King,
};

// ======================================================
// Tests
// ======================================================

#[test]
fn test_01() {
    assert_eq!(from_str("").unwrap(), WHITE_PAWN);
}

#[test]
fn test_02() {
    assert_eq!(from_str("N").unwrap(), WHITE_KNIGHT);
}

#[test]
fn test_03() {
    assert_eq!(from_str("B").unwrap(), WHITE_BISHOP);
}

#[test]
fn test_04() {
    assert_eq!(from_str("R").unwrap(), WHITE_ROOK);
}

#[test]
fn test_05() {
    assert_eq!(from_str("Q").unwrap(), WHITE_QUEEN);
}

#[test]
fn test_06() {
    assert_eq!(from_str("K").unwrap(), WHITE_KING);
}

#[test]
fn test_07() {
    assert_eq!(to_side(false,WHITE_PAWN), BLACK_PAWN);
}

#[test]
fn test_08() {
    assert_eq!(to_side(false,WHITE_KNIGHT), BLACK_KNIGHT);
}

#[test]
fn test_09() {
    assert_eq!(to_side(false,WHITE_BISHOP), BLACK_BISHOP);
}

#[test]
fn test_10() {
    assert_eq!(to_side(false,WHITE_ROOK), BLACK_ROOK);
}

#[test]
fn test_11() {
    assert_eq!(to_side(false,WHITE_QUEEN), BLACK_QUEEN);
}

#[test]
fn test_12() {
    assert_eq!(to_side(false,WHITE_KING), BLACK_KING);
}

#[test]
fn test_13() {
    assert!(from_str("e").is_err());
}

#[test]
fn test_14() {
    assert!(from_str("p").is_err());
}
