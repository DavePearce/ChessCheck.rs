use super::moves::*;
use std::fmt;
use std::str;

struct Game {
    // Sequence of moves beggining with White's and alternating between White and Black.
    moves: Vec<Box<SimpleMove>>,
}

/**
 * Trait for reading a game in from a string.
 */
impl str::FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ms = Vec::<Box<SimpleMove>>::new();
        // Read line-by-line
        for l in s.lines() {
            // Split moves
            let v: Vec<&str> = l.split(' ').collect();
            //
            match v.len() {
                1 => {
                    let wm: SimpleMove = SimpleMove::from_str(v[0])?;
                    // Append white's move
                    ms.push(Box::new(wm));
                }
                2 => {
                    let wm: SimpleMove = SimpleMove::from_str(v[0])?;
                    let bm: SimpleMove = SimpleMove::from_str(v[1])?;
                    // Append white's move
                    ms.push(Box::new(wm));
                    // Append black's move
                    ms.push(Box::new(bm));
                }
                _ => return Err(()),
            }
        }
        // Dummy for now
        Ok(Game { moves: ms })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = true;
        for m in &self.moves {
            if w {
                write!(f, "{} ", m)?;
            } else {
                writeln!(f, "{} ", m)?;
            }
            w = !w;
        }
        Ok(())
    }
}
