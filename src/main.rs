mod piece;
mod board;
mod moves;
mod squares;
mod game;

use self::moves::Move;

//use self::moves;

fn main() {
    let ms = vec!["a2-c4","Bb2-c4","b2xd2","Bb2xf3","c2xBe4"];
    //
    for s in ms {
	let m = moves::from_str(s).unwrap();
	println!("MOVE: {}",m);	
    }

}
