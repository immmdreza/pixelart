use crate::{
    pixels::{
        maybe::MaybePixel,
        position::{IntoPixelStrictPosition, PixelStrictPositionInterface, StrictPositions},
        PixelInterface,
    },
    prelude::{PixelColor, TOP_RIGHT},
};

use super::{drawable::Drawable, PixelCanvas, PixelCanvasMutInterface, SharedMutPixelCanvasExt};

pub mod alien_monster;
pub mod heart;

pub trait Template<const H: usize, const W: usize> {
    fn define<C: PixelCanvasMutInterface<H, W, MaybePixel>>(&self, canvas: &mut C);

    fn create(&self) -> PixelCanvas<H, W, MaybePixel> {
        let mut canvas = PixelCanvas::<H, W, MaybePixel>::default();
        self.define(&mut canvas);
        canvas
    }

    fn apply_existing<C: PixelCanvasMutInterface<H, W, MaybePixel>>(&self, canvas: &mut C) {
        self.define(canvas)
    }
}

impl<const H: usize, const W: usize, T: Template<H, W>> Drawable<H, W, MaybePixel> for T {
    fn draw_on<const HC: usize, const WC: usize, P, C, E>(
        &self,
        start_pos: impl crate::pixels::position::IntoPixelStrictPosition<HC, WC>,
        canvas: &mut C,
    ) where
        P: crate::pixels::PixelMutInterface + PartialEq + Clone + Default,
        C: PixelCanvasMutInterface<HC, WC, P>,
        P::ColorType: TryFrom<<MaybePixel as PixelInterface>::ColorType, Error = E>,
    {
        let template = self.create();
        canvas.draw(start_pos, template);
    }
}

/// A template vertical line with const `H` height.
pub fn vertical_line<const H: usize>(
    color: impl Into<PixelColor>,
) -> PixelCanvas<H, 1, MaybePixel> {
    PixelCanvas::<H, 1, MaybePixel>::from_fill_color(Some(color.into()))
}

/// A template horizontal line with const `W` width.
pub fn horizontal_line<const W: usize>(
    color: impl Into<PixelColor>,
) -> PixelCanvas<1, W, MaybePixel> {
    PixelCanvas::<1, W, MaybePixel>::from_fill_color(Some(color.into()))
}

/// Bordered `H`  * `W` square.
pub fn rectangle<const H: usize, const W: usize>(
    color: impl Into<PixelColor> + Clone,
) -> PixelCanvas<H, W, MaybePixel> {
    let mut table = PixelCanvas::<H, W, MaybePixel>::default();

    table.draw(StrictPositions::TopLeft, vertical_line::<H>(color.clone()));

    let strict_pos: crate::pixels::position::PixelStrictPosition<H, W> =
        TOP_RIGHT.into_pixel_strict_position();
    println!("{:?}", strict_pos);
    table.draw(strict_pos, vertical_line::<H>(color.clone()));

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

pub fn square<const H: usize>(
    color: impl Into<PixelColor> + Clone,
) -> PixelCanvas<H, H, MaybePixel> {
    rectangle::<H, H>(color)
}

#[cfg(test)]
mod tests {
    use crate::pixels::{
        canvas::SharedPixelCanvasExt,
        color::{PixelColor, PixelColorExt},
    };

    use super::*;

    #[test]
    fn test_name() {
        let mut canvas = PixelCanvas::<5>::default();
        canvas.draw_exact_abs(rectangle(PixelColor::BLACK));

        canvas.fill_inside(PixelColor::GREEN, StrictPositions::Center);

        let image = canvas.default_image_builder().with_scale(5);
        image.save("arts/template_0.png").unwrap();
    }
}
