use pixelart::{
    animation::{simple::create_simple_animation, Repeat},
    prelude::*,
};

pub fn moving_plus() {
    create_simple_animation::<10, 10, 3, 3>(
        TOP_LEFT,
        5,
        Repeat::Infinite,
        Repeat::Infinite,
        |ctx| {
            let canvas = ctx.body_mut();

            for (i, row) in canvas.iter_mut().enumerate() {
                for pixel in row.iter_mut() {
                    pixel.color = PixelColor::from_blue(255 - (i as u8 * 20))
                }
            }

            let part = ctx.part_mut();

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
            if let Some(next) = ctx.part().position().next() {
                ctx.part_mut().crop_to(next);
                true
            } else {
                false
            }
        },
    )
    .view()
    .unwrap();
}
