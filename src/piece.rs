use std::fmt;
use std::str;

#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct Piece {
    pub white: bool,
    pub kind: Kind,
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
