use pixelart::{
    animation::{create_simple_animation, PixelAnimationBuilder, Repeat},
    pixels::canvas::{
        templates::alien_monster::AlienMonster, SharedMutPixelCanvasExt, SharedPixelCanvasExt,
    },
    prelude::*,
    viewer::view,
};

pub mod first_stone;
pub mod pen;
pub mod template;

fn main() {
    let mut canvas = PixelCanvas::<17, 20, MaybePixel>::default();
    canvas.draw_exact_abs(AlienMonster);

    view([
        vec![canvas.default_image_builder().with_scale(2).get_image()],
        create_simple_animation::<5, 5, 1, 1>(
            TOP_LEFT,
            PixelAnimationBuilder::new_empty(Repeat::Infinite, 5),
            Repeat::Infinite,
            |ctx| {
                ctx.update_body_color(YELLOW);
                ctx.update_part_color(BLUE);
            },
            |i, ctx| {
                if let Some(next) = ctx.part.position().next() {
                    ctx.part
                        .replace_to(next, PixelColor::from_blue(255 - (i as u8 * 10) % 250));
                    ctx.update_part_color(PixelColor::from_blue(255 - (i as u8 * 10) % 250));
                    true
                } else {
                    false
                }
            },
            |_, _ctx| {},
        )
        .take_images(),
    ])
    .unwrap()
}
