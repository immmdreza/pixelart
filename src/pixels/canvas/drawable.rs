use crate::pixels::{
    color::PixelColor,
    maybe::{MaybePixel, PixelCanvasExt},
    position::{IntoPixelStrictPosition, PixelPositionInterface, PixelStrictPositionInterface},
    PixelInterface, PixelMutInterface,
};

use super::{PixelCanvas, PixelCanvasInterface};

/// Something that can later be drawn on a [`PixelCanvas`].
pub trait Drawable<const H: usize, const W: usize> {
    /// Draws the drawable on the canvas.
    ///
    /// The `H` and `W` on canvas and drawable dose'nt have to be the same though they can.
    fn draw_on_canvas<const HC: usize, const WC: usize, P, C>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface,
        C: PixelCanvasInterface<HC, WC, P>,
        <P as PixelInterface>::ColorType: From<PixelColor>;
}

impl<const H: usize, const W: usize> Drawable<H, W> for PixelCanvas<H, W, MaybePixel> {
    fn draw_on_canvas<const HC: usize, const WC: usize, P, C>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface,
        C: PixelCanvasInterface<HC, WC, P>,
        <P as PixelInterface>::ColorType: From<PixelColor>,
    {
        let start_pos = start_pos.into_pixel_strict_position();
        for pixel in self.iter_existing_pixels() {
            if let Ok(Ok(pos_on_canvas)) = start_pos
                .checked_down(pixel.position().row())
                .map(|res| res.checked_right(pixel.position().column()))
            {
                canvas.table_mut()[pos_on_canvas].update_color(pixel.color().unwrap());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pixels::{
        canvas::{PixelCanvasExt, PixelCanvasMutExt},
        color::{PixelColor, PixelColorExt},
        position::{PixelStrictPosition, StrictPositions},
        PixelIterExt, PixelIterMutExt,
    };

    use super::*;

    #[test]
    fn test_drawing_maybe_on_real() {
        let mut canvas = PixelCanvas::<5>::new(PixelColor::default());
        let mut my_5x5_diagonal_line_template = PixelCanvas::<5, 5, MaybePixel>::new(None);

        my_5x5_diagonal_line_template.draw_on_canvas(StrictPositions::TopLeft, &mut canvas);
        assert!(
            canvas
                .iter_pixels()
                .all(|pix| pix.color() == &PixelColor::WHITE),
            "All pixels on the canvas must be at default value because the template was empty."
        );

        my_5x5_diagonal_line_template
            .iter_pixels_mut()
            .filter_position(|p| p.column() == p.row())
            .update_colors(PixelColor::BLACK);

        my_5x5_diagonal_line_template.draw_on_canvas(StrictPositions::TopLeft, &mut canvas);

        assert!(canvas
            .iter_pixels()
            .filter_position(|p| p.column() != p.row())
            .all(|pix| pix.color() == &PixelColor::WHITE));
        assert!(canvas
            .iter_pixels()
            .filter_position(|p| p.column() == p.row())
            .all(|pix| pix.color() == &PixelColor::BLACK));

        canvas.clear();
        my_5x5_diagonal_line_template
            .draw_on_canvas(PixelStrictPosition::new(0, 2).unwrap(), &mut canvas);

        assert_eq!(
            canvas.iter_pixels().filter_color(PixelColor::BLACK).count(),
            3
        );

        let image = canvas.image_builder_default().with_scale(5);
        image.save("arts/drawing_0.png").unwrap();
    }

    #[test]
    fn test_drawing_on_drawing() {
        let mut my_5x5_diagonal_line_template = PixelCanvas::<5, 5, MaybePixel>::new(None);
        my_5x5_diagonal_line_template
            .iter_pixels_mut()
            .filter_position(|p| p.column() == p.row())
            .update_colors(PixelColor::RED);

        let mut my_other_5x5_diagonal_line_template = PixelCanvas::<5, 5, MaybePixel>::new(None);
        my_other_5x5_diagonal_line_template
            .iter_pixels_mut()
            .filter_position(|p| p.column() + p.row() == 4)
            .update_colors(PixelColor::BLUE);

        my_other_5x5_diagonal_line_template
            .draw_on_canvas(StrictPositions::TopLeft, &mut my_5x5_diagonal_line_template);

        let mut canvas = PixelCanvas::<5>::new(PixelColor::default());
        my_5x5_diagonal_line_template.draw_on_canvas(StrictPositions::TopLeft, &mut canvas);

        assert_eq!(
            canvas.iter_pixels().filter_color(PixelColor::RED).count(),
            4
        );
        assert_eq!(
            canvas.iter_pixels().filter_color(PixelColor::BLUE).count(),
            5
        );

        let image = canvas.image_builder_default().with_scale(5);
        image.save("arts/drawing_1.png").unwrap();
    }
}
