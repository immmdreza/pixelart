use pixelart::{
    pixels::canvas::{
        templates::alien_monster::AlienMonster, SharedMutPixelCanvasExt, SharedPixelCanvasExt,
    },
    prelude::*,
};

pub mod first_stone;
pub mod pen;
pub mod template;

fn main() -> ViewResult {
    let mut canvas = PixelCanvas::<17, 20, MaybePixel>::default();
    canvas.draw_exact_abs(AlienMonster);

    canvas
        .default_image_builder()
        .with_scale(2)
        .view_with_others([PixelCanvas::<5>::default()
            .default_image_builder()
            .with_scale(5)
            .get_image()])
}
