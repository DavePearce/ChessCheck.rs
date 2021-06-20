mod piece;
mod board;
mod moves;
mod squares;
mod game;

//use self::moves;

fn main() {
    let game : &str = "a1-b1";
    let mv = moves::from_str(game).unwrap();
    //
    println!("MOVE: {}",mv);
}
