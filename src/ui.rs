use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};

/**
 * Determines dimensions of a square on the board.
 */
const SQUARE : f64 = 50.0;
/**
 * Detemines background color for white player.
 */
const BG_COLOUR_WHITE : &str = "#EEEEEE";
/**
 * Detemines background color for black player.
 */
const BG_COLOUR_BLACK : &str = "#000099";

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d) {
    let mut colour = true;
    // Iterate columns
    for i in 0 .. 8 {
	// Calculate X position
	let x = (i as f64) * SQUARE;
	// Iterate rows
	for j in 0 .. 8 {
	    // Calculate Y position
	    let y = (j as f64) * SQUARE;
	    // Update style
	    if colour {
		ctx.set_fill_style(&JsValue::from_str(BG_COLOUR_WHITE));
	    } else {
		ctx.set_fill_style(&JsValue::from_str(BG_COLOUR_BLACK));
	    }
	    // Draw square
	    ctx.fill_rect(x,y,SQUARE,SQUARE);
	    // Update colouror
	    colour = !colour;
	}
	// Update colouror
	colour = !colour;	
    }
}
