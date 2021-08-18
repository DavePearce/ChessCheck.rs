#![warn(missing_docs)]

//! A simple crate for checking games in algebraic notation are
//! correct.

mod piece;
mod board;
mod muve;
mod squares;
mod game;

use std::env;
use std::fs;
use std::str::FromStr;
use game::Game;

fn main() {
    // Extract command-line arguments
    let args: Vec<String> = env::args().collect();
    // Determine game filename
    let filename = &args[1];
    println!("Reading file {}", filename);
    // Read the game file!
    let contents = fs::read_to_string(filename)
        .expect("error reading game file");
    // Parse game string
    let g = Game::from_str(&contents).unwrap();
    // Apply each move to initial board producing a potentially
    // updated board.    
    let brd = g.apply(board::INITIAL).unwrap();
    // Print game
    println!("Game:\n{}\n",g);
    // Print actual board
    println!("Actual:\n{}",brd.to_string());
}
