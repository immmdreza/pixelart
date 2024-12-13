#![cfg(test)]
#![cfg(debug_assertions)]

use crate::{pixels::canvas::templates::square, prelude::*};

fn create_and_save_canvas_of_size<const H: usize, const W: usize>() {
    let mut px = PixelCanvas::<H, W>::default();

    px.draw(CENTER, square::<1>(BLACK));
    px.default_image_builder().save("arts/growth.png").unwrap();
}

#[test]
fn test_growth() {
    // That's enough ?!
    // A single 110x110 canvas will cause the stack to overflow!
    // currently ~11,881 normal [`Pixel`]s can be handled at most.
    // This can surely be further increased, but is it needed?
    create_and_save_canvas_of_size::<109, 109>();
}
