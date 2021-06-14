use std::fmt;
use std::fs;

struct Round {
    // White always has a move
    white: String,
    // Black may not have a move
    black: Option<String>
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.black {
            Some(ref s) => {
                write!(f,"{} {}",self.white,s)
            }
            None => {
                write!(f,"{}.",self.white)
            }
        }
    }
}

fn parse_line(line: &str) -> Round {
    return Round{
        white: line.to_string(),
        black: None
    };
}

fn main() {
    let filename = "game.txt";
    //
    let contents = fs::read_to_string(filename)
        .expect("Error!");
    //
    let mut rounds = Vec::new();    
    //
    for l in contents.lines() {
        rounds.push(parse_line(l));
    }
    //
    let mut i : u32 = 0;    
    for r in rounds {
        println!("ROUND {}: {}",i,r);
        i = i + 1;        
    }
}
