use crate::pixels::canvas::SharedMutPixelCanvasExt;
use crate::{
    pixels::{color::PixelColorExt, position::PixelStrictPositionInterface, PixelMutInterface},
    prelude::{MaybePixel, PixelColor, StrictPositions},
};

use super::Template;

pub struct HalfAlienMonster;

impl Template<17, 10> for HalfAlienMonster {
    fn define<C: crate::pixels::canvas::PixelCanvasMutInterface<17, 10, MaybePixel>>(
        &self,
        canvas: &mut C,
    ) {
        canvas
            .attach_new_pen(PixelColor::BLACK, StrictPositions::BottomLeft)
            .up(4)
            .start()
            .right(1)
            .down(2)
            .right(5)
            .down(2)
            .left(2)
            .up(4)
            .right(2)
            .branch(|pen| pen.up(2))
            .right(1)
            .up(2)
            .right(1)
            .up(1)
            .right(1)
            .up(5)
            .left(2)
            .branch(|pen| pen.down(4))
            .up(2)
            .left(1)
            .branch(|pen| pen.up(2).left(2).down(4).right(1).down(1).right(1).down(3))
            .left(3)
            .branch(|pen| pen.down(1))
            .up(1)
            .left(2)
            .down(2)
            .left(1)
            .stop()
            .down(4)
            .right(2)
            .start()
            .down(2)
            .right(1)
            .up(2);

        let color = PixelColor::from_red(106).blue(127);

        canvas.fill_inside(color, StrictPositions::LeftCenter);
        canvas.table_mut()[(1, 5)].update_color(color);
        canvas.fill_inside(color, (3, 5));
        canvas.fill_inside(color, (5, 8));
        canvas.table_mut()[(15, 5)].update_color(color);
    }
}

/// An alien monster 👾 show case.
pub struct AlienMonster;

impl Template<17, 20> for AlienMonster {
    fn define<C: crate::pixels::canvas::PixelCanvasMutInterface<17, 20, MaybePixel>>(
        &self,
        canvas: &mut C,
    ) {
        // Create left side by flipping half monster.
        canvas.draw(StrictPositions::TopLeft, HalfAlienMonster.create().flip_y());
        // Right side
        canvas.draw(
            StrictPositions::TopLeft.bounding_right(10),
            HalfAlienMonster,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::pixels::canvas::{MaybePixelCanvas, SharedMutPixelCanvasExt, SharedPixelCanvasExt};

    use super::*;

    #[test]
    fn monster() {
        let mut canvas = MaybePixelCanvas::<17, 20>::default();
        canvas.draw_exact_abs(AlienMonster);

        canvas
            .default_image_builder()
            .with_scale(2)
            .save("arts/alien_monster.png")
            .unwrap();
    }
}
