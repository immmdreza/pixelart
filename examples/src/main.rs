use pixelart::{
    animation::{simple::create_simple_animation, Repeat},
    pixels::canvas::{
        templates::{alien_monster::AlienMonster, heart::Heart, Template},
        SharedMutPixelCanvasExt, SharedPixelCanvasExt,
    },
    prelude::*,
    viewer::view,
};

pub mod first_stone;
pub mod partition;
pub mod pen;
pub mod template;

fn main() {
    // partition::moving_plus();
    Heart
        .create()
        .default_image_builder()
        .save("arts/heart.png")
        .unwrap();
}

#[allow(dead_code)]
fn default() {
    let mut canvas = PixelCanvas::<17, 20, MaybePixel>::default();
    canvas.draw_exact_abs(AlienMonster);

    view([
        create_simple_animation::<5, 5, 1, 1>(
            TOP_LEFT,
            5,
            Repeat::Infinite,
            Repeat::Infinite,
            |ctx| {
                ctx.update_body_color(YELLOW);
                ctx.update_part_color(BLUE);
            },
            |i, ctx| {
                if let Some(next) = ctx.part().position().next() {
                    ctx.part_mut().crop_to(next);
                    ctx.update_part_color(PixelColor::from_blue(255 - (i as u8 * 10) % 250));
                    true
                } else {
                    false
                }
            },
        )
        .take_images(),
        vec![canvas.default_image_builder().with_scale(2).get_image()],
    ])
    .unwrap()
}
