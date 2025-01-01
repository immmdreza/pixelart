use super::Template;

use crate::prelude::*;

pub struct HalfHeart;

impl Template<6, 4> for HalfHeart {
    fn define<
        C: crate::pixels::canvas::PixelCanvasMutInterface<6, 4, crate::prelude::MaybePixel>,
    >(
        &self,
        canvas: &mut C,
    ) {
        canvas
            .attach_new_pen(BLACK, TOP_LEFT)
            .right(1)
            .start()
            .branch(|right| right.right(1).down_right(1))
            .branch(|down_left| down_left.down_left(1).down(1).down_right(3));

        canvas.fill_inside(PixelColor::new(196, 0, 0), CENTER);
    }
}

pub struct Heart;

impl Template<6, 7> for Heart {
    fn define<C: crate::pixels::canvas::PixelCanvasMutInterface<6, 7, MaybePixel>>(
        &self,
        canvas: &mut C,
    ) {
        canvas.draw(TOP_LEFT, HalfHeart);
        canvas.draw(TOP_CENTER, HalfHeart.create().flipped_x());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        Heart
            .create()
            .default_image_builder()
            .save("arts/heart.png")
            .unwrap();
    }

    #[test]
    fn test_duplicated() {
        let mut canvas = MaybePixelCanvas::<2>::default();

        canvas.get_pixel_mut(TOP_LEFT).update_color(RED);
        canvas.get_pixel_mut(TOP_LEFT).update_color(RED);

        canvas
            .default_image_builder()
            .save("arts/test/duplicated.png")
            .unwrap();
    }
}
