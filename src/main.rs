#![warn(missing_docs)]

//! A simple crate for checking games in algebraic notation are
//! correct.

mod piece;
mod board;
mod muve;
mod squares;
mod game;

//use self::moves;

fn main() {
    let v : Vec<&str> = "  x y".split_ascii_whitespace().collect();
    //
    for s in v {
	println!("GOT {}",s);
    }
}
