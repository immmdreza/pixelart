use crate::pixels::{
    color::IntoPixelColor,
    maybe::MaybePixel,
    position::{PixelStrictPositionInterface, StrictPositions},
};

use super::{PixelCanvas, SharedMutPixelCanvasExt};

/// A template vertical line with const `H` height.
pub fn vertical_line<const H: usize>(color: impl IntoPixelColor) -> PixelCanvas<H, 1, MaybePixel> {
    PixelCanvas::<H, 1, MaybePixel>::new(Some(color.into_pixel_color()))
}

/// A template horizontal line with const `W` width.
pub fn horizontal_line<const W: usize>(
    color: impl IntoPixelColor,
) -> PixelCanvas<1, W, MaybePixel> {
    PixelCanvas::<1, W, MaybePixel>::new(Some(color.into_pixel_color()))
}

/// Bordered `H`  * `W` square.
pub fn square<const H: usize, const W: usize>(
    color: impl IntoPixelColor + Clone,
) -> PixelCanvas<H, W, MaybePixel> {
    let mut table = PixelCanvas::<H, W, MaybePixel>::default();

    table.draw(StrictPositions::TopLeft, vertical_line::<H>(color.clone()));
    table.draw(StrictPositions::TopRight, vertical_line::<H>(color.clone()));

    table.draw(
        StrictPositions::TopLeft.bounding_right(1),
        horizontal_line::<W>(color.clone()),
    );
    table.draw(
        StrictPositions::BottomLeft.bounding_right(1),
        horizontal_line::<W>(color),
    );

    table
}

#[cfg(test)]
mod tests {
    use crate::pixels::{
        canvas::{PixelCanvasExt, PixelCanvasMutExt},
        color::{PixelColor, PixelColorExt},
    };

    use super::*;

    #[test]
    fn test_name() {
        let mut canvas = PixelCanvas::<5>::new(PixelColor::default());
        canvas.draw_exact_abs(square(PixelColor::BLACK));

        canvas.fill_inside(PixelColor::GREEN, StrictPositions::Center);

        let image = canvas.default_image_builder().with_scale(5);
        image.save("arts/template_0.png").unwrap();
    }
}
