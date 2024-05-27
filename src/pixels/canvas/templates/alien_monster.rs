use crate::{
    pixels::{position::PixelStrictPositionInterface, PixelMutInterface},
    prelude::{MaybePixel, PixelColor, PixelColorExt, SharedMutPixelCanvasExt, StrictPositions},
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
            .go_up(4)
            .start_drawing()
            .go_right(1)
            .go_down(2)
            .go_right(5)
            .go_down(2)
            .go_left(2)
            .go_up(4)
            .go_right(2)
            .branch(|pen| pen.go_up(2))
            .go_right(1)
            .go_up(2)
            .go_right(1)
            .go_up(1)
            .go_right(1)
            .go_up(5)
            .go_left(2)
            .branch(|pen| pen.go_down(4))
            .go_up(2)
            .go_left(1)
            .branch(|pen| {
                pen.go_up(2)
                    .go_left(2)
                    .go_down(4)
                    .go_right(1)
                    .go_down(1)
                    .go_right(1)
                    .go_down(3)
            })
            .go_left(3)
            .branch(|pen| pen.go_down(1))
            .go_up(1)
            .go_left(2)
            .go_down(2)
            .go_left(1)
            .stop_drawing()
            .go_down(4)
            .go_right(2)
            .start_drawing()
            .go_down(2)
            .go_right(1)
            .go_up(2);

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
    use crate::pixels::canvas::{MaybePixelCanvas, SharedPixelCanvasExt};

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
