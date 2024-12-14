use pixelart::{
    animation::{create_simple_animation, PixelAnimationBuilder, Repeat},
    prelude::*,
};

pub fn moving_plus() {
    create_simple_animation::<10, 10, 3, 3>(
        TOP_LEFT,
        PixelAnimationBuilder::new_empty(Repeat::Infinite, 2),
        Repeat::Infinite,
        |ctx| {
            let canvas = ctx.body_mut();

            for (i, row) in canvas.iter_mut().enumerate() {
                for pixel in row.iter_mut() {
                    pixel.color = PixelColor::from_blue(255 - (i as u8 * 20))
                }
            }

            let part = &mut ctx.part;

            part.clear();

            let pen = PixelPen::new(RED);
            pen.attach(part, LEFT_CENTER)
                .start()
                .right(1)
                .branch(|pen| pen.up(1))
                .branch(|pen| pen.down(1))
                .branch(|pen| pen.right(1));

            part.write_source();
        },
        |_i, ctx| {
            if let Some(next) = ctx.part.position().next() {
                ctx.part.crop_to(next);
                true
            } else {
                false
            }
        },
        |_, _ctx| {},
    )
    .save("../arts/animation_2.gif")
    .unwrap();
}
