use crate::pixels::{
    position::{
        IntoPixelStrictPosition, PixelPositionInterface, PixelStrictPositionInterface,
        StrictPositions,
    },
    PixelInterface, PixelMutInterface,
};

use super::{table::PixelTable, PixelCanvas, PixelCanvasInterface, PixelCanvasMutInterface};

/// Something that can later be drawn on a [`PixelCanvas`].
pub trait Drawable<const H: usize, const W: usize, MP>
where
    MP: PixelInterface,
    MP::ColorType: Clone,
{
    /// Draws the drawable on the canvas.
    ///
    /// The `H` and `W` on canvas and drawable dose'nt have to be the same though they can.
    fn draw_on<const HC: usize, const WC: usize, P, C>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<HC, WC, P>,
        P::ColorType: From<MP::ColorType>;

    /// As same as [`Drawable::draw_on`] but the `H` and `W` on canvas and drawable are same
    fn draw_on_exact<P, C>(&self, start_pos: impl IntoPixelStrictPosition<H, W>, canvas: &mut C)
    where
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<H, W, P>,
        P::ColorType: From<MP::ColorType>,
    {
        self.draw_on::<H, W, P, C>(start_pos, canvas)
    }

    /// As same as [`Drawable::draw_on_exact`] but the start point is TopLeft (0, 0).
    fn draw_on_exact_abs<P, C>(&self, canvas: &mut C)
    where
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<H, W, P>,
        P::ColorType: From<MP::ColorType>,
    {
        self.draw_on_exact::<P, C>(StrictPositions::TopLeft, canvas)
    }
}

pub fn draw_canvas_on<const H: usize, const W: usize, const HC: usize, const WC: usize, P, C, MP>(
    me: &PixelTable<H, W, MP>,
    start_pos: impl IntoPixelStrictPosition<HC, WC>,
    canvas: &mut C,
) where
    MP: PixelInterface,
    P: PixelMutInterface,
    C: PixelCanvasMutInterface<HC, WC, P>,
    MP::ColorType: Clone,
    P::ColorType: From<MP::ColorType>,
{
    let start_pos = start_pos.into_pixel_strict_position();
    for pixel in me.iter_pixels().filter(|f| f.has_color()) {
        if let Ok(Ok(pos_on_canvas)) = start_pos
            .checked_down(pixel.position().row())
            .map(|res| res.checked_right(pixel.position().column()))
        {
            canvas.table_mut()[pos_on_canvas].update_color(pixel.color().clone());
        }
    }
}

impl<const H: usize, const W: usize, MP: PixelInterface> Drawable<H, W, MP> for PixelTable<H, W, MP>
where
    MP::ColorType: Clone,
{
    fn draw_on<const HC: usize, const WC: usize, P, C>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<HC, WC, P>,
        P::ColorType: From<MP::ColorType>,
    {
        draw_canvas_on(self, start_pos, canvas)
    }
}

impl<const H: usize, const W: usize, MP: PixelInterface> Drawable<H, W, MP>
    for PixelCanvas<H, W, MP>
where
    MP::ColorType: Clone,
{
    fn draw_on<const HC: usize, const WC: usize, P, C>(
        &self,
        start_pos: impl IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: PixelMutInterface,
        C: PixelCanvasMutInterface<HC, WC, P>,
        P::ColorType: From<MP::ColorType>,
    {
        self.table().draw_on(start_pos, canvas);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        pixels::{
            canvas::{SharedMutPixelCanvasExt, SharedPixelCanvasExt},
            color::{PixelColor, PixelColorExt},
            position::{PixelStrictPosition, StrictPositions},
            PixelIterExt, PixelIterMutExt,
        },
        prelude::MaybePixel,
    };

    use super::*;

    #[test]
    fn test_drawing_maybe_on_real() {
        let mut canvas = PixelCanvas::<5>::new(PixelColor::default());
        let mut my_5x5_diagonal_line_template = PixelCanvas::<5, 5, MaybePixel>::new(None);

        my_5x5_diagonal_line_template.draw_on(StrictPositions::TopLeft, &mut canvas);
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

        my_5x5_diagonal_line_template.draw_on(StrictPositions::TopLeft, &mut canvas);

        assert!(canvas
            .iter_pixels()
            .filter_position(|p| p.column() != p.row())
            .all(|pix| pix.color() == &PixelColor::WHITE));
        assert!(canvas
            .iter_pixels()
            .filter_position(|p| p.column() == p.row())
            .all(|pix| pix.color() == &PixelColor::BLACK));

        canvas.clear();
        my_5x5_diagonal_line_template.draw_on(PixelStrictPosition::new(0, 2).unwrap(), &mut canvas);

        assert_eq!(
            canvas.iter_pixels().filter_color(PixelColor::BLACK).count(),
            3
        );

        let image = canvas.default_image_builder().with_scale(5);
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
            .draw_on(StrictPositions::TopLeft, &mut my_5x5_diagonal_line_template);

        let mut canvas = PixelCanvas::<5>::new(PixelColor::default());
        my_5x5_diagonal_line_template.draw_on(StrictPositions::TopLeft, &mut canvas);

        assert_eq!(
            canvas.iter_pixels().filter_color(PixelColor::RED).count(),
            4
        );
        assert_eq!(
            canvas.iter_pixels().filter_color(PixelColor::BLUE).count(),
            5
        );

        let image = canvas.default_image_builder().with_scale(5);
        image.save("arts/drawing_1.png").unwrap();
    }
}
