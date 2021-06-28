use std::fmt;
use std::str;

/**
 * Represents a give position on the board, such as "a1" or "h5", etc.
 */
pub struct Square {
    row: u8, // where 0 <= row <= 7
    col: u8, // where 0 <= col <= 7
}

/**
 * Convert a string into a square.
 */
pub fn from_str(s: &str) -> Result<Square, ()> {
    // Parse column
    let c = parse_col(&s[0..1]).ok_or(())?;
    // Parse row
    let r = parse_row(&s[1..2]).ok_or(())?;    
    // Done
    Ok(Square { row: r, col: c })
}

fn parse_col(s: &str) -> Option<u8> {
    match s {
        "a" => Some(0),
        "b" => Some(1),
        "c" => Some(2),
        "d" => Some(3),
        "e" => Some(4),
        "f" => Some(5),
        "g" => Some(6),
        "h" => Some(7),
        _ => None,
    }
}

fn parse_row(s: &str) -> Option<u8> {
    // This could be simpler!
    match s {
        "1" => Some(0),
        "2" => Some(1),
        "3" => Some(2),
        "4" => Some(3),
        "5" => Some(4),
        "6" => Some(5),
        "7" => Some(6),
        "8" => Some(7),
        _ => None,
    }
}


/**
 * Provide textual representation of pieces, where white's pieces are
 * uppercase and black's are lowercase.
 */
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self.col {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            _ => "h"
        };
        write!(f,"{}{}",c,self.row+1)
    }
}
