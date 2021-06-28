mod piece;
mod board;
mod moves;
mod squares;
mod game;

//use self::moves;

fn main() {
    let game : &str = "Bb2-c4";
    let mv = moves::from_str(game).unwrap();
    //
    println!("MOVE: {}",mv);
}
