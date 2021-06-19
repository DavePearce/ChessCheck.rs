use std::fmt;
use super::moves::*;

struct Game {
    // Sequence of moves beggining with White's and alternating between White and Black.
    moves: Vec<Box<dyn Move>>
}



impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut w = true;
        for m in &self.moves {
            if w {
                write!(f,"{} ",m)?;
            } else {
                writeln!(f,"{} ",m)?;
            }
            w = !w;
        }
        Ok(())
    }
}