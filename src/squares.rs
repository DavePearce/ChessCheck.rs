use std::fmt;
use std::str;

/**
 * Represents a give position on the board, such as "a1" or "h5", etc.
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    col: u8, // where 0 <= col <= 7    
    row: u8  // where 0 <= row <= 7
}

impl Square {
    /**
     * Calculate offset into squares array that this square represents.
     */
    pub fn to_offset(&self) -> usize {
	return ((self.row*8) + self.col) as usize;
    }
}
/**
 * Convert a string into a square.
 */
pub fn from_str(s: &str) -> Result<Square, ()> {
    if s.len() < 2 {
	Err(())
    } else {
	// Parse column
	let c = parse_col(&s[0..1]).ok_or(())?;
	// Parse row
	let r = parse_row(&s[1..2]).ok_or(())?;
	// Done
	Ok(Square { row: r, col: c })
    }
}

/**
 * Convert column identifier into integer
 */
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

/**
 * Convert row into integer
 */
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

// ======================================================
// Tests
// ======================================================

#[test]
fn test_01() {
    assert_eq!(from_str("a1").unwrap(), Square{col:0,row:0});
}

#[test]
fn test_02() {
    assert_eq!(from_str("b3").unwrap(), Square{col:1,row:2});
}

#[test]
fn test_03() {
    assert_eq!(from_str("c6").unwrap(), Square{col:2,row:5});
}

#[test]
fn test_04() {
    assert_eq!(from_str("d8").unwrap(), Square{col:3,row:7});
}

#[test]
fn test_05() {
    assert_eq!(from_str("e2").unwrap(), Square{col:4,row:1});
}

#[test]
fn test_06() {
    assert!(from_str("xx").is_err());
}

#[test]
fn test_07() {
    assert!(from_str("x").is_err());
}


#[test]
fn test_08() {
    assert!(from_str("").is_err());
}

#[test]
fn test_09() {
    assert!(from_str("a").is_err());
}

#[test]
fn test_10() {
    assert!(from_str("aa").is_err());
}

#[test]
fn test_11() {
    assert!(from_str("a9").is_err());
}

#[test]
fn test_12() {
    assert!(from_str("i1").is_err());
}
